pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230324_173454_dinners;
mod m20230324_174906_alter_user;
mod m20230324_175003_orders;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230324_173454_dinners::Migration),
            Box::new(m20230324_174906_alter_user::Migration),
            Box::new(m20230324_175003_orders::Migration),
        ]
    }
}
