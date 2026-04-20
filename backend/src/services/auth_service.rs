use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    errors::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

pub struct AuthService;

impl AuthService {
    /// Hash a raw password using Argon2
    pub fn hash_password(password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hash error: {}", e)))?
            .to_string();

        Ok(password_hash)
    }

    /// Verify a plain text password against an Argon2 hash
    pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid hash format: {}", e)))?;

        let is_valid = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    }

    /// Generate an Access Token and a Refresh Token
    pub fn generate_tokens(user_id: Uuid, config: &AppConfig) -> AppResult<(String, String)> {
        let now = Utc::now();
        let access_exp = now + Duration::seconds(config.jwt_access_expiry_secs);
        let refresh_exp = now + Duration::seconds(config.jwt_refresh_expiry_secs);

        let access_claims = Claims {
            sub: user_id,
            exp: access_exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            token_type: TokenType::Access,
        };

        let refresh_claims = Claims {
            sub: user_id,
            exp: refresh_exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            token_type: TokenType::Refresh,
        };

        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_bytes());

        let access_token = encode(&Header::default(), &access_claims, &encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encoding error: {}", e)))?;

        let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encoding error: {}", e)))?;

        Ok((access_token, refresh_token))
    }

    /// Verify a token and ensure it matches the requested TokenType
    pub fn verify_token(
        token: &str,
        config: &AppConfig,
        expected_type: TokenType,
    ) -> AppResult<Claims> {
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        let validation = Validation::default();

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

        if token_data.claims.token_type != expected_type {
            return Err(AppError::Unauthorized("Invalid token type".to_string()));
        }

        Ok(token_data.claims)
    }
}
