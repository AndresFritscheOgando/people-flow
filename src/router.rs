use axum::Router;

use crate::modules::{auth, users};
use crate::state::AppState;

pub fn build() -> Router<AppState> {
    Router::new()
        .nest("/api/auth", auth::routes::router())
        .nest("/api/users", users::routes::router())
}
