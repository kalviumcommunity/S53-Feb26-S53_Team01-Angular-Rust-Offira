use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

use crate::errors::app_error::AppError;
use crate::models::user::UserWithRole;

pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<UserWithRole>>, AppError> {
    let users = sqlx::query_as::<_, UserWithRole>(
        r#"
        SELECT users.id, users.full_name, users.email, roles.name as role
        FROM users
        JOIN roles ON users.role_id = roles.id
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| AppError::internal("Failed to fetch users"))?;

    Ok(Json(users))
}

pub async fn get_user_by_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<UserWithRole>, AppError> {
    let user = sqlx::query_as::<_, UserWithRole>(
        r#"SELECT users.id, users.full_name, users.email, roles.name as role
        FROM users
        JOIN roles ON users.role_id = roles.id
        WHERE users.id = $1"#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if matches!(e, sqlx::Error::RowNotFound) {
            AppError::not_found("User not found")
        } else {
            AppError::internal("Database error")
        }
    })?;
    Ok(Json(user))
}
