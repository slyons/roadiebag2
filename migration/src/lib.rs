#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20240123_152813_items;
mod m20240123_154524_taken_items;
mod m20240129_160019_add_taken_starting_rounds;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20240123_152813_items::Migration),
            Box::new(m20240123_154524_taken_items::Migration),
            Box::new(m20240129_160019_add_taken_starting_rounds::Migration),
        ]
    }
}