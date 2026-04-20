use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::{invitation, rsvp_response},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Public endpoints
        .route("/{invitation_id}", post(submit_rsvp))
        // Protected endpoints (Membutuhkan AuthUser dalam handler)
        .route("/{invitation_id}", get(list_rsvps))
}

#[derive(Deserialize)]
pub struct SubmitRsvpReq {
    pub guest_name: String,
    pub attendance_status: String,
    pub guest_count: i32,
    pub message: Option<String>,
    pub extra_data: Option<sea_orm::JsonValue>,
}

#[derive(Serialize)]
pub struct SubmitRsvpRes {
    pub success: bool,
    pub rsvp_id: i32,
}

/// Endpoint untuk tamu submit RSVP form (Public)
pub async fn submit_rsvp(
    State(state): State<AppState>,
    Path(invitation_id): Path<Uuid>,
    Json(payload): Json<SubmitRsvpReq>,
) -> AppResult<Json<SubmitRsvpRes>> {
    // Pastikan invitation-nya ada
    let inv = invitation::Entity::find_by_id(invitation_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Invitation not found".to_string()))?;

    // Masukkan RSVP tamu
    let new_rsvp = rsvp_response::ActiveModel {
        invitation_id: Set(inv.id),
        guest_name: Set(payload.guest_name),
        attendance_status: Set(payload.attendance_status),
        guest_count: Set(payload.guest_count),
        message: Set(payload.message),
        extra_data: Set(payload.extra_data),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let result = new_rsvp.insert(&state.db).await?;

    Ok(Json(SubmitRsvpRes {
        success: true,
        rsvp_id: result.id,
    }))
}

/// Endpoint untuk Owner ngeliat daftar tamunya (Protected)
pub async fn list_rsvps(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(invitation_id): Path<Uuid>,
) -> AppResult<Json<Vec<rsvp_response::Model>>> {
    // Cek dulu apakah invitation ini milik user yang lagi request
    let is_owner = invitation::Entity::find()
        .filter(invitation::Column::Id.eq(invitation_id))
        .filter(invitation::Column::UserId.eq(user_id))
        .one(&state.db)
        .await?
        .is_some();

    if !is_owner {
        return Err(AppError::Forbidden(
            "You are not the owner of this invitation".to_string(),
        ));
    }

    // Ambil semua data RSVP berdasarkan invitation id
    let rsvps = rsvp_response::Entity::find()
        .filter(rsvp_response::Column::InvitationId.eq(invitation_id))
        .all(&state.db)
        .await?;

    Ok(Json(rsvps))
}
