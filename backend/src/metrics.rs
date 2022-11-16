use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use metrics_process::Collector as ProcessCollector;
use std::time::Instant;

static EXPONENTIAL_SECONDS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];


pub fn setup() -> (PrometheusHandle, ProcessCollector) {
        let prometheus_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full("http_requests_duration_seconds".to_string()),
                EXPONENTIAL_SECONDS,
            )
            .unwrap()
            .install_recorder()
            .unwrap(); // TODO: replace with error handling

        let process_collector = ProcessCollector::default();
        process_collector.describe();

    (prometheus_handle, process_collector)
}

pub async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let method = req.method().clone();

    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}
