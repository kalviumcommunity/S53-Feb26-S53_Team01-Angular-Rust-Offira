use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode, DecodingKey, Validation, decode};

use crate::auth::claims::Claims;

const SECRET: &[u8] = b"super_secret_key";

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;

    Ok(decoded.claims)
}

pub fn generate_token(user_id: i32, role_id: i32) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        user_id,
        role_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap()
}
