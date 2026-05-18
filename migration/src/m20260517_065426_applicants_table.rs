use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Applicants::Table)
                    .if_not_exists()
                    .col(pk_uuid(Applicants::Id))
                    .col(string(Applicants::FirstName))
                    .col(string(Applicants::LastName))
                    .col(string_uniq(Applicants::Email))
                    .col(string_null(Applicants::PhoneNumber))
                    .col(string(Applicants::Status))
                    .col(timestamp_with_time_zone(Applicants::AppliedAt))
                    .col(date_null(Applicants::DateOfBirth))
                    .col(timestamp_with_time_zone(Applicants::CreatedAt))
                    .col(timestamp_with_time_zone(Applicants::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Applicants::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Applicants {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    PhoneNumber,
    Status,
    AppliedAt,
    DateOfBirth,
    CreatedAt,
    UpdatedAt,
}
