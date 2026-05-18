use axum::{extract::State, routing::get, Json, Router};
use uuid::Uuid;

use crate::{errors::AppResult, middleware::auth::Claims, state::AppState};

use super::{dto::UserResponse, service::UserService};

pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(get_me))
}

async fn get_me(State(state): State<AppState>, claims: Claims) -> AppResult<Json<UserResponse>> {
    let id = Uuid::parse_str(&claims.sub).map_err(|_| crate::errors::AppError::Unauthorized)?;
    let svc = UserService::new(&state.db);
    let user = svc.get_by_id(id).await?;
    Ok(Json(user))
}
