use axum::extract::OriginalUri;
use axum::handler::Handler;
use serde::Serialize;

use crate::errors;
use crate::response;
use crate::response::SuccessResponse;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(root))
        .fallback(not_found.into_service())
}

#[derive(Debug, Serialize)]
pub struct RootResponse {
    greeting: String,
}

async fn root() -> response::Result<RootResponse> {
    let res = SuccessResponse::ok(RootResponse {
        greeting:
            "Hello and welcome to this amazing API! Please do not continue. Sincerely, sveatlo."
                .to_owned(),
    });

    Ok(res)
}

async fn not_found(uri: OriginalUri) -> response::Result<()> {
    let uri: String = uri.0.path().to_owned();

    Err(errors::AppError::NotFound(uri))
}
