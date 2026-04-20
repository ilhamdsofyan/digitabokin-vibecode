use axum::{
    extract::{Path, State},
    middleware,
    routing::{get, post, put},
    Json, Router,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::invitation,
    services::invitation_service::InvitationService,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Protected routes (Protected by AuthUser extractor present in handlers)
        .route("/", post(create_invitation).get(list_invitations))
        .route("/:id", get(get_invitation))
        .route("/:id/design", put(update_design))
}

#[derive(Deserialize)]
pub struct CreateInvitationReq {
    pub template_id: i32,
    pub title: String,
    pub slug: String,
}

pub async fn create_invitation(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateInvitationReq>,
) -> AppResult<Json<invitation::Model>> {
    let inv = InvitationService::create_from_template(
        &state.db,
        user_id,
        payload.template_id,
        &payload.title,
        &payload.slug,
    )
    .await?;

    Ok(Json(inv))
}

pub async fn list_invitations(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> AppResult<Json<Vec<invitation::Model>>> {
    let invs = invitation::Entity::find()
        .filter(invitation::Column::UserId.eq(user_id))
        .all(&state.db)
        .await?;

    Ok(Json(invs))
}

pub async fn get_invitation(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<invitation::Model>> {
    let inv = invitation::Entity::find_by_id(id)
        .filter(invitation::Column::UserId.eq(user_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Invitation not found".to_string()))?;

    Ok(Json(inv))
}

#[derive(Deserialize)]
pub struct UpdateDesignReq {
    pub design_state: sea_orm::JsonValue,
    pub expected_version: i32,
}

pub async fn update_design(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateDesignReq>,
) -> AppResult<Json<invitation::Model>> {
    let inv = InvitationService::update_design_state(
        &state.db,
        user_id,
        id,
        payload.design_state,
        payload.expected_version,
    )
    .await?;

    Ok(Json(inv))
}
