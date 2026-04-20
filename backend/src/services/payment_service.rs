use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    errors::{AppError, AppResult},
    models::{invitation, payment},
};

pub struct PaymentService;

#[derive(Serialize)]
struct MidtransChargeReq {
    transaction_details: TransactionDetails,
}

#[derive(Serialize)]
struct TransactionDetails {
    order_id: String,
    gross_amount: f64,
}

#[derive(Deserialize)]
struct MidtransChargeRes {
    token: String,
    redirect_url: String,
}

impl PaymentService {
    pub async fn create_snap_transaction(
        db: &DatabaseConnection,
        config: &AppConfig,
        user_id: Uuid,
        invitation_id: Uuid,
        amount: f64,
    ) -> AppResult<payment::Model> {
        // Cek dulu apakah undangan benar milik user ini
        let _inv = invitation::Entity::find_by_id(invitation_id)
            .filter(invitation::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Invitation not found".to_string()))?;

        // 1. Generate Order ID
        let order_id = format!(
            "DIGI-{}-{}",
            Utc::now().timestamp(),
            Uuid::new_v4()
                .to_string()
                .chars()
                .take(5)
                .collect::<String>()
        );

        // 2. Tembak Midtrans API
        let server_key = &config.midtrans_server_key;
        let base_url = if config.midtrans_is_production {
            "https://app.midtrans.com/snap/v1/transactions"
        } else {
            "https://app.sandbox.midtrans.com/snap/v1/transactions"
        };

        // Midtrans membutuhkan HTTP Basic Auth format: `ServerKey:` encode Base64
        let auth_string = format!("{}:", server_key);
        let b64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

        let payload = MidtransChargeReq {
            transaction_details: TransactionDetails {
                order_id: order_id.clone(),
                gross_amount: amount,
            },
        };

        let client = Client::new();
        let res = client
            .post(base_url)
            .header("Authorization", format!("Basic {}", b64_auth))
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Midtrans request failed: {}", e)))?;

        if !res.status().is_success() {
            let err_body = res.text().await.unwrap_or_default();
            return Err(AppError::Internal(anyhow::anyhow!(
                "Midtrans error: {}",
                err_body
            )));
        }

        let snap_res: MidtransChargeRes = res.json().await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed parsing Midtrans response: {}", e))
        })?;

        // 3. Simpan ke Database
        use rust_decimal::prelude::FromPrimitive;
        let decimal_amount = rust_decimal::Decimal::from_f64(amount).unwrap_or_default();

        let new_payment = payment::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            invitation_id: Set(invitation_id),
            order_id: Set(order_id),
            amount: Set(decimal_amount),
            status: Set("PENDING".to_string()),
            midtrans_snap_token: Set(Some(snap_res.token.clone())),
            midtrans_snap_url: Set(Some(snap_res.redirect_url.clone())),
            paid_at: Set(None),
            created_at: Set(Utc::now().naive_utc()),
        };

        let result = new_payment.insert(db).await?;
        Ok(result)
    }

    pub async fn handle_midtrans_webhook(
        db: &DatabaseConnection,
        order_id: &str,
        transaction_status: &str,
    ) -> AppResult<()> {
        let payment_record = payment::Entity::find()
            .filter(payment::Column::OrderId.eq(order_id))
            .one(db)
            .await?;

        if let Some(payment) = payment_record {
            let mut active_payment: payment::ActiveModel = payment.clone().into();

            // Map Midtrans status -> Our status
            let new_status = match transaction_status {
                "settlement" | "capture" => "PAID",
                "expire" => "EXPIRED",
                "cancel" | "deny" => "FAILED",
                _ => "PENDING",
            };

            active_payment.status = Set(new_status.to_string());
            if new_status == "PAID" {
                active_payment.paid_at = Set(Some(Utc::now().naive_utc()));
            }
            active_payment.update(db).await?;

            // Jika dibayar, langsung PUBLISH undangannya otomatis
            if new_status == "PAID" {
                let inv = invitation::Entity::find_by_id(payment.invitation_id)
                    .one(db)
                    .await?;
                if let Some(inv) = inv {
                    let mut active_inv: invitation::ActiveModel = inv.into();
                    active_inv.is_published = Set(true);
                    active_inv.update(db).await?;
                }
            }
        }

        Ok(())
    }
}
