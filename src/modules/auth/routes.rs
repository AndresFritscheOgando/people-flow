use axum::{extract::State, routing::post, Json, Router};
use validator::Validate;

use crate::{
    errors::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{AuthResponse, LoginRequest, RegisterRequest},
    service::AuthService,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let svc = AuthService::new(&state.db, &state.config);
    let res = svc.register(body).await?;
    Ok(Json(res))
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let svc = AuthService::new(&state.db, &state.config);
    let res = svc.login(body).await?;
    Ok(Json(res))
}
