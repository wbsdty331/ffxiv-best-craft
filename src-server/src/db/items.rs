//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "Items")]
pub struct Model {
    #[sea_orm(column_name = "Id", primary_key)]
    pub id: u32,
    #[sea_orm(column_name = "Name", column_type = "custom(\"LONGTEXT\")")]
    pub name: String,
    #[sea_orm(column_name = "Level")]
    pub level: u32,
    #[sea_orm(column_name = "CanBeHQ")]
    pub can_be_hq: i8,
    #[sea_orm(column_name = "CategoryId")]
    pub category_id: Option<u32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Categories,
    #[sea_orm(has_many = "super::item_with_amount::Entity")]
    ItemWithAmount,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::item_with_amount::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemWithAmount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
