use serde::Deserialize;

#[derive(Deserialize)]
pub struct InviteUserRequest {
    pub email: String,
    pub role_id: i32,
}

#[derive(Deserialize)]
pub struct SetPasswordRequest {
    pub token: String,
    pub full_name: String,
    pub password: String,
}
