use uuid::Uuid;

use super::{
    entities::{CheckIn, dto::CreateCheckInRequest},
    errors::CheckinError,
};

pub trait CheckinRepository: Send + Sync {
    fn create(
        &self,
        checkin: CheckIn,
    ) -> impl Future<Output = Result<CheckIn, CheckinError>> + Send;
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<CheckIn>, CheckinError>> + Send;
    fn find_by_user_id(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Vec<CheckIn>, CheckinError>> + Send;
}

pub trait CheckinService: Send + Sync {
    fn create(
        &self,
        dto: CreateCheckInRequest,
    ) -> impl Future<Output = Result<CheckIn, CheckinError>> + Send;
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<CheckIn, CheckinError>> + Send;
    fn find_by_user_id(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Vec<CheckIn>, CheckinError>> + Send;
}
