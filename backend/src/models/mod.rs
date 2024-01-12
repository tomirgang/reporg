pub mod cafe;
pub mod user;

use sea_orm::*;
use sea_orm_migration::prelude::*;

pub async fn establish_connection(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(db_url).await?;

    if cfg!(test) {
        use crate::migrator::Migrator;
        Migrator::refresh(&db).await.unwrap();
    }

    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_db_connection() {        
        let _db = establish_connection("sqlite::memory:").await.unwrap();
    }
}
