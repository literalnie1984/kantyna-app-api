//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub email: String,
    pub username: String,
    pub password: String,
    pub balance: i32,
    pub verified: i8,
    pub admin: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::dinner_orders::Entity")]
    DinnerOrders,
}

impl Related<super::dinner_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DinnerOrders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
