use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    errors::AppError,
    services::auth_service::{AuthService, TokenType},
};

/// Middleware to extract the JWT from the Authorization header and attach User ID to the request.
pub async fn require_auth(
    State(config): State<AppConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
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
    let claims = AuthService::verify_token(token, &config, TokenType::Access)?;

    // Insert user_id into request extensions for subsequent handlers
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}

/// Extractor to easily pull User ID out of requests in handler parameters.
pub struct AuthUser(pub Uuid);

use axum::extract::FromRequestParts;
use axum::http::request::Parts;

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id = parts.extensions.get::<Uuid>().ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "User ID not found in extensions. Did you forget `require_auth` middleware?"
            ))
        })?;

        Ok(AuthUser(*user_id))
    }
}
