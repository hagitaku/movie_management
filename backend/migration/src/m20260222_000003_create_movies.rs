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
                    .table(Movies::Table)
                    .col(
                        ColumnDef::new(Movies::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string_len(Movies::Title, 255).not_null())
                    .col(string_len(Movies::Description, 500).not_null())
                    .col(string_len(Movies::Memo, 500).not_null())
                    .col(unsigned(Movies::CreatedUserId).not_null())
                    .col(date_time(Movies::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_movies_created_user_id")
                            .from(Movies::Table, Movies::CreatedUserId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .index(
                        Index::create()
                            .name("idx_movies_created_user_id")
                            .col(Movies::CreatedUserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Movies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Movies {
    Table,
    Id,
    Title,
    Description,
    Memo,
    CreatedUserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}
