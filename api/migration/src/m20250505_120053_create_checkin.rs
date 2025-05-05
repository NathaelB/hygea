use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250502_230938_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Checkin::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Checkin::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Checkin::UserId).uuid().not_null())
                    .col(ColumnDef::new(Checkin::Date).date().not_null())
                    .col(ColumnDef::new(Checkin::Mood).string().null())
                    .col(ColumnDef::new(Checkin::EnergyLevel).integer().null())
                    .col(ColumnDef::new(Checkin::Note).text().null())
                    .col(
                        ColumnDef::new(Checkin::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Checkin::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Checkin::Table, Checkin::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Symptom::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Symptom::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Symptom::CheckinId).uuid().not_null())
                    .col(ColumnDef::new(Symptom::Label).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Symptom::Table, Symptom::CheckinId)
                            .to(Checkin::Table, Checkin::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Symptom::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Checkin::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Checkin {
    Table,
    Id,
    UserId,
    Date,
    Mood,
    EnergyLevel,
    Symptoms,
    Note,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Symptom {
    Table,
    Id,
    CheckinId,
    Label,
}
