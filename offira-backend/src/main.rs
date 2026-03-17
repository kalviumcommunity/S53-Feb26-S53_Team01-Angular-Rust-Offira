use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod auth;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use db::connection::connect_db;

use crate::routes::auth_routes::auth_routes;
use crate::routes::invite_routes::invite_routes;
use crate::routes::leave_balance_routes::leave_balance_routes;
use crate::routes::leave_policy_routes::leave_policy_routes;
use crate::routes::leave_routes::leave_routes;
use crate::routes::organization_routes::organization_routes;
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
        .merge(auth_routes(pool.clone()))
        .merge(organization_routes(pool.clone()))
        .merge(leave_balance_routes(pool.clone()))
        .merge(leave_policy_routes(pool.clone()))
        .merge(invite_routes(pool.clone()))
        .merge(leave_routes(pool.clone()));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let address = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&address).await?;

    println!("Server running at http://{}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
