use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCheckInRequest {
    pub date: NaiveDate,
    pub mood: Option<String>,
    pub energy_level: Option<u8>,
    pub symptoms: Vec<String>,
    pub note: Option<String>,
}
