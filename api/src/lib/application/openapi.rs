use utoipa::OpenApi;
use crate::application::http::checkin::router::CheckinApiDoc;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Hygea API",
    ),
    nest(
        (path = "/users/{user_id}/checkin", api = CheckinApiDoc)
    )
)]
pub struct ApiDoc;
