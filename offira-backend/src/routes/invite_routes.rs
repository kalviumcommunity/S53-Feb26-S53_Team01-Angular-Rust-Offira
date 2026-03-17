use crate::{
    handlers::invite_handler::{invite_user, set_password, validate_invite},
    middleware::{auth_middleware::auth_middleware, role_middleware::admin_only},
};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::PgPool;

pub fn invite_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users/invite", post(invite_user))
        .route("/auth/invite/{token}", get(validate_invite))
        .route("/auth/set-password", post(set_password))
        .layer(middleware::from_fn(admin_only))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool)
}
