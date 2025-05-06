use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateCheckInRequest {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub mood: Option<String>,
    pub energy_level: Option<i32>,
    pub symptoms: Vec<String>,
    pub note: Option<String>,
}
