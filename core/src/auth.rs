use core::errors::AppError;
use crate::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub fn create_jwt_token(user_id: &str, secret: &str) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn verify_user(username: &str, password: &str) -> Result<Claims, AppError> {
    if username == "testuser" && password == "password123" {
        let claims = Claims {
            sub: "testuser".to_string(),
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        Ok(claims)
    } else {
        Err(AppError::Unauthorized("Invalid credentials".to_string()))
    }
}