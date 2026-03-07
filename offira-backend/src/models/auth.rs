use serde::Deserialize; // Deserialize - Json to rust struct

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
