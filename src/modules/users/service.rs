use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};

use super::dto::UserResponse;
use super::entity::Entity;

pub struct UserService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: Uuid) -> AppResult<UserResponse> {
        let user = Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(UserResponse::from(user))
    }
}
