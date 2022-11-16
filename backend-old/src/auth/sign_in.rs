use serde::{Deserialize, Serialize};
use slog::info;
use warp::reject;

use crate::{
    response,
    schema::users,
    state::AppState,
    users::{User, UserRepository},
};

#[derive(Debug)]
struct LoginError;
impl reject::Reject for LoginError {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInRequest {
    username: String,
    password: String,
}

pub struct SignInError {}

impl Into<response::ErrorResponse> for SignInError {
    fn into(self) -> response::ErrorResponse {
        response::ErrorResponse::not_found("cannot find user")
    }
}

pub async fn sign_in(req: SignInRequest, state: AppState) -> response::Response<User> {
    let err_msg = "user or password invalid";

    let repo = UserRepository::new(state.db.conn());
    let user = repo
        .find_by_username(req.username)
        .map_err(|_| response::not_found(err_msg))?;

    let valid_password = bcrypt::verify(req.password, &*user.password)
        .map_err(|_| response::unauthorized(err_msg))?;

    if !valid_password {
        return Err(response::unauthorized(err_msg));
    }

    Ok(response::success(user))
}
