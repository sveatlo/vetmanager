use serde::Serialize;
use warp;

#[allow(dead_code)]
pub fn success<T: Serialize + Send>(data: T) -> SuccessResponse<T> {
    SuccessResponse { data }
}

#[allow(dead_code)]
pub fn bad_request(message: &str) -> warp::reject::Rejection {
    ErrorResponse::bad_request(message).into()
}

#[allow(dead_code)]
pub fn unauthorized(message: &str) -> warp::reject::Rejection {
    ErrorResponse::unauthorized(message).into()
}

#[allow(dead_code)]
pub fn forbidden(message: &str) -> warp::reject::Rejection {
    ErrorResponse::forbidden(message).into()
}

#[allow(dead_code)]
pub fn not_found(message: &str) -> warp::reject::Rejection {
    ErrorResponse::not_found(message).into()
}

#[allow(dead_code)]
pub fn conflict(message: &str) -> warp::reject::Rejection {
    ErrorResponse::conflict(message).into()
}

#[allow(dead_code)]
pub fn internal_server_error(message: &str) -> warp::reject::Rejection {
    ErrorResponse::internal_server_error(message).into()
}

pub type Response<S> = Result<SuccessResponse<S>, warp::reject::Rejection>;

#[derive(Serialize)]
pub struct SuccessResponse<T: Serialize + Send> {
    data: T,
}
impl<T: Serialize + Send> warp::Reply for SuccessResponse<T> {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    http_status: warp::http::StatusCode,
    message: String,
}
impl ErrorResponse {
    pub fn bad_request(message: &str) -> ErrorResponse {
        ErrorResponse {
            http_status: warp::http::StatusCode::BAD_REQUEST,
            message: message.to_string(),
        }
    }

    pub fn unauthorized(message: &str) -> ErrorResponse {
        ErrorResponse {
            http_status: warp::http::StatusCode::UNAUTHORIZED,
            message: message.to_string(),
        }
    }

    pub fn forbidden(message: &str) -> ErrorResponse {
        ErrorResponse {
            http_status: warp::http::StatusCode::FORBIDDEN,
            message: message.to_string(),
        }
    }

    pub fn not_found(message: &str) -> ErrorResponse {
        ErrorResponse {
            http_status: warp::http::StatusCode::NOT_FOUND,
            message: message.to_string(),
        }
    }

    pub fn conflict(message: &str) -> ErrorResponse {
        ErrorResponse {
            http_status: warp::http::StatusCode::CONFLICT,
            message: message.to_string(),
        }
    }

    pub fn internal_server_error(message: &str) -> ErrorResponse {
        ErrorResponse {
            http_status: warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
        }
    }
}
impl warp::reject::Reject for ErrorResponse {}
impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        let reply = warp::reply::json(&self).into_response();
        warp::reply::with_status(reply, self.http_status).into_response()
    }
}
