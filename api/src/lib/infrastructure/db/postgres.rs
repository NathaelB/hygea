use std::sync::Arc;

use sea_orm::{
    ConnectOptions, Database, DatabaseConnection,
    sqlx::{self, PgPool},
};

use crate::env::Env;

#[derive(Debug, Clone)]
pub struct Postgres {
    pub pool: sqlx::PgPool,
    pub db: DatabaseConnection,
}

impl Postgres {
    pub async fn new(env: Arc<Env>) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPool::connect(&env.database_url).await?;

        let mut opt = ConnectOptions::new(env.database_url.clone());

        opt.max_connections(100).min_connections(5);

        let db = Database::connect(opt)
            .await
            .map_err(|_| sqlx::Error::Configuration("Failed to connect to database".into()))?;
        Ok(Postgres { pool, db })
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub fn get_db(&self) -> DatabaseConnection {
        self.db.clone()
    }
}
