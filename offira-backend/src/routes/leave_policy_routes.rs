use crate::handlers::leave_policy_handler::{create_leave_policy, get_leave_policies};
use axum::{
    Router, middleware, routing::{get, post}
};
use sqlx::PgPool;

use crate::middleware::{auth_middleware::auth_middleware, role_middleware::admin_only};

pub fn leave_policy_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/leave-policies", post(create_leave_policy))
        .route("/leave-policies", get(get_leave_policies))
        .layer(middleware::from_fn(admin_only))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool)
}
