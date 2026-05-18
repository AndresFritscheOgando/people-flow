use axum::{middleware, Router};

use crate::middleware::auth::require_auth;
use crate::modules::{private, public};
use crate::state::AppState;

pub fn build(state: AppState) -> Router<AppState> {
    let private = Router::new()
        .nest("/api/users", private::users::routes::router())
        .nest("/api/applicants", private::applicants::routes::router())
        .route_layer(middleware::from_fn_with_state(state, require_auth));

    Router::new()
        .nest("/api/auth", public::auth::routes::router())
        .merge(private)
}
