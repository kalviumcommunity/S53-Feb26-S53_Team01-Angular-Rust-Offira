use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::auth::jwt::generate_token;
use crate::auth::password::verify_password;
use crate::errors::app_error::AppError;
use crate::models::auth::LoginRequest;
use crate::models::auth_response::AuthResponse;

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query!(
        "SELECT id, email, password_hash, role_id FROM users WHERE email = $1",
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

    if !verify_password(&user.password_hash, &payload.password) {
        return Err(AppError::internal("Invalid credentials"));
    }

    let token = generate_token(user.id, user.role_id);

    Ok(Json(AuthResponse { token }))
}
