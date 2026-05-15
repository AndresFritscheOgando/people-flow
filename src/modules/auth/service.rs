use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    config::Config,
    errors::{AppError, AppResult},
    middleware::auth::Claims,
    modules::users::entity::{ActiveModel, Column, Entity, Model},
};

use super::dto::{AuthResponse, LoginRequest, RegisterRequest};

pub struct AuthService<'a> {
    db: &'a DatabaseConnection,
    config: &'a Arc<Config>,
}

impl<'a> AuthService<'a> {
    pub fn new(db: &'a DatabaseConnection, config: &'a Arc<Config>) -> Self {
        Self { db, config }
    }

    pub async fn register(&self, req: RegisterRequest) -> AppResult<AuthResponse> {
        let existing = Entity::find()
            .filter(Column::Email.eq(&req.email))
            .one(self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("email already registered".into()));
        }

        let password_hash = hash_password(&req.password)?;
        let id = Uuid::new_v4();

        let user = ActiveModel {
            id: Set(id),
            email: Set(req.email.clone()),
            password_hash: Set(password_hash),
            name: Set(req.name),
            ..Default::default()
        };

        user.insert(self.db).await?;

        let access_token = self.make_access_token(id.to_string(), req.email)?;
        let refresh_token = generate_refresh_token();

        Ok(AuthResponse::new(access_token, refresh_token))
    }

    pub async fn login(&self, req: LoginRequest) -> AppResult<AuthResponse> {
        let user: Model = Entity::find()
            .filter(Column::Email.eq(&req.email))
            .one(self.db)
            .await?
            .ok_or(AppError::Unauthorized)?;

        verify_password(&req.password, &user.password_hash)?;

        let access_token = self.make_access_token(user.id.to_string(), user.email)?;
        let refresh_token = generate_refresh_token();

        Ok(AuthResponse::new(access_token, refresh_token))
    }

    fn make_access_token(&self, sub: String, email: String) -> AppResult<String> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub,
            email,
            iat: now,
            exp: now + self.config.jwt_access_expires_in,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.into()))
    }
}

fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("password hash failed: {e}")))
}

fn verify_password(password: &str, hash: &str) -> AppResult<()> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid password hash: {e}")))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized)
}

fn generate_refresh_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}
