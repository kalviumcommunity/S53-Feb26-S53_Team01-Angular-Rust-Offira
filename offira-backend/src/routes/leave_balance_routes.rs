use crate::{
    handlers::leave_balance_handler::get_my_leave_balances,
    middleware::auth_middleware::auth_middleware,
};
use axum::{Router, middleware, routing::get};
use sqlx::PgPool;

pub fn leave_balance_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/leave-balances/me", get(get_my_leave_balances))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool)
}
