//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "auth_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub content_type_id: i32,
    pub codename: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth_group_permissions::Entity")]
    AuthGroupPermissions,
    #[sea_orm(has_many = "super::auth_user_user_permissions::Entity")]
    AuthUserUserPermissions,
    #[sea_orm(
        belongs_to = "super::django_content_type::Entity",
        from = "Column::ContentTypeId",
        to = "super::django_content_type::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    DjangoContentType,
}

impl Related<super::auth_group_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthGroupPermissions.def()
    }
}

impl Related<super::auth_user_user_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthUserUserPermissions.def()
    }
}

impl Related<super::django_content_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DjangoContentType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
