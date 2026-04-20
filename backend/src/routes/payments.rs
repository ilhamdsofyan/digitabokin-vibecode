use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::AppResult,
    middleware::auth::AuthUser,
    services::payment_service::PaymentService,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/checkout", post(checkout))
        .route("/midtrans-webhook", post(midtrans_webhook))
}

#[derive(Deserialize)]
pub struct CheckoutReq {
    pub invitation_id: Uuid,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct CheckoutRes {
    pub snap_token: String,
    pub snap_url: String,
    pub order_id: String,
}

/// Endpoint untuk ping Midtrans Snap Token
pub async fn checkout(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CheckoutReq>,
) -> AppResult<Json<CheckoutRes>> {
    let payment = PaymentService::create_snap_transaction(
        &state.db,
        &state.config,
        user_id,
        payload.invitation_id,
        payload.amount,
    )
    .await?;

    Ok(Json(CheckoutRes {
        snap_token: payment.midtrans_snap_token.unwrap_or_default(),
        snap_url: payment.midtrans_snap_url.unwrap_or_default(),
        order_id: payment.order_id,
    }))
}

#[derive(Deserialize)]
pub struct MidtransNotificationReq {
    pub order_id: String,
    pub transaction_status: String,
    // Midtrans ngirim payload gede banget, tapi kita cuma butuh 2 fields ini buat logic dasar
}

/// Webhook endpoint khusus diserang oleh Server Midtrans (Public)
pub async fn midtrans_webhook(
    State(state): State<AppState>,
    Json(payload): Json<MidtransNotificationReq>,
) -> AppResult<&'static str> {
    // Pada production aslinya, Endpoint ini wajib verifikasi signature midtrans_signature_key!
    // Untuk Fase ini, kita pake Transaction Status murni dari web payload

    PaymentService::handle_midtrans_webhook(
        &state.db,
        &payload.order_id,
        &payload.transaction_status,
    )
    .await?;

    Ok("OK")
}
