use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{
    errors::app_error::AppError,
    models::leave_policy::{LeavePolicy, CreateLeavePolicy},
};

pub async fn create_leave_policy(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLeavePolicy>,
) -> Result<Json<LeavePolicy>, AppError> {

    let policy = sqlx::query_as!(
        LeavePolicy,
        r#"
        INSERT INTO leave_policies (organization_id, name, total_days)
        VALUES ($1, $2, $3)
        RETURNING id, organization_id, name, total_days
        "#,
        1,
        payload.name,
        payload.total_days
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::internal("Database error"))?;

    Ok(Json(policy))
}

pub async fn get_leave_policies(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<LeavePolicy>>, AppError> {

    let policies = sqlx::query_as!(
        LeavePolicy,
        r#"
        SELECT id, organization_id, name, total_days
        FROM leave_policies
        WHERE organization_id = $1
        "#,
        1
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| AppError::internal("Database error"))?;

    Ok(Json(policies))
}