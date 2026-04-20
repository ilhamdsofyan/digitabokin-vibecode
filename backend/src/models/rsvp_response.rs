use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "rsvp_responses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub invitation_id: Uuid,
    pub guest_name: String,
    pub attendance_status: String,
    pub guest_count: i32,
    pub message: Option<String>,
    pub extra_data: Option<Json>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::invitation::Entity",
        from = "Column::InvitationId",
        to = "super::invitation::Column::Id"
    )]
    Invitation,
}

impl Related<super::invitation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invitation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
