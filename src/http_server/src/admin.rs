use hyper::{Body, Request, Response};

use crate::http_server::{HTTPResponse, Params};
use crate::utils::chunked_body::ChunkedBody;

pub async fn about_handler(_req: Request<Body>, _params: Params) -> HTTPResponse {
    Ok(Response::new(ChunkedBody::from(format!("Hello"))))
}

pub async fn home_handler(_req: Request<Body>, _params: Params) -> HTTPResponse {
    Ok(Response::new(ChunkedBody::from(format!("Hello"))))
}

pub async fn health_handler(_req: Request<Body>, _params: Params) -> HTTPResponse {
    Ok(Response::new(ChunkedBody::from(format!("Hello"))))
}

pub async fn metrics_handler(_req: Request<Body>, _params: Params) -> HTTPResponse {
    Ok(Response::new(ChunkedBody::from(format!("Hello"))))
}