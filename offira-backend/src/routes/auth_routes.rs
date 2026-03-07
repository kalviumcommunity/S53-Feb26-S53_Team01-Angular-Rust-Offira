use crate::handlers::auth_handler::login;
use axum::{Router, routing::post};
use sqlx::{PgPool};

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .with_state(pool)
}
