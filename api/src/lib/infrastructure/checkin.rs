use std::str::FromStr;

use crate::domain::checkin::entities::Symptom;
use crate::domain::checkin::entities::mood::Mood;
use crate::domain::checkin::{entities::CheckIn, errors::CheckinError, ports::CheckinRepository};
use entity::checkin::{ActiveModel, Entity as CheckinEntity};
use entity::prelude::Symptom as SymptomEntity;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use tracing::error;

impl From<entity::checkin::Model> for CheckIn {
    fn from(model: entity::checkin::Model) -> Self {
        let mood = model.mood.unwrap();

        CheckIn {
            id: model.id,
            user_id: model.user_id,
            date: model.date,
            energy_level: model.energy_level,
            mood: Mood::from_str(mood.as_str()).unwrap(),
            symptoms: vec![],
            note: model.note,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

impl From<entity::symptom::Model> for Symptom {
    fn from(model: entity::symptom::Model) -> Self {
        Symptom {
            id: model.id,
            checkin_id: model.checkin_id,
            label: model.label,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostgresCheckinRepository {
    pub db: DatabaseConnection,
}

impl PostgresCheckinRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl CheckinRepository for PostgresCheckinRepository {
    async fn create(&self, checkin: CheckIn) -> Result<CheckIn, CheckinError> {
        let new_checkin = ActiveModel {
            id: Set(checkin.id),
            user_id: Set(checkin.user_id),
            date: Set(checkin.date),
            energy_level: Set(checkin.energy_level),
            mood: Set(Some(checkin.mood.to_string())),
            note: Set(checkin.note),
            created_at: Set(checkin.created_at.into()),
            updated_at: Set(checkin.updated_at.into()),
        };

        let insert_result = CheckinEntity::insert(new_checkin)
            .exec(&self.db)
            .await
            .map_err(|_| CheckinError::CreateError)?;

        for symptom in checkin.symptoms {
            let t = Symptom::new(insert_result.last_insert_id, symptom.label);
            let new_symptom = entity::symptom::ActiveModel {
                checkin_id: Set(t.checkin_id),
                id: Set(t.id),
                label: Set(t.label),
            };

            let _ = entity::symptom::Entity::insert(new_symptom)
                .exec(&self.db)
                .await
                .map_err(|_| CheckinError::CreateError)?;
        }
        todo!("Implement create method");
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<CheckIn>, CheckinError> {
        let checkin_model =
            match CheckinEntity::find_by_id(id)
                .one(&self.db)
                .await
                .map_err(|e| {
                    error!("Database error when finding checkin by ID {}: {}", id, e);
                    CheckinError::NotFound
                })? {
                Some(model) => model,
                None => return Ok(None),
            };

        let symptoms = checkin_model
            .find_related(SymptomEntity)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!(
                    "Database error when finding symptoms for checkin {}: {}",
                    id, e
                );
                CheckinError::NotFound
            })?
            .into_iter()
            .map(Symptom::from)
            .collect();

        // Create CheckIn entity with symptoms included
        let mut checkin = CheckIn::from(checkin_model);
        checkin.symptoms = symptoms;

        Ok(Some(checkin))
    }

    async fn find_by_user_id(&self, user_id: uuid::Uuid) -> Result<Vec<CheckIn>, CheckinError> {
        let checkins = CheckinEntity::find()
            .filter(entity::checkin::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|_| CheckinError::NotFound)?;

        let mut result = Vec::new();
        for checkin_model in checkins {
            let symptoms = checkin_model
                .find_related(SymptomEntity)
                .all(&self.db)
                .await
                .map_err(|_| CheckinError::NotFound)?
                .into_iter()
                .map(Symptom::from)
                .collect();

            let mut checkin = CheckIn::from(checkin_model);
            checkin.symptoms = symptoms;
            result.push(checkin);
        }

        Ok(result)
    }
}
