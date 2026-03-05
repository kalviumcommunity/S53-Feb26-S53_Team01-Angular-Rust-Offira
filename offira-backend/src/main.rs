use axum::extract::State;
use axum::response::Json;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};
use std::env;
use tokio::net::TcpListener;

#[derive(Serialize, FromRow)]
struct UserWithRole {
    id: i32,
    full_name: String,
    email: String,
    role: String,
}

async fn health() -> &'static str {
    "Offira backend is running 🚀"
}

async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<UserWithRole>>, String> {
    let users = sqlx::query_as::<_, UserWithRole>(
        r#"
        SELECT users.id, users.full_name, users.email, roles.name as role
        FROM users
        JOIN roles ON users.role_id = roles.id
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(users))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    println!("✅ Connected to PostgreSQL");

    let app = Router::new()
        .route("/", get(health))
        .route("/users", get(get_users))
        .with_state(pool);

    let port = match env::var("PORT") {
        Ok(value) => value,
        Err(_) => {
            println!("PORT not set, using default 3000");
            "3000".to_string()
        }
    };

    let address = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&address).await?;

    println!("Server running at http://{}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
