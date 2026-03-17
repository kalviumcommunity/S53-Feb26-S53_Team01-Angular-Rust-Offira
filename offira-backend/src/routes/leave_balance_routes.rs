use crate::handlers::leave_balance_handler::{
    get_my_leave_balances
};
use axum::{
    Router,
    routing::{get},
};
use sqlx::PgPool;

pub fn leave_balance_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/leave-balances/me", get(get_my_leave_balances))
        .with_state(pool)
}
