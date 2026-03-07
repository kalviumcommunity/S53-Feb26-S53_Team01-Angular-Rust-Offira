use axum::middleware;
use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::user_handlers::get_users;
use crate::handlers::user_handlers::get_user_by_id;
use crate::middleware::auth_middleware::auth_middleware;

pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user_by_id))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool)
}
