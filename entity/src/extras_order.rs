//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "extras_order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_dinner_id: i32,
    pub extras_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::extras::Entity",
        from = "Column::ExtrasId",
        to = "super::extras::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Extras,
    #[sea_orm(
        belongs_to = "super::user_dinner_orders::Entity",
        from = "Column::Id",
        to = "super::user_dinner_orders::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    UserDinnerOrders,
}

impl Related<super::extras::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Extras.def()
    }
}

impl Related<super::user_dinner_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserDinnerOrders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
