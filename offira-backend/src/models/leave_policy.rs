use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct LeavePolicy {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub total_days: i32,
}

#[derive(Deserialize)]
pub struct CreateLeavePolicy {
    pub name: String,
    pub total_days: i32,
}