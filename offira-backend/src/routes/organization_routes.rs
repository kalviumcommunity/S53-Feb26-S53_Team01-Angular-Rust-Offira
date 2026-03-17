use axum::{
    Router, middleware, routing::{get, post}
};
use sqlx::PgPool;

use crate::{handlers::organization_handler::{
    create_organization, delete_organization, get_organization_by_id, get_organizations,
    update_organization,
}, middleware::{auth_middleware::auth_middleware, role_middleware::admin_only}};

pub fn organization_routes(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/organizations",
            post(create_organization).get(get_organizations),
        )
        .route(
            "/organizations/{id}",
            get(get_organization_by_id)
                .put(update_organization)
                .delete(delete_organization),
        )
        .layer(middleware::from_fn(admin_only))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool)
}
