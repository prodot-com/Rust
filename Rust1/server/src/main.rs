use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Routes
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/user", post(create_user));

    // Bind listener
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running at http://127.0.0.1:3000");

    // Start server (Axum 0.7 way)
    axum::serve(listener, app).await.unwrap();
}

// Handlers
async fn root() -> &'static str {
    "Welcome to Rust Server!"
}

async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello from Rust!".to_string(),
    })
}

async fn create_user(Json(payload): Json<User>) -> Json<User> {
    Json(payload)
}

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    age: u8,
}
