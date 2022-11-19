pub mod db;
pub mod errors;
mod http;
mod metrics;
pub mod response;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace::TraceLayer,
};

pub fn create_router() -> Router {
    let (prometheus_handle, process_collector) = metrics::setup();

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(metrics::track_metrics)) // TODO: create layer from manager
        // Compress responses
        .layer(CompressionLayer::new())
        // set CORS
        .layer(CorsLayer::permissive())
        // Mark the `Authorization` request header as sensitive so it doesn't
        // show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            ::http::header::AUTHORIZATION,
        )))
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(
            ::http::header::HeaderName::from_static("x-request-id"),
        ))
        .into_inner();

    Router::new()
        .merge(http::main::routes())
        .route(
            "/metrics",
            axum::routing::get(move || {
                process_collector.collect();
                std::future::ready(prometheus_handle.render())
            }),
        )
        .layer(middleware_stack)
}
