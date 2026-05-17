use crate::{
    errors::{AppResult},
    modules::applicants::{
        dto::{ApplicantResponse, CreateApplicantDto, UpdateApplicantDto},
        service::ApplicantService,
    },
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn get_all(State(state): State<AppState>) -> AppResult<Json<Vec<ApplicantResponse>>> {
    let applicants = ApplicantService::get_all_async(&state.db).await?;

    Ok(Json(applicants))
}

pub async fn create_async(
    State(state): State<AppState>,
    Json(dto): Json<CreateApplicantDto>,
) -> AppResult<Json<ApplicantResponse>> {
    let applicant = ApplicantService::create_async(&state.db, dto).await?;

    Ok(Json(applicant))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateApplicantDto>,
) -> AppResult<Json<ApplicantResponse>> {
    let applicant = ApplicantService::update_async(&state.db, id, dto).await?;

    Ok(Json(applicant))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> AppResult<StatusCode> {
    ApplicantService::delete_async(&state.db, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
