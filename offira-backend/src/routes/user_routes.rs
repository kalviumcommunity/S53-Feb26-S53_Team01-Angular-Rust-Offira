use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::user_handlers::get_users;

pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .with_state(pool)
}
