use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Applicant::Table)
                    .if_not_exists()
                    .col(pk_uuid(Applicant::Id))
                    .col(string(Applicant::FirstName))
                    .col(string(Applicant::LastName))
                    .col(string_uniq(Applicant::Email))
                    .col(string_null(Applicant::PhoneNumber))
                    .col(string(Applicant::Status))
                    .col(timestamp_with_time_zone(Applicant::AppliedAt))
                    .col(date_null(Applicant::DateOfBirth))
                    .col(timestamp_with_time_zone(Applicant::CreatedAt))
                    .col(timestamp_with_time_zone(Applicant::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Applicant::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Applicant {
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
