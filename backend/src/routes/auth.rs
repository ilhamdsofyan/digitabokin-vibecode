use axum::{extract::State, routing::post, Json, Router};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::user,
    services::auth_service::AuthService,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    // 1. Cek email apakah sudah terdaftar
    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(&state.db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict(
            "Email is already registered".to_string(),
        ));
    }

    // 2. Hash password pakai Argon2
    let password_hash = AuthService::hash_password(&payload.password)?;
    let user_id = Uuid::new_v4();

    // 3. Simpan ke database
    let new_user = user::ActiveModel {
        id: Set(user_id),
        email: Set(payload.email.clone()),
        password_hash: Set(password_hash),
        full_name: Set(payload.full_name),
        created_at: Set(chrono::Utc::now().naive_utc()),
    };

    new_user.insert(&state.db).await?;

    // 4. Generate JWT tokens
    let (access_token, refresh_token) = AuthService::generate_tokens(user_id, &state.config)?;

    Ok(Json(AuthResponse {
        user_id,
        access_token,
        refresh_token,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    // 1. Cari user berdasarkan email
    let user_db = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.email))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    // 2. Verifikasi argon2 password
    let is_valid = AuthService::verify_password(&payload.password, &user_db.password_hash)?;

    if !is_valid {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // 3. Generate tokens
    let (access_token, refresh_token) = AuthService::generate_tokens(user_db.id, &state.config)?;

    Ok(Json(AuthResponse {
        user_id: user_db.id,
        access_token,
        refresh_token,
    }))
}
