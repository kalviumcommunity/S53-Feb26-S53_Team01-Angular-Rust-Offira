use serde::Serialize; // used to convert Rust structs into JSON format for API responses
use sqlx::FromRow; // used to convert database rows into Rust structs

#[derive(Serialize, FromRow)]
pub struct LeaveBalance {
    pub policy_id: i32,
    pub policy_name: String,
    pub remaining_days: i32,
}