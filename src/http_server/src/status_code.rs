use hyper::{Response, StatusCode};

use crate::http_server::HTTPResponse;
use crate::utils::chunked_body::ChunkedBody;

macro_rules! http_response {
    ( $cmd:expr ) => {
        match $cmd {
            Ok(_)   => ok("ok"),
            Err(e)  => error(&e.to_string())
        }
    };
}
pub(crate) use http_response;

pub fn ok(str: &str) -> HTTPResponse {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(ChunkedBody::from(str))
        .unwrap())
}

pub fn ok_chunked(body: ChunkedBody) -> HTTPResponse {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap())
}

pub fn error(message: &str) -> HTTPResponse {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(ChunkedBody::from(message))
        .unwrap())
}

pub fn invalid(message: &str) -> HTTPResponse {
    Ok(Response::builder()
        .status(StatusCode::UNPROCESSABLE_ENTITY)
        .body(ChunkedBody::from(message))
        .unwrap())
}

pub fn not_found() -> HTTPResponse {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(ChunkedBody::from("not_found"))
        .unwrap())
}