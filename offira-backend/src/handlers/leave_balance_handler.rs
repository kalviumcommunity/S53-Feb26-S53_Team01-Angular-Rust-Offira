use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{
    errors::app_error::AppError,
    models::leave_balance::LeaveBalance,
};

pub async fn get_my_leave_balances(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<LeaveBalance>>, AppError> {

    let balances = sqlx::query_as!(
        LeaveBalance,
        r#"
        SELECT
            lb.policy_id,
            lp.name as policy_name,
            lb.remaining_days
        FROM leave_balances lb
        JOIN leave_policies lp
        ON lb.policy_id = lp.id
        WHERE lb.user_id = $1
        "#,
        1
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| AppError::internal("Database error"))?;

    Ok(Json(balances))
}