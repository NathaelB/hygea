use uuid::Uuid;

use super::{entities::User, errors::UserError};

pub trait UserRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<User>, UserError>> + Send;
    fn find_by_oauth2_id(
        &self,
        oauth2_id: String,
    ) -> impl Future<Output = Result<Option<User>, UserError>> + Send;
    fn create(&self, user: User) -> impl Future<Output = Result<User, UserError>> + Send;
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<(), UserError>> + Send;
}

pub trait UserService: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<User, UserError>> + Send;
    fn find_by_oauth2_id(
        &self,
        oauth2_id: String,
    ) -> impl Future<Output = Result<User, UserError>> + Send;
}
