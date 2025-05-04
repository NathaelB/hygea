use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Other = 2,
}

#[derive(Debug, Clone)]
pub enum DiabetesType {
    None = 0,
    Type1 = 1,
    Type2 = 2,
    Gestational = 3,
}

impl From<DiabetesType> for i32 {
    fn from(value: DiabetesType) -> Self {
        match value {
            DiabetesType::None => 0,
            DiabetesType::Type1 => 1,
            DiabetesType::Type2 => 2,
            DiabetesType::Gestational => 3,
        }
    }
}

impl From<i32> for DiabetesType {
    fn from(value: i32) -> Self {
        match value {
            0 => DiabetesType::None,
            1 => DiabetesType::Type1,
            2 => DiabetesType::Type2,
            3 => DiabetesType::Gestational,
            _ => panic!("Invalid value for DiabetesType"),
        }
    }
}

impl From<Gender> for i32 {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => 0,
            Gender::Female => 1,
            Gender::Other => 2,
        }
    }
}

impl From<i32> for Gender {
    fn from(value: i32) -> Self {
        match value {
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::Other,
            _ => panic!("Invalid value for Gender"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub oauth2_id: String,
    pub name: String,
    pub email: String,
    pub gender: Gender,
    pub height_cm: f32,
    pub weight_kg: f32,
    pub diabetes_type: DiabetesType,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
