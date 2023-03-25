//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "extras")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Decimal(Some((6, 2)))", nullable)]
    pub price: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::extras_dinner::Entity")]
    ExtrasDinner,
    #[sea_orm(has_many = "super::extras_order::Entity")]
    ExtrasOrder,
}

impl Related<super::extras_dinner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExtrasDinner.def()
    }
}

impl Related<super::extras_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExtrasOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
