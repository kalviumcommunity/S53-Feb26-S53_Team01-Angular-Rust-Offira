use crate::handlers::leave_request_handler::{
    create_leave_request,
    get_leave_requests,
    get_team_leave_requests,
    update_leave_request_status,
};

use crate::middleware::auth_middleware::auth_middleware;
use crate::middleware::role_middleware::manager_or_admin;

use axum::{
    Router,
    routing::{get, post, patch},
    middleware,
};

use sqlx::PgPool;

pub fn leave_routes(pool: PgPool) -> Router {
    
    let employee_routes = Router::new()
        .route("/leave-requests", post(create_leave_request))
        .route("/leave-requests/me", get(get_leave_requests))
        .layer(middleware::from_fn(auth_middleware));

    let manager_routes = Router::new()
        .route("/leave-requests/team", get(get_team_leave_requests))
        .route("/leave-requests/{id}", patch(update_leave_request_status))
        .layer(middleware::from_fn(manager_or_admin))
        .layer(middleware::from_fn(auth_middleware));

    employee_routes
        .merge(manager_routes)
        .with_state(pool)
}