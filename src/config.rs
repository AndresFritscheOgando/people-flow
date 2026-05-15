use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expires_in: i64,  // seconds
    pub jwt_refresh_expires_in: i64, // seconds
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL missing")?,
            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET missing")?,
            jwt_access_expires_in: std::env::var("JWT_ACCESS_EXPIRES_IN")
                .unwrap_or_else(|_| "900".into())
                .parse()
                .context("JWT_ACCESS_EXPIRES_IN must be a number")?,
            jwt_refresh_expires_in: std::env::var("JWT_REFRESH_EXPIRES_IN")
                .unwrap_or_else(|_| "604800".into())
                .parse()
                .context("JWT_REFRESH_EXPIRES_IN must be a number")?,
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .context("SERVER_PORT must be a number")?,
        })
    }
}
