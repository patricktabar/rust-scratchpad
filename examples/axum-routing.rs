use axum::{Router, extract::State, routing::get};

use std::sync::Arc;

struct AppState {
    api_version: String,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        api_version: "1.0".to_string(),
    });

    let app = Router::new()
        .route("/version", get(get_version))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server architecture deployed to http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_version(State(state): State<Arc<AppState>>) -> String {
    format!("API Version: {}", state.api_version)
}
