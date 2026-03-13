use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;

use crate::models::organizations::{CreateOrganization, Organization, UpdateOrganization};

pub async fn create_organization(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateOrganization>,
) -> Json<Organization> {

    let org = sqlx::query_as!(
        Organization,
        r#"
        INSERT INTO organizations (name, industry, size, website, official_email)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, industry, size, website, official_email, created_at
        "#,
        payload.name,
        payload.industry,
        payload.size,
        payload.website,
        payload.official_email
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(org)
}

pub async fn get_organizations(
    State(pool): State<PgPool>,
) -> Json<Vec<Organization>> {

    let orgs = sqlx::query_as!(
        Organization,
        r#"
        SELECT id, name, industry, size, website, official_email, created_at
        FROM organizations
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(orgs)
}

pub async fn get_organization_by_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Json<Organization> {

    let org = sqlx::query_as!(
        Organization,
        r#"
        SELECT id, name, industry, size, website, official_email, created_at
        FROM organizations
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(org)
}

pub async fn update_organization(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateOrganization>,
) -> Json<Organization> {

    let org = sqlx::query_as!(
        Organization,
        r#"
        UPDATE organizations
        SET name = $1,
            industry = $2,
            size = $3,
            website = $4,
            official_email = $5
        WHERE id = $6
        RETURNING id, name, industry, size, website, official_email, created_at
        "#,
        payload.name,
        payload.industry,
        payload.size,
        payload.website,
        payload.official_email,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(org)
}

pub async fn delete_organization(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Json<String> {

    sqlx::query!(
        "DELETE FROM organizations WHERE id = $1",
        id
    )
    .execute(&pool)
    .await
    .unwrap();

    Json("Organization deleted successfully".to_string())
}