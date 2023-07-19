pub mod m20230718_141525_create_tables;

use async_trait::async_trait;
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230718_141525_create_tables::Migration)]
    }
}
