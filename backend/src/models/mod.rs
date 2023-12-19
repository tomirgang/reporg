pub mod cafe;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn establish_connection() -> SqliteConnection {
    if cfg!(test) {
        let mut conn = SqliteConnection::establish(":memory:")
            .unwrap_or_else(|_| panic!("Error creating test database"));

        let _result = run_migrations(&mut conn);
        conn
    } else {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_db_connection() {
        use crate::models::cafe::Cafe;
        use crate::models::schema::cafe::dsl::*;

        let mut connection = establish_connection();
        cafe.limit(1)
            .select(Cafe::as_select())
            .load(&mut connection)
            .expect("Error loading cafes.");
    }
}
