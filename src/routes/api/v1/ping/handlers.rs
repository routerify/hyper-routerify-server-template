use crate::routes::api::v1::ping::controllers;
use hyper::{Body, Request, Response};

pub async fn ping_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    let ping_msg = controllers::gen_ping_message().await?;
    Ok(Response::new(Body::from(ping_msg)))
}

pub async fn pong_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    let pong_msg = controllers::gen_pong_message().await?;
    Ok(Response::new(Body::from(pong_msg)))
}
