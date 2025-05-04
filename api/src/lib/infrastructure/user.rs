use crate::domain::user::entities::{DiabetesType, Gender};
use crate::domain::user::{entities::User, errors::UserError, ports::UserRepository};
use chrono::{TimeZone, Utc};
use entity::user::{self, ActiveModel, Entity as UserEntity};
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

impl From<entity::user::Model> for User {
    fn from(model: entity::user::Model) -> Self {
        User {
            id: model.id,
            name: model.name,
            email: model.email,
            gender: Gender::Male,
            height_cm: model.height_cm,
            weight_kg: model.weight_kg,
            diabetes_type: DiabetesType::Type1,
            oauth2_id: model.oauth2_id,
            created_at: Utc.from_utc_datetime(&model.created_at),
            updated_at: Utc.from_utc_datetime(&model.updated_at),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostgresUserRepository {
    pub db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        PostgresUserRepository { db }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, UserError> {
        let user = UserEntity::find()
            .filter(user::Column::Id.eq(id))
            .one(&self.db.clone())
            .await
            .map_err(|_| UserError::NotFound)?
            .map(|model| User::from(model));

        Ok(user)
    }

    async fn find_by_oauth2_id(&self, oauth2_id: String) -> Result<Option<User>, UserError> {
        let user = UserEntity::find()
            .filter(user::Column::Oauth2Id.eq(oauth2_id))
            .one(&self.db.clone())
            .await
            .map_err(|_| UserError::NotFound)?
            .map(|model| User::from(model));

        Ok(user)
    }

    async fn create(&self, user: User) -> Result<User, UserError> {
        let new_user = ActiveModel {
            id: Set(user.id),
            oauth2_id: Set(user.oauth2_id),
            name: Set(user.name),
            email: Set(user.email),
            diabetes_type: Set(user.diabetes_type.into()),
            gender: Set(user.gender.into()),
            height_cm: Set(user.height_cm),
            weight_kg: Set(user.weight_kg),
            created_at: Set(user.created_at.naive_utc()),
            updated_at: Set(user.updated_at.naive_utc()),
            ..Default::default()
        };

        let insert_result = UserEntity::insert(new_user)
            .exec(&self.db)
            .await
            .map_err(|_| UserError::NotFound)?;

        let user = UserEntity::find()
            .filter(user::Column::Id.eq(insert_result.last_insert_id))
            .one(&self.db)
            .await
            .map_err(|_| UserError::NotFound)?
            .map(|model| User::from(model))
            .ok_or(UserError::NotFound)?;

        Ok(user)
    }

    async fn delete_by_id(&self, id: uuid::Uuid) -> Result<(), UserError> {
        let delete_result = UserEntity::delete_many()
            .filter(user::Column::Id.eq(id))
            .exec(&self.db)
            .await
            .map_err(|_| UserError::NotFound)?;

        if delete_result.rows_affected == 0 {
            return Err(UserError::NotFound);
        }
        Ok(())
    }
}
