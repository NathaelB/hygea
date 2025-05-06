use axum::Router;
use axum_extra::routing::RouterExt;
use utoipa::OpenApi;

use super::handlers::create_checkin::{__path_create_checkin_handler, create_checkin_handler};
use crate::application::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_checkin_handler,
    ),
    tags(
        (name = "checkin", description = "Checkin API"),
    )
)]
pub struct CheckinApiDoc;

pub fn checkin_routes() -> Router<AppState> {
    Router::new().typed_post(create_checkin_handler)
}
