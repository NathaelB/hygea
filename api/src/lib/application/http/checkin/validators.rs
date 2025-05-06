use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::domain::checkin::entities::dto::CreateCheckInRequest;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCheckinValidator {
    pub date: NaiveDate,
    pub mood: Option<String>,
    pub energy_level: Option<i32>,
    pub symptoms: Vec<String>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CheckinRequestWrapper {
    pub user_id: Uuid,
    pub checkin: CreateCheckinValidator,
}

impl From<CheckinRequestWrapper> for CreateCheckInRequest {
    fn from(wrapper: CheckinRequestWrapper) -> Self {
        CreateCheckInRequest {
            user_id: wrapper.user_id,
            date: wrapper.checkin.date,
            mood: wrapper.checkin.mood,
            energy_level: wrapper.checkin.energy_level,
            symptoms: wrapper.checkin.symptoms,
            note: wrapper.checkin.note,
        }
    }
}
