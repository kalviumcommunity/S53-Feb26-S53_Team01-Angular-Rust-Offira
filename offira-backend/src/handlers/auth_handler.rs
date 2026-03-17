use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::auth::jwt::generate_token;
use crate::auth::password::verify_password;
use crate::errors::app_error::AppError;
use crate::models::auth::LoginRequest;
use crate::models::auth_response::AuthResponse;

struct LoginUser {
    id: i32,
    organization_id: i32,
    password_hash: Option<String>,
    role_id: i32,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as!(
        LoginUser,
        "SELECT id, password_hash, role_id, organization_id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if matches!(e, sqlx::Error::RowNotFound) {
            AppError::not_found("Invalid credentials")
        } else {
            AppError::internal("Database error")
        }
    })?;

    let password_hash = user
        .password_hash
        .ok_or(AppError::internal("User has no password"))?;

    if !verify_password(&password_hash, &payload.password) {
        return Err(AppError::internal("Invalid credentials"));
    }

    let token = generate_token(user.id, user.role_id, user.organization_id);

    Ok(Json(AuthResponse { token }))
}
