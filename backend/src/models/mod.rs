pub mod cafe;
pub mod user;
pub mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use r2d2::Pool;
use std::env;
use std::error::Error;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(
    connection: &mut SqliteConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn establish_connection() -> DbPool {
    let database_url = if cfg!(test) {
        String::from(":memory:")
    } else {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    match run_migrations(&mut pool.get().unwrap()) {
        Ok(_) => {}
        Err(e) => panic!("Migrations failed: {}", e),
    }

    pool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_db_connection() {
        use crate::models::cafe::Cafe;
        use crate::models::schema::cafe::dsl::*;
        use diesel::prelude::*;

        let pool = establish_connection();
        cafe.limit(1)
            .select(Cafe::as_select())
            .load(&mut pool.get().unwrap())
            .expect("Error loading cafes.");
    }
}
