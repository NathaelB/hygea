use std::sync::Arc;

use clap::Parser;
use hygea::{
    application::{
        http::{HttpServer, HttpServerConfig},
        state::{AppServer, AppState},
    },
    domain::{checkin::services::DefaultCheckinService, user::services::DefaultUserService},
    env::{AppEnv, Env},
};
use snafu::Whatever;

use tokio;

fn init_logger(env: Arc<Env>) {
    match env.env {
        AppEnv::Development => {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO) // Définir explicitement le niveau
                .init();
        }
        AppEnv::Production => {
            tracing_subscriber::fmt()
                .json()
                .with_max_level(tracing::Level::INFO)
                .with_writer(std::io::stdout)
                .init();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Whatever> {
    dotenv::dotenv().ok();

    let env = Arc::new(Env::parse());
    init_logger(Arc::clone(&env));

    let app_server = AppServer::new(env.clone()).await?;

    let user_service = DefaultUserService::new(app_server.user_repository.clone());
    let checkin_service = DefaultCheckinService::new(app_server.checkin_repository.clone());

    let app_state = AppState::new(user_service, checkin_service);

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config, app_state).await?;

    http_server.run().await?;
    Ok(())
}
