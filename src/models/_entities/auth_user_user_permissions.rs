//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "auth_user_user_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i32,
    pub permission_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::auth_permission::Entity",
        from = "Column::PermissionId",
        to = "super::auth_permission::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AuthPermission,
    #[sea_orm(
        belongs_to = "super::auth_user::Entity",
        from = "Column::UserId",
        to = "super::auth_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AuthUser,
}

impl Related<super::auth_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthPermission.def()
    }
}

impl Related<super::auth_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
