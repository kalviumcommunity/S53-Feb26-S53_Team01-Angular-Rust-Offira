use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

use crate::models::user::UserWithRole;

pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<UserWithRole>>, String> {
    let users = sqlx::query_as::<_, UserWithRole>(
        r#"
        SELECT users.id, users.full_name, users.email, roles.name as role
        FROM users
        JOIN roles ON users.role_id = roles.id
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(users))
}

pub async fn get_user_by_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<UserWithRole>, String> {
    let user = sqlx::query_as::<_, UserWithRole>(
        r#"SELECT users.id, users.full_name, users.email, roles.name as role
        FROM users
        JOIN roles ON users.role_id = roles.id
        WHERE users.id = $1"#,
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(user))
}
