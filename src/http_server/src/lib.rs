use harana_common::hashbrown::HashMap;
use std::convert::Infallible;
use hyper::{Body, Method, Request, Response};
use url::form_urlencoded::parse;
use crate::http_server::status_code::not_found;

use crate::utils::chunked_body::ChunkedBody;

pub mod admin;
pub mod data;
pub mod status_code;
mod chunked_body;
mod main;

pub type HTTPResponse = Result<Response<ChunkedBody>, Infallible>;
pub type Params = HashMap<String, String>;

pub async fn router(req: Request<Body>) -> Result<Response<ChunkedBody>, Infallible> {
    let params = req.uri().query().map(|v| parse(v.as_bytes()).into_owned().collect()).unwrap_or_else(HashMap::new);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => admin::home_handler(req, params).await,
        (&Method::PUT, "/copy") => data::copy_handler(req, params).await,
        (&Method::DELETE, "/delete") => data::delete_handler(req, params).await,
        (&Method::GET, "/download") => data::download_handler(req, params).await,
        (&Method::GET, "/list") => data::list_handler(req, params).await,
        (&Method::PUT, "/mkdir") => data::make_directory_handler(req, params).await,
        (&Method::PUT, "/move") => data::move_handler(req, params).await,
        (&Method::PUT, "/rename") => data::rename_handler(req, params).await,
        (&Method::POST, "/upload") => data::upload_handler(req, params).await,

        (&Method::GET, "/_about") => admin::about_handler(req, params).await,
        (&Method::GET, "/_health") => admin::health_handler(req, params).await,
        (&Method::GET, "/_metrics") => admin::metrics_handler(req, params).await,

        _ => not_found(),
    }
}