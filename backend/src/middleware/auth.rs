use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    services::auth_service::{AuthService, TokenType},
    AppState,
};

/// Extractor to easily pull User ID out of requests and verify JWT automatically.
pub struct AuthUser(pub Uuid);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                auth_header.trim_start_matches("Bearer ")
            } else {
                return Err(AppError::Unauthorized(
                    "Invalid Authorization header format".to_string(),
                ));
            }
        } else {
            return Err(AppError::Unauthorized(
                "Missing Authorization header".to_string(),
            ));
        };

        // Verify token validity
        let claims = AuthService::verify_token(token, &state.config, TokenType::Access)?;

        Ok(AuthUser(claims.sub))
    }
}
