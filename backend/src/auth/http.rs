use crate::AppState;
use warp::Filter;

use super::sign_in;

pub fn get_routes(
    state: AppState,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::reject::Rejection> + Clone {
    let signin = warp::path("sign_in")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state))
        .and_then(sign_in::sign_in);

    warp::path("auth").and(signin)
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
