use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(uuid(User::Id).primary_key())
                    .col(
                        ColumnDef::new(User::Oauth2Id)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(string(User::Name))
                    .col(string(User::Email))
                    .col(integer(User::Gender))
                    .col(float(User::HeightCm))
                    .col(float(User::WeightKg))
                    .col(integer(User::DiabetesType))
                    .col(timestamp(User::CreatedAt))
                    .col(timestamp(User::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    #[sea_orm(primary_key)]
    Id,
    #[sea_orm(unique)]
    Oauth2Id,
    Name,
    Email,
    Gender,
    HeightCm,
    WeightKg,
    DiabetesType,
    CreatedAt,
    UpdatedAt,
}
