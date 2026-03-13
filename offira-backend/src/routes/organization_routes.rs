use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::handlers::organization_handler::{
    create_organization, get_organization_by_id, get_organizations, update_organization, delete_organization
};

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
        .with_state(pool)
}
