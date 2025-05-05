use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mood {
    Happy,
    Neutral,
    Sad,
    Tired,
    Anxious,
    Angry,
    Stressed,
}

impl std::str::FromStr for Mood {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "happy" => Ok(Mood::Happy),
            "neutral" => Ok(Mood::Neutral),
            "sad" => Ok(Mood::Sad),
            "tired" => Ok(Mood::Tired),
            "anxious" => Ok(Mood::Anxious),
            "angry" => Ok(Mood::Angry),
            "stressed" => Ok(Mood::Stressed),
            _ => Err(()),
        }
    }
}

impl ToString for Mood {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
