use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AppError {
    #[error("Invalid request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Resouce `{0}` was not found")]
    NotFound(String),

    #[error("{0}")]
    Unknown(#[from] anyhow::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            // 4XX Errors
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,

            AppError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
