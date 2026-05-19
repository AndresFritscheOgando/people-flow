use sea_orm::entity::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::private::enums::project_type::ProjectType;

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub id: String,
    pub title: String,
    pub location: String,
    pub description: String,
    pub is_active: bool,
    pub applicants_counter: u32,
    pub views_counter: u32,
    pub project_type: ProjectType,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JobCreateDto {
    #[validate(length(min=1,max=100))]
    pub title: String,
    #[validate(length(min=1,max=50))]
    pub location: String,
    #[validate(length(min=10,max=10000))]
    pub description: String,
    pub is_active: bool,
    pub applicants_counter: u32,
    pub views_counter: u32,
    pub project_type: ProjectType,
}
#[derive(Debug, Deserialize, Validate)]
pub struct JobUpdateDto {
    #[validate(length(min=1,max=100))]
    pub title: Option<String>,
    #[validate(length(min=1,max=50))]
    pub location: Option<String>,
    #[validate(length(min=10,max=10000))]
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub applicants_counter: Option<u32>,
    pub views_counter: Option<u32>,
    pub project_type: Option<ProjectType>,
}

#[derive(Debug, Deserialize)]
pub struct FilterJobDto {
    pub title: Option<String>,
    pub location: Option<String>,
}
