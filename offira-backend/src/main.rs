use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn health() -> &'static str {
    "Offira backend is running 🚀"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}