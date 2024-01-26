use std::{fs, path::Path};
use std::fs::{metadata, remove_file};
use futures::{StreamExt, AsyncWriteExt};
use futures_lite::AsyncReadExt;
use glommio::{enclose, io::{BufferedFile, StreamReaderBuilder, StreamWriterBuilder}};
use hyper::{Body, Request, Response};
use tokio::sync::mpsc;

use crate::http_server::{HTTPResponse, Params};
use crate::http_server::status_code::{error, invalid, http_response, ok, ok_chunked};
use crate::io::{file::list_directory};
use crate::io::file::copy_directory;
use crate::utils::chunked_body::{Chunk, ChunkedBody};

pub async fn copy_handler(_req: Request<Body>, params: Params) -> HTTPResponse {
    validate_path(params.get("from"), |from| {
        validate_path(params.get("to"), |to| {
            if to.is_dir() {
                let overwrite = params.get("overwrite").map(|x| x == "true").unwrap_or(false);
                if from.is_dir() {
                    http_response!(copy_directory(from, to))
                } else {
                    if !to.join(from.file_name().unwrap()).exists() || overwrite {
                        http_response!(fs::copy(from, to))
                    } else {
                        return invalid("File at destination already exists.");
                    }
                }
            } else {
                return invalid("Destination is not a directory.");
            }
        })
    })
}

pub async fn delete_handler(_req: Request<Body>, params: Params) -> HTTPResponse {
    validate_path(params.get("path"), |path| {
        if metadata(path).unwrap().is_dir() {
            http_response!(rm_rf::ensure_removed(path))
        } else {
            http_response!(remove_file(path))
        }
    })
}

pub async fn download_handler(_req: Request<Body>, params: Params) -> HTTPResponse {
    validate_path(params.get("path"), |path| {
        let (sender, receiver) = mpsc::channel(128);
        let mut body = ChunkedBody::from(Chunk::Channel(receiver));
        body.set_size(fs::metadata(path).unwrap().len());
        let owned_path = path.to_owned();

        glommio::spawn_local(enclose! {(sender) async move {
            let file = BufferedFile::open(owned_path).await.unwrap();
            let mut file_stream = StreamReaderBuilder::new(file).build();
            let mut buf = vec![0; 4 << 10];

            loop {
                if file_stream.read(&mut buf).await.unwrap() == 0 {
                    break;
                }
                sender.send(Chunk::from(buf.as_slice())).await.unwrap();
            }

            file_stream.close().await.unwrap();
        }}).detach();

        ok_chunked(body)
    })
}

pub async fn list_handler(_req: Request<Body>, params: Params) -> HTTPResponse {
    validate_path(params.get("path"), |path| {
        let recursive = match params.get("recursive") {
            Some(str) => str.eq("true"),
            None => false
        };
        let initial_files = &mut Vec::new();
        let files = list_directory(path, initial_files, recursive);
        ok(&*serde_json::to_string(&files).unwrap())
    })
}

pub async fn make_directory_handler(_req: Request<Body>, params: Params) -> HTTPResponse {
    validate_path(params.get("path"), |path| {
        http_response!(fs::create_dir(path))
    })
}

pub async fn move_handler(_req: Request<Body>, params: Params) -> HTTPResponse {
    validate_path(params.get("from"), |from| {
        validate_path(params.get("to"), |to| {
            if !to.exists() {
                http_response!(fs::rename(from, to))
            } else {
                invalid("File at destination already exists.")
            }
        })
    })
}

pub async fn rename_handler(_req: Request<Body>, params: Params) -> HTTPResponse {
    validate_path(params.get("path"), |path| {
        match params.get("name") {
            Some(name) => {
                if !path.with_file_name(name).exists() {
                    http_response!(fs::rename(path, path.with_file_name(name)))
                } else {
                    invalid("Renamed file already exists")
                }
            }
            None => invalid("Required parameter (name) has not been provided")
        }
    })
}

pub async fn upload_handler(req: Request<Body>, params: Params) -> HTTPResponse {
    match params.get("path") {
        Some(path) => {
            let body = req.into_body();

            glommio::spawn_local(enclose! {(path) async move {
                let file = BufferedFile::create(Path::new(&path)).await.unwrap();
                let mut file_stream = StreamWriterBuilder::new(file).build();

                loop {
                    match &body.next().await {
                        Some(next) => {
                            let bytes = next.as_ref().unwrap();
                            file_stream.write(bytes).await.unwrap();
                        }
                        None => break
                    }
                }
                file_stream.close().await.unwrap();
            }}).detach();

            Ok(Response::new(ChunkedBody::from("response")))
        }
        None => invalid("Okay")
    }
}

fn validate_path(path: Option<&String>, success: impl Fn(&Path) -> HTTPResponse) -> HTTPResponse {
    match path {
        Some(path) => {
            let p = Path::new(path);
            if p.exists() {
                invalid("Path already exists")
            } else {
                success(p)
            }
        }
        None => invalid("Required parameter (path) has not been provided.")
    }
}