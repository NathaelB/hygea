use uuid::Uuid;

use crate::infrastructure::user::PostgresUserRepository;

use super::{
    entities::User,
    errors::UserError,
    ports::{UserRepository, UserService},
};

pub type DefaultUserService = UserServiceImpl<PostgresUserRepository>;

#[derive(Clone)]
pub struct UserServiceImpl<U>
where
    U: UserRepository,
{
    pub user_repository: U,
}

impl<U> UserServiceImpl<U>
where
    U: UserRepository,
{
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }
}

impl<U> UserService for UserServiceImpl<U>
where
    U: UserRepository,
{
    async fn find_by_id(&self, id: Uuid) -> Result<User, UserError> {
        self.user_repository
            .find_by_id(id)
            .await?
            .ok_or(UserError::NotFound)
    }

    async fn find_by_oauth2_id(&self, oauth2_id: String) -> Result<User, UserError> {
        self.user_repository
            .find_by_oauth2_id(oauth2_id)
            .await?
            .ok_or(UserError::NotFound)
    }
}
