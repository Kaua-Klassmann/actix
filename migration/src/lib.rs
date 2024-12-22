pub use sea_orm_migration::prelude::*;

mod m20241219_174638_create_user_table;
mod m20241222_003656_create_post_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241219_174638_create_user_table::Migration),
            Box::new(m20241222_003656_create_post_table::Migration),
        ]
    }
}
