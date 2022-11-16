#![allow(dead_code)]

use crate::errors::AppError;
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;

pub type Result<D, E = AppError> = std::result::Result<SuccessResponse<D>, E>;

// SuccessResponse holds data and metadata for successful or partially successful responses
pub struct SuccessResponse<D>(StatusCode, D);

impl<D> SuccessResponse<D>
where
    D: Serialize + Send + Sync,
{
    pub fn ok(data: D) -> Self {
        Self(http::StatusCode::OK, data)
    }

    pub fn created(data: D) -> Self {
        Self(http::StatusCode::CREATED, data)
    }
}

impl<D> IntoResponse for SuccessResponse<D>
where
    D: Serialize + Send + Sync,
{
    fn into_response(self) -> axum::response::Response {
        let (status_code, data) = (self.0, self.1);
        let response_body = Json(data);

        (status_code, response_body).into_response()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        let body = Json(ErrorResponse {
            message: self.to_string(),
        });

        (status_code, body).into_response()
    }
}
