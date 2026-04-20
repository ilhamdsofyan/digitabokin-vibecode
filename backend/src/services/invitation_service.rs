use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::{invitation, template},
};

pub struct InvitationService;

impl InvitationService {
    pub async fn create_from_template(
        db: &DatabaseConnection,
        user_id: Uuid,
        template_id: i32,
        title: &str,
        slug: &str,
    ) -> AppResult<invitation::Model> {
        // Find the template
        let tpl = template::Entity::find_by_id(template_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Template not found".to_string()))?;

        // Check unique slug
        let existing = invitation::Entity::find()
            .filter(invitation::Column::Slug.eq(slug))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("Slug is already in use".to_string()));
        }

        // Create invitation using template's base_design_state
        let new_invitation = invitation::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            template_id: Set(template_id),
            slug: Set(slug.to_string()),
            title: Set(title.to_string()),
            thumbnail_url: Set(None),
            music_url: Set(None),
            is_published: Set(false),
            design_state: Set(tpl.base_design_state),
            version: Set(1),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let result = new_invitation.insert(db).await?;
        Ok(result)
    }

    pub async fn update_design_state(
        db: &DatabaseConnection,
        user_id: Uuid,
        invitation_id: Uuid,
        new_design_state: sea_orm::JsonValue,
        expected_version: i32,
    ) -> AppResult<invitation::Model> {
        let inv = invitation::Entity::find_by_id(invitation_id)
            .filter(invitation::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Invitation not found".to_string()))?;

        // Optimistic locking check
        if inv.version != expected_version {
            return Err(AppError::Conflict(
                "Design state has been modified by another session. Please reload.".to_string(),
            ));
        }

        let mut active_inv: invitation::ActiveModel = inv.into();
        active_inv.design_state = Set(new_design_state);
        active_inv.version = Set(expected_version + 1);
        active_inv.updated_at = Set(Utc::now().naive_utc());

        let result = active_inv.update(db).await?;
        Ok(result)
    }
}
