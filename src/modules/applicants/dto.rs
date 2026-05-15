use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
pub struct CreateApplicantDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub status: String,
    pub applied_at: DateTimeUtc,
    pub date_of_birth: Option<NaiveDate>,
}
#[derive(Debug, Deserialize)]
pub struct UpdateApplicantDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub status: Option<String>,
    pub applied_at: Option<DateTimeUtc>,
    pub date_of_birth: Option<NaiveDate>,
}
