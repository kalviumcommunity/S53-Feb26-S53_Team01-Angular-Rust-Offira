use crate::handlers::leave_policy_handler::{create_leave_policy, get_leave_policies};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub fn leave_policy_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/leave-policies", post(create_leave_policy))
        .route("/leave-policies", get(get_leave_policies))
        .with_state(pool)
}
