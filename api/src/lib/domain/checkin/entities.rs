use std::{str::FromStr, vec};

use chrono::{DateTime, NaiveDate, Utc};
use dto::CreateCheckInRequest;
use mood::Mood;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod dto;
pub mod mood;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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

impl From<CreateCheckInRequest> for CheckIn {
    fn from(dto: CreateCheckInRequest) -> Self {
        let mood = Mood::from_str(dto.mood.unwrap().as_str()).unwrap();
        Self {
            id: Uuid::new_v4(),
            user_id: dto.user_id,
            date: dto.date,
            mood: mood,
            energy_level: dto.energy_level,
            symptoms: vec![],
            note: dto.note,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
