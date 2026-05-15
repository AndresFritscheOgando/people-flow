use serde::{Deserialize, Serialize};
use validator::Validate;

use super::entity::Model;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: String,
}

impl From<Model> for UserResponse {
    fn from(m: Model) -> Self {
        Self {
            id: m.id.to_string(),
            email: m.email,
            name: m.name,
            created_at: m.created_at.to_rfc3339(),
        }
    }
}
