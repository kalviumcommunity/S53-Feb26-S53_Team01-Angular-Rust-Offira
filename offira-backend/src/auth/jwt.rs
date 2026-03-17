use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::auth::claims::Claims;

fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes()
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = get_secret();

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret),
        &Validation::default(),
    )?;

    Ok(decoded.claims)
}

pub fn generate_token(user_id: i32, role_id: i32) -> String {
    let secret = get_secret();

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Failed to calculate token expiration")
        .timestamp() as usize;

    let claims = Claims {
        user_id,
        role_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret),
    )
    .expect("Failed to generate JWT token")
}
