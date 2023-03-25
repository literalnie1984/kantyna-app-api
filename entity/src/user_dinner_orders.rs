//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_dinner_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub dinner_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dinner::Entity",
        from = "Column::DinnerId",
        to = "super::dinner::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Dinner,
    #[sea_orm(
        belongs_to = "super::dinner_orders::Entity",
        from = "Column::OrderId",
        to = "super::dinner_orders::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    DinnerOrders,
    #[sea_orm(has_many = "super::extras_order::Entity")]
    ExtrasOrder,
}

impl Related<super::dinner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dinner.def()
    }
}

impl Related<super::dinner_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DinnerOrders.def()
    }
}

impl Related<super::extras_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExtrasOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}