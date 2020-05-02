use crate::constants;
use crate::{utils, ResultExt};
use hyper::{Body, Request, Response, StatusCode};
use routerify::{Middleware, Router};
use routerify_cors::enable_cors_all;
use routerify_json_response::{json_failed_resp_with_message, json_success_resp};
use serde_json::json;

mod api;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .middleware(enable_cors_all())
        .get("/", home_get)
        .scope("/api", api::router())
        .err_handler(error_handler)
        .build()
        .unwrap()
}

async fn logger_middleware(req: Request<Body>) -> crate::Result<Request<Body>> {
    info!(
        "{} {} {}",
        utils::extract_client_ip_from_req(&req),
        req.method(),
        req.uri()
    );
    Ok(req)
}

async fn home_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    json_success_resp(&json!({
        "name": constants::APP_NAME,
        "version": constants::APP_VERSION,
        "description": constants::APP_DESCRIPTION,
    }))
    .wrap()
}

async fn error_handler(err: routerify::Error) -> Response<Body> {
    error!("{}", err);
    json_failed_resp_with_message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        .expect("Couldn't create a response while handling the server error")
}
