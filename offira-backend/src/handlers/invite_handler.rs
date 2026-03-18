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

use crate::utils::email::{build_invite_email, send_email};
use std::env;

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

    if invite.expires_at < Utc::now().naive_utc() {
        return Err(AppError::internal("Invite expired"));
    }

    let existing_user = sqlx::query!("SELECT id FROM users WHERE email = $1", invite.email)
        .fetch_optional(&pool)
        .await
        .map_err(|_| AppError::internal("Database error"))?;

    if existing_user.is_some() {
        return Err(AppError::internal("User already exists"));
    }

    let password_hash = hash_password(&payload.password);

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

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| AppError::internal("Failed to start transaction"))?;

    let result = sqlx::query!(
        r#"
        INSERT INTO user_invites (email, organization_id, role_id, token, expires_at, is_active)
        VALUES ($1,$2,$3,$4,$5, TRUE)
        "#,
        payload.email,
        1,
        payload.role_id,
        token,
        expires_at
    )
    .execute(&mut *tx)
    .await;

    if let Err(e) = result {
        tx.rollback().await.ok();

        if e.to_string().contains("unique_active_invite") {
            return Err(AppError::internal("User already invited"));
        }

        return Err(AppError::internal("Failed to create invite"));
    }

    let frontend_url =
        env::var("FRONTEND_URL").map_err(|_| AppError::internal("Missing frontend URL"))?;

    let invite_link = format!("{}/accept-invite?token={}", frontend_url, token);

    let html = build_invite_email(&invite_link);

    if let Err(e) = send_email(&payload.email, "You're invited!", &html).await {
        tx.rollback().await.ok();
        return Err(AppError::internal(&e));
    }

    tx.commit()
        .await
        .map_err(|_| AppError::internal("Failed to commit transaction"))?;

    Ok(Json("Invite sent successfully".to_string()))
}