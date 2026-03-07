use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::auth::jwt::generate_token;
use crate::auth::password::verify_password;
use crate::models::auth::LoginRequest;
use crate::models::auth_response::AuthResponse;

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Json<AuthResponse> {
    let user = sqlx::query!(
        "SELECT id, email, password_hash, role_id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool)
    .await;

    let user = match user {
        Ok(u) => u,
        Err(_) => {
            println!("User not found");
            return Json(AuthResponse {
                token: "".to_string(),
            });
        }
    };

    if !verify_password(&user.password_hash, &payload.password) {
        println!("Invalid password");
        return Json(AuthResponse {
            token: "".to_string(),
        });
    }

    let token = generate_token(user.id, user.role_id);

    Json(AuthResponse { token })
}
