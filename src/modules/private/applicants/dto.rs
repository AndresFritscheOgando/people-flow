use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct ApplicantResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub status: String,
    pub applied_at: DateTimeUtc,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateApplicantDto {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(max = 20))]
    pub phone_number: Option<String>,
    #[validate(length(min = 1))]
    pub status: String,
    pub applied_at: DateTimeUtc,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateApplicantDto {
    #[validate(length(min = 1, max = 100))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 20))]
    pub phone_number: Option<String>,
    #[validate(length(min = 1))]
    pub status: Option<String>,
    pub applied_at: Option<DateTimeUtc>,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct FilterApplicantDto {
    pub status: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
