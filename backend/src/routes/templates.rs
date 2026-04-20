use axum::{extract::State, routing::get, Json, Router};
use sea_orm::EntityTrait;

use crate::{errors::AppResult, models::template, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(list_templates))
}

pub async fn list_templates(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<template::Model>>> {
    let templates = template::Entity::find().all(&state.db).await?;
    Ok(Json(templates))
}
