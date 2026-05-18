use crate::modules::private::applicants::dto::{
    ApplicantResponse, CreateApplicantDto, UpdateApplicantDto,
};
use crate::{
    errors::{AppError, AppResult},
    modules::private::applicants,
};
use applicants::entity::Entity as ApplicantEntity;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct ApplicantService {
    db: DatabaseConnection,
}

impl ApplicantService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all_async(db: &DatabaseConnection) -> AppResult<Vec<ApplicantResponse>> {
        let applicants = ApplicantEntity::find().all(db).await?;

        Ok(applicants
            .into_iter()
            .map(|a| ApplicantResponse {
                first_name: a.first_name,
                last_name: a.last_name,
                email: a.email,
                phone_number: a.phone_number,
                status: a.status,
                applied_at: a.applied_at,
                date_of_birth: a.date_of_birth,
                created_at: a.created_at,
                updated_at: a.updated_at,
            })
            .collect())
    }
    pub async fn create_async(
        db: &DatabaseConnection,
        dto: CreateApplicantDto,
    ) -> AppResult<ApplicantResponse> {
        let now = Utc::now();
        let new_id = Uuid::new_v4();
        tracing::info!("create_async called, generated id={}", new_id);
        let applicant = applicants::entity::ActiveModel {
            id: Set(new_id),
            first_name: Set(dto.first_name),
            last_name: Set(dto.last_name),
            email: Set(dto.email),
            phone_number: Set(dto.phone_number),
            status: Set(dto.status),
            applied_at: Set(dto.applied_at),
            date_of_birth: Set(dto.date_of_birth),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let created = applicant.insert(db).await?;

        Ok(ApplicantResponse {
            first_name: created.first_name,
            last_name: created.last_name,
            email: created.email,
            phone_number: created.phone_number,
            status: created.status,
            applied_at: created.applied_at,
            date_of_birth: created.date_of_birth,
            created_at: created.created_at,
            updated_at: created.updated_at,
        })
    }

    pub async fn update_async(
        db: &DatabaseConnection,
        id: Uuid,
        dto: UpdateApplicantDto,
    ) -> AppResult<ApplicantResponse> {
        let applicant = applicants::entity::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut applicant: applicants::entity::ActiveModel = applicant.into();

        if let Some(first_name) = dto.first_name {
            applicant.first_name = Set(first_name);
        }

        if let Some(last_name) = dto.last_name {
            applicant.last_name = Set(last_name);
        }

        if let Some(email) = dto.email {
            applicant.email = Set(email);
        }

        if let Some(phone_number) = dto.phone_number {
            applicant.phone_number = Set(Some(phone_number));
        }

        if let Some(status) = dto.status {
            applicant.status = Set(status);
        }

        if let Some(applied_at) = dto.applied_at {
            applicant.applied_at = Set(applied_at);
        }

        if let Some(date_of_birth) = dto.date_of_birth {
            applicant.date_of_birth = Set(Some(date_of_birth));
        }

        applicant.updated_at = Set(Utc::now());

        let updated = applicant.update(db).await?;

        Ok(ApplicantResponse {
            first_name: updated.first_name,
            last_name: updated.last_name,
            email: updated.email,
            phone_number: updated.phone_number,
            status: updated.status,
            applied_at: updated.applied_at,
            date_of_birth: updated.date_of_birth,
            created_at: updated.created_at,
            updated_at: updated.updated_at,
        })
    }
    pub async fn delete_async(db: &DatabaseConnection, id: Uuid) -> AppResult<()> {
        let applicant = applicants::entity::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        let active: applicants::entity::ActiveModel = applicant.into();
        active.delete(db).await?;

        Ok(())
    }
}
