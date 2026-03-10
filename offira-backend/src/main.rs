use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod db;
mod handlers;
mod models;
mod routes;
mod auth;
mod middleware;
mod errors;

use db::connection::connect_db;

use crate::routes::auth_routes::auth_routes;
use crate::routes::user_routes::user_routes;

async fn health() -> &'static str {
    "Offira backend is running 🚀"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = connect_db().await;

    println!("✅ Connected to PostgreSQL");

    let app = Router::new()
        .route("/", get(health))
        .merge(user_routes(pool.clone()))
        .merge(auth_routes(pool.clone()));

    let port = env::var("PORT").unwrap_or("3000".to_string());

    let address = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&address).await?;

    println!("Server running at http://{}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
