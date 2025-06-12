use core::{errors::AppError, models::{Claims, User}};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn create_token(&self, user: &User) -> Result<String, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // Token läuft nach 24 Stunden ab

        let claims = Claims {
            sub: user.id.clone(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
            role: user.role.clone(),
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())?;
        Ok(token_data.claims)
    }

    pub fn extract_token_from_header(auth_header: &str) -> Result<&str, AppError> {
        if auth_header.starts_with("Bearer ") {
            Ok(&auth_header[7..])
        } else {
            Err(AppError::Unauthorized("Invalid authorization header format".to_string()))
        }
    }
}