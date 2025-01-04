//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "auth_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth_group_permissions::Entity")]
    AuthGroupPermissions,
    #[sea_orm(has_many = "super::auth_user_groups::Entity")]
    AuthUserGroups,
}

impl Related<super::auth_group_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthGroupPermissions.def()
    }
}

impl Related<super::auth_user_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthUserGroups.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
