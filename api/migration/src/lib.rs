pub use sea_orm_migration::prelude::*;

mod m20250502_230938_create_user_table;
mod m20250505_120053_create_checkin;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250502_230938_create_user_table::Migration),
            Box::new(m20250505_120053_create_checkin::Migration),
        ]
    }
}
