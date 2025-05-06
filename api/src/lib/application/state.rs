use std::sync::Arc;

use snafu::ResultExt;

use crate::{
    domain::{
        checkin::{ports::CheckinRepository, services::DefaultCheckinService},
        user::{ports::UserRepository, services::DefaultUserService},
    },
    env::Env,
    infrastructure::{
        checkin::PostgresCheckinRepository, db::postgres::Postgres, user::PostgresUserRepository,
    },
};

pub struct AppServer<U, C>
where
    U: UserRepository,
    C: CheckinRepository,
{
    pub user_repository: U,
    pub checkin_repository: C,
}

impl AppServer<PostgresUserRepository, PostgresCheckinRepository> {
    pub async fn new(env: Arc<Env>) -> Result<Self, snafu::Whatever> {
        let postgres = Postgres::new(env.clone())
            .await
            .with_whatever_context(|_| "failed to create postgres instance")?;

        let user_repository = PostgresUserRepository::new(postgres.get_db());
        let checkin_repository = PostgresCheckinRepository::new(postgres.get_db());

        Ok(AppServer {
            user_repository,
            checkin_repository,
        })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub user_service: DefaultUserService,
    pub checkin_service: DefaultCheckinService,
}

impl AppState {
    pub fn new(user_service: DefaultUserService, checkin_service: DefaultCheckinService) -> Self {
        AppState {
            user_service,
            checkin_service,
        }
    }
}
