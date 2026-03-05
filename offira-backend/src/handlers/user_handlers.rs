use axum::{Json, extract::State};
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
