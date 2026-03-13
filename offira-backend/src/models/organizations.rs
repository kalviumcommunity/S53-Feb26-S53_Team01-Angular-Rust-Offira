use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub website: Option<String>,
    pub official_email: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub website: Option<String>,
    pub official_email: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateOrganization {
    pub name: String,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub website: Option<String>,
    pub official_email: Option<String>,
}