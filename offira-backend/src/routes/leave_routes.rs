use crate::handlers::leave_request_handler::{
    create_leave_request, get_leave_requests, get_team_leave_requests, update_leave_request_status,
};
use axum::{
    Router,
    routing::{get, post, patch},
};
use sqlx::PgPool;

pub fn leave_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/leave-requests", post(create_leave_request))
        .route("/leave-requests/me", get(get_leave_requests))
        .route("/leave-requests/team", get(get_team_leave_requests))
        .route("/leave-requests/{id}", patch(update_leave_request_status))
        .with_state(pool)
}
