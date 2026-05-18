use crate::{
    errors::{AppError, AppResult},
    modules::private::applicants::{
        dto::{ApplicantResponse, CreateApplicantDto, FilterApplicantDto, UpdateApplicantDto},
        service::ApplicantService,
    },
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use uuid::Uuid;
use validator::Validate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all).post(create_async))
        .route("/:id", put(update).delete(delete_async))
}

pub async fn get_all(
    State(state): State<AppState>,
    Query(filter): Query<FilterApplicantDto>,
) -> AppResult<Json<Vec<ApplicantResponse>>> {
    let applicants = ApplicantService::get_all_async(&state.db, filter).await?;

    Ok(Json(applicants))
}

pub async fn create_async(
    State(state): State<AppState>,
    Json(dto): Json<CreateApplicantDto>,
) -> AppResult<Json<ApplicantResponse>> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let applicant = ApplicantService::create_async(&state.db, dto).await?;

    Ok(Json(applicant))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateApplicantDto>,
) -> AppResult<Json<ApplicantResponse>> {
    dto.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let applicant = ApplicantService::update_async(&state.db, id, dto).await?;

    Ok(Json(applicant))
}

pub async fn delete_async(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    ApplicantService::delete_async(&state.db, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
