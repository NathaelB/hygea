use crate::{
    application::responses::error::{ApiError, ValidationError},
    domain::checkin::errors::CheckinError,
};

impl From<CheckinError> for ApiError {
    fn from(value: CheckinError) -> Self {
        match value {
            CheckinError::NotFound => ApiError::NotFound("Checkin not found".to_string()),
            CheckinError::AlreadyExists => ApiError::UnProcessableEntity(vec![ValidationError {
                message: "Checkin already exists".to_string(),
                field: "checkin".to_string(),
            }]),
            CheckinError::CreateError => {
                ApiError::InternalServerError("Failed to create checkin".to_string())
            }
            CheckinError::UpdateError => {
                ApiError::InternalServerError("Failed to update checkin".to_string())
            }
            CheckinError::DeleteError => {
                ApiError::InternalServerError("Failed to delete checkin".to_string())
            }
            CheckinError::InvalidData => {
                ApiError::InternalServerError("Invalid checkin data".to_string())
            }
        }
    }
}
