use std::sync::Arc;

use snafu::ResultExt;

use crate::{domain::user::{ports::UserRepository, services::DefaultUserService}, env::Env, infrastructure::{db::postgres::Postgres, user::PostgresUserRepository}};


pub struct AppServer<U> 
where
    U: UserRepository
{
    pub user_repository: U
}

impl AppServer<PostgresUserRepository> {
    pub async fn new(env: Arc<Env>) -> Result<Self, snafu::Whatever> {
        let postgres = Postgres::new(env.clone()).await
            .with_whatever_context(|_| "failed to create postgres instance")?;

        let user_repository = PostgresUserRepository::new(postgres.get_db());
        Ok(AppServer { user_repository })
    }
}

#[derive(Clone)]
pub struct AppState {
    pub user_service: DefaultUserService,
}

impl AppState {
    pub fn new(user_service: DefaultUserService) -> Self {
        AppState {
            user_service,
        }
    }
}
