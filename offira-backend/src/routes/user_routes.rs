use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::user_handlers::get_users;
use crate::handlers::user_handlers::get_user_by_id;

pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user_by_id))
        .with_state(pool)
}
