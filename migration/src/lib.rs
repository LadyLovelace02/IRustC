pub use sea_orm_migration::prelude::*;

mod m20220716_182324_initial_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220716_182324_initial_tables::Migration)]
    }
}
