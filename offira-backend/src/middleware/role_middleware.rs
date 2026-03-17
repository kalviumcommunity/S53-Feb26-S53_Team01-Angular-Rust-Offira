use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::auth::claims::Claims;

// ADMIN ONLY
pub async fn admin_only(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req.extensions().get::<Claims>();

    match claims {
        Some(c) if c.role_id == 1 => Ok(next.run(req).await),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

// MANAGER OR ADMIN
pub async fn manager_or_admin(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req.extensions().get::<Claims>();

    match claims {
        Some(c) if c.role_id == 1 || c.role_id == 2 => Ok(next.run(req).await),
        _ => Err(StatusCode::FORBIDDEN),
    }
}