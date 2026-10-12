pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_accounts;
mod m20260222_000002_create_sessions;
mod m20260222_000003_create_movies;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_accounts::Migration),
            Box::new(m20260222_000002_create_sessions::Migration),
            Box::new(m20260222_000003_create_movies::Migration),
        ]
    }
}
