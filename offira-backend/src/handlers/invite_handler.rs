use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::password::hash_password, errors::app_error::AppError, models::invite::InviteUserRequest,
    models::invite::SetPasswordRequest,
};

pub async fn validate_invite(
    Path(token): Path<String>,
    State(pool): State<PgPool>,
) -> Result<Json<String>, AppError> {
    let invite = sqlx::query!(
        r#"
        SELECT email, expires_at
        FROM user_invites
        WHERE token = $1
        "#,
        token
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::not_found("Invalid invite token"))?;

    if invite.expires_at < Utc::now().naive_utc() {
        return Err(AppError::internal("Invite expired"));
    }

    Ok(Json(invite.email))
}

pub async fn set_password(
    State(pool): State<PgPool>,
    Json(payload): Json<SetPasswordRequest>,
) -> Result<Json<String>, AppError> {
    // 1️⃣ Fetch invite
    let invite = sqlx::query!(
        r#"
        SELECT email, organization_id, role_id, expires_at
        FROM user_invites
        WHERE token = $1
        "#,
        payload.token
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::not_found("Invalid invite token"))?;

    // 2️⃣ Expiration check
    if invite.expires_at < Utc::now().naive_utc() {
        return Err(AppError::internal("Invite expired"));
    }

    // 3️⃣ Duplicate user check
    let existing_user = sqlx::query!("SELECT id FROM users WHERE email = $1", invite.email)
        .fetch_optional(&pool)
        .await
        .map_err(|_| AppError::internal("Database error"))?;

    if existing_user.is_some() {
        return Err(AppError::internal("User already exists"));
    }

    // 4️⃣ Hash password
    let password_hash = hash_password(&payload.password);

    // 5️⃣ Create user
    sqlx::query!(
        r#"
        INSERT INTO users (full_name, email, password_hash, role_id, organization_id)
        VALUES ($1,$2,$3,$4,$5)
        "#,
        payload.full_name,
        invite.email,
        password_hash,
        invite.role_id,
        invite.organization_id
    )
    .execute(&pool)
    .await
    .map_err(|_| AppError::internal("Failed to create user"))?;

    // 6️⃣ Delete invite (prevent reuse)
    sqlx::query!(
        r#"
        DELETE FROM user_invites
        WHERE token = $1
        "#,
        payload.token
    )
    .execute(&pool)
    .await
    .map_err(|_| AppError::internal("Failed to delete invite"))?;

    Ok(Json("Account created".to_string()))
}

pub async fn invite_user(
    State(pool): State<PgPool>,
    Json(payload): Json<InviteUserRequest>,
) -> Result<Json<String>, AppError> {
    let token = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::hours(24)).naive_utc();

    sqlx::query!(
        r#"
        INSERT INTO user_invites (email, organization_id, role_id, token, expires_at)
        VALUES ($1,$2,$3,$4,$5)
        "#,
        payload.email,
        1,
        payload.role_id,
        token,
        expires_at
    )
    .execute(&pool)
    .await
    .map_err(|_| AppError::internal("Failed to create invite"))?;

    Ok(Json("Invite created".to_string()))
}
