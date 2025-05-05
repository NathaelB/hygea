use chrono::{DateTime, NaiveDate, Utc};
use mood::Mood;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod dto;
pub mod mood;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckIn {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,

    pub mood: Mood,
    pub energy_level: Option<i32>,
    pub symptoms: Vec<Symptom>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Symptom {
    pub id: Uuid,
    pub checkin_id: Uuid,
    pub label: String,
}

impl Symptom {
    pub fn new(checkin_id: Uuid, label: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            checkin_id,
            label,
        }
    }
}
