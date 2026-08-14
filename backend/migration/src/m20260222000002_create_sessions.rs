use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Sessions::Table)
                    .col(string_len(Sessions::Token, 64).not_null().primary_key())
                    .col(unsigned(Sessions::AccountId).not_null())
                    .col(date_time(Sessions::Expiration).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_account_id")
                            .from(Sessions::Table, Sessions::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_sessions_account_id")
                            .col(Sessions::AccountId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Token,
    AccountId,
    Expiration,
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}
