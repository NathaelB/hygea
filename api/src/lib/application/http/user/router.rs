use axum::Router;

use crate::application::state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
}