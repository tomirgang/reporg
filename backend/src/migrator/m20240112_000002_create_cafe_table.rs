use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240112_000001_create_cafe_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cafe::Table)
                    .col(
                        ColumnDef::new(Cafe::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cafe::Location).string().not_null())
                    .col(ColumnDef::new(Cafe::Address).string().not_null())
                    .col(ColumnDef::new(Cafe::Date).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cafe::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Cafe {
    Table,
    Id,
    Location,
    Address,
    Date,
}
