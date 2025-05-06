use uuid::Uuid;

use crate::infrastructure::checkin::PostgresCheckinRepository;

use super::{
    entities::{CheckIn, dto::CreateCheckInRequest},
    errors::CheckinError,
    ports::{CheckinRepository, CheckinService},
};

pub type DefaultCheckinService = CheckinServiceImpl<PostgresCheckinRepository>;

#[derive(Clone)]
pub struct CheckinServiceImpl<C>
where
    C: CheckinRepository,
{
    pub checkin_repository: C,
}

impl<C> CheckinServiceImpl<C>
where
    C: CheckinRepository,
{
    pub fn new(checkin_repository: C) -> Self {
        Self { checkin_repository }
    }
}

impl<C> CheckinService for CheckinServiceImpl<C>
where
    C: CheckinRepository,
{
    async fn create(&self, dto: CreateCheckInRequest) -> Result<CheckIn, CheckinError> {
        let symptoms = dto.symptoms.clone();
        let checkin: CheckIn = dto.into();
        self.checkin_repository.create(checkin, symptoms).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<CheckIn, CheckinError> {
        self.checkin_repository
            .find_by_id(id)
            .await?
            .ok_or(CheckinError::NotFound)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<CheckIn>, CheckinError> {
        self.checkin_repository
            .find_by_user_id(user_id)
            .await
            .map_err(|_| CheckinError::NotFound)
    }
}
