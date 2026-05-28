/// This example demonstrates how to use middleware in an Axum
/// application to track the execution time of each request.
/// The `track_request_metrics` middleware captures the start time of the request,
/// allows the request to be processed, and then calculates and logs
/// the duration of the request processing.
/// The `heavy_computation` handler simulates a time-consuming operation
/// by sleeping for a specified duration before responding.

use axum::{
    Router,
    routing::get,
    body::Body,
    http::{Request, Response},
    middleware::{self, Next},
    response::IntoResponse,
};
use std::time::{Instant, Duration};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/compute", get(heavy_computation))
        .layer(middleware::from_fn(track_request_metrics));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Instrumented Web Architecture deployed to http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn heavy_computation() -> impl IntoResponse {
    let timespan = Duration::from_millis(50);
    tokio::time::sleep(timespan).await;
    format!("Computation complete in {} millis", timespan.as_millis())
}


async fn track_request_metrics(req: Request<Body>, next: Next) -> Response<Body> {
    let start_time = Instant::now();
    let path = req.uri().path().to_string();

    let response = next.run(req).await;

    let duration = start_time.elapsed();

    println!(
        "[METRIC ARCHITECT] Route '{}' execution finished in {:.2?}",
        path, duration
    );

    response
}
