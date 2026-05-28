use axum::{Json, Router, response::IntoResponse, routing::post};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "guest".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/register", post(handle_registration));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("High-performance ingestion API live at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_registration(Json(payload): Json<RegisterUser>) -> impl IntoResponse {
    format!(
        "Architect Ingestion Success: User '{}' with email '{}' assigned role '{}'.",
        payload.username, payload.email, payload.role
    )
}
