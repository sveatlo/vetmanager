use crate::errors;
use crate::response;
use crate::response::SuccessResponse;
use anyhow::Context;
use axum::Json;
use serde::Serialize;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/new", axum::routing::get(test_new))
        .route("/missing", axum::routing::get(not_found))
        .route("/panic", axum::routing::get(panic))
}

#[derive(Debug, Serialize)]
pub struct RootResponse {
    greeting: String,
}

pub async fn root() -> response::Result<RootResponse> {
    let res = SuccessResponse::ok(RootResponse {
        greeting:
            "Hello and welcome to this amazing API! Please do not continue. Sincerely, sveatlo."
                .to_owned(),
    });
    Ok(res)
}

pub async fn test_new() -> response::Result<String> {
    Ok(SuccessResponse::created("new thing".to_owned()))
}

pub async fn not_found() -> response::Result<()> {
    Err(errors::AppError::NotFound("this resource".to_owned()))
}

pub async fn panic() -> response::Result<()> {
    panic!("aaaaaaaaaaa")
}
