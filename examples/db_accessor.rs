use sqlx::PgPool;
use serde::Serialize;
use std::sync::Arc;
use axum::{routing::get, extract::State, Router, Json, http::StatusCode};

struct AppContext {
    db_pool: PgPool,
}

#[derive(Serialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub active: bool,
}

#[tokio::main]
async fn main() {
    let db_url = "postgres://ptabar:1234@localhost/mydb";
    let pool = PgPool::connect(db_url).await.expect("Failed to connect to database pool");

    let context = Arc::new(AppContext { db_pool: pool });

    let app = Router::new()
        .route("/user", get(fetch_active_user))
        .with_state(context);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();


    println!("Type-Safe Data Architecture deployed to http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

// This functions does the following:
// 1. It defines an asynchronous function `fetch_active_user` that takes the application context
//   as an argument, which contains the database connection pool.
// 2. It executes a SQL query to fetch an active user from the `users` table, mapping
// the result to a `UserProfile` struct.
// 3. If the query is successful, it returns the user profile as a JSON response. If there is an error during the database query, it returns an error message.
// This function demonstrates how to perform a database query in an asynchronous context using `sqlx` and return the result as a JSON response in an Axum web application.
async fn fetch_active_user(State(ctx): State<Arc<AppContext>>) -> Result<Json<UserProfile>, (StatusCode, String)> {
    let user = sqlx::query_as!(
        UserProfile,
        "SELECT id, username, active FROM users WHERE active = true LIMIT 1"
    )
    .fetch_optional(&ctx.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err((StatusCode::NOT_FOUND, "No active user found".to_string())),
    }
}
