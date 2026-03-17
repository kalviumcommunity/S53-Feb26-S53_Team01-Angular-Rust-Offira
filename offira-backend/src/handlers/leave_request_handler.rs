use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

use crate::auth::claims::Claims;
use axum::extract::Extension;

use crate::{
    errors::app_error::AppError,
    models::leave_request::{CreateLeaveRequest, LeaveRequest, UpdateLeaveStatus},
};

pub async fn create_leave_request(
    Extension(claims): Extension<Claims>,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLeaveRequest>,
) -> Result<Json<LeaveRequest>, AppError> {
    let leave = sqlx::query_as!(
        LeaveRequest,
        r#"
        INSERT INTO leave_requests
        (user_id, organization_id, policy_id, start_date, end_date, reason)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, organization_id, policy_id,
        start_date, end_date, reason, status
        "#,
        claims.user_id,
        claims.organization_id,
        payload.policy_id,
        payload.start_date,
        payload.end_date,
        payload.reason
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::internal("Database error"))?;

    Ok(Json(leave))
}

pub async fn get_leave_requests(
    Extension(claims): Extension<Claims>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<LeaveRequest>>, AppError> {
    let leaves = sqlx::query_as!(
        LeaveRequest,
        r#"
        SELECT id, user_id, organization_id, policy_id,
        start_date, end_date, reason, status
        FROM leave_requests
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        claims.user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| AppError::internal("Database error"))?;

    Ok(Json(leaves))
}

pub async fn get_team_leave_requests(
    Extension(claims): Extension<Claims>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<LeaveRequest>>, AppError> {
    let leaves = sqlx::query_as!(
        LeaveRequest,
        r#"
        SELECT id, user_id, organization_id, policy_id,
        start_date, end_date, reason, status
        FROM leave_requests
        WHERE organization_id = $1
        ORDER BY created_at DESC
        "#,
        claims.organization_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| AppError::internal("Database error"))?;

    Ok(Json(leaves))
}

pub async fn update_leave_request_status(
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateLeaveStatus>,
) -> Result<Json<String>, AppError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| AppError::internal("Transaction start failed"))?;

    let leave = sqlx::query!(
        r#"
        SELECT user_id, policy_id, start_date, end_date
        FROM leave_requests
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| AppError::not_found("Leave request not found"))?;

    let is_owner = leave.user_id == claims.user_id;
    let is_manager_or_admin = claims.role_id == 2 || claims.role_id == 1;

    if !is_owner && !is_manager_or_admin {
        return Err(AppError::forbidden("Not authorized"));
    }

    let leave_days = ((leave.end_date - leave.start_date).num_days() + 1) as i32;

    if payload.status == "APPROVED" {
        sqlx::query!(
            r#"
            UPDATE leave_balances
            SET remaining_days = remaining_days - $1
            WHERE user_id = $2 AND policy_id = $3
            "#,
            leave_days,
            leave.user_id,
            leave.policy_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| AppError::internal("Balance deduction failed"))?;
    }

    sqlx::query!(
        r#"
        UPDATE leave_requests
        SET status = $1
        WHERE id = $2
        "#,
        payload.status,
        id
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| AppError::internal("Leave update failed"))?;

    tx.commit()
        .await
        .map_err(|_| AppError::internal("Transaction commit failed"))?;

    Ok(Json("Leave request updated".to_string()))
}
