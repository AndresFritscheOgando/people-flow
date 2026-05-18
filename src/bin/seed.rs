use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

// Re-use the workspace crate's db + entity modules
use people_flow::{
    config::Config,
    db,
    modules::private::users::entity::{ActiveModel, Column, Entity},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;
    let db = db::connect(&config.database_url).await?;

    let admin_email =
        std::env::var("SEED_ADMIN_EMAIL").unwrap_or_else(|_| "admin@peopleflow.dev".into());
    let admin_password =
        std::env::var("SEED_ADMIN_PASSWORD").unwrap_or_else(|_| "Admin1234!".into());
    let admin_name =
        std::env::var("SEED_ADMIN_NAME").unwrap_or_else(|_| "Admin".into());

    let existing = Entity::find()
        .filter(Column::Email.eq(&admin_email))
        .one(&db)
        .await?;

    if existing.is_some() {
        println!("Admin user already exists: {admin_email}");
        return Ok(());
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(admin_password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| anyhow::anyhow!("hash failed: {e}"))?;

    let now = Utc::now();
    let user = ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(admin_email.clone()),
        password_hash: Set(password_hash),
        name: Set(admin_name),
        created_at: Set(now),
        updated_at: Set(now),
    };

    user.insert(&db).await?;
    println!("Admin user created: {admin_email}");

    Ok(())
}
