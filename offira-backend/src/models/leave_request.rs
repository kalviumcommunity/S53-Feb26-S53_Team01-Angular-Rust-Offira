use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Serialize, FromRow)]
pub struct LeaveRequest {
    pub id: i32,
    pub user_id: i32,
    pub organization_id: i32,
    pub policy_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateLeaveRequest {
    pub policy_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateLeaveStatus {
    pub status: String,
}