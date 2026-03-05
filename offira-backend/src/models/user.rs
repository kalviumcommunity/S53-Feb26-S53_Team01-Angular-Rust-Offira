use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct UserWithRole {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub role: String,
}