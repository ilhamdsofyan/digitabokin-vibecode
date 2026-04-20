use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::{invitation, user},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/{slug}", get(get_guest_view))
}

#[derive(Serialize)]
pub struct GuestViewRes {
    pub invitation_id: Uuid,
    pub title: String,
    pub design_state: sea_orm::JsonValue,
    pub music_url: Option<String>,
    pub host_name: Option<String>,
}

pub async fn get_guest_view(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<GuestViewRes>> {
    // Cari invitation berdasarkan custom slug-nya
    let inv = invitation::Entity::find()
        .filter(invitation::Column::Slug.eq(slug))
        .filter(invitation::Column::IsPublished.eq(true)) // Pastikan sudah di-publish!
        .one(&state.db)
        .await?
        .ok_or_else(|| {
            AppError::NotFound("Invitation not found or not published yet".to_string())
        })?;

    // Ambil data usernya (untuk nampilin Nama Host)
    let host = user::Entity::find_by_id(inv.user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "Host data missing for published invitation"
            ))
        })?;

    Ok(Json(GuestViewRes {
        invitation_id: inv.id,
        title: inv.title,
        design_state: inv.design_state,
        music_url: inv.music_url,
        host_name: host.full_name,
    }))
}
