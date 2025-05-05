use axum::Router;

use crate::application::state::AppState;

pub fn checkin_routes() -> Router<AppState> {
    Router::new()
}
