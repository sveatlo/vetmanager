use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AppError {
    #[error("Invalid request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Resouce `{0}` was not found")]
    NotFound(String),
    #[error("I'm a teapot.")]
    Teapot,

    #[error("{0}")]
    Unknown(#[from] anyhow::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            // 4XX Errors
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Teapot => StatusCode::IM_A_TEAPOT,

            AppError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
