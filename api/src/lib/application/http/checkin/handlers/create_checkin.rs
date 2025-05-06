use axum::extract::State;
use axum_macros::TypedPath;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::{
        http::checkin::validators::{CheckinRequestWrapper, CreateCheckinValidator},
        responses::{
            Response,
            error::{ApiError, ValidateJson},
        },
        state::AppState,
    },
    domain::checkin::{
        entities::{CheckIn, dto::CreateCheckInRequest},
        ports::CheckinService,
    },
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/users/{user_id}/checkin")]
pub struct CreateCheckinHandler {
    pub user_id: Uuid,
}

#[derive(Serialize, PartialEq)]
pub struct CreateCheckinResponse {
    pub data: CheckIn,
}

#[utoipa::path(
    post,
    path = "",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
    ),
    tag = "checkin",
    request_body = CreateCheckInRequest,
)]
pub async fn create_checkin_handler(
    CreateCheckinHandler { user_id }: CreateCheckinHandler,
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<CreateCheckinValidator>,
) -> Result<Response<CreateCheckinResponse>, ApiError> {
    state
        .checkin_service
        .create(
            CheckinRequestWrapper {
                user_id,
                checkin: payload,
            }
            .into(),
        )
        .await
        .map_err(ApiError::from)
        .map(|checkin| Response::Created(CreateCheckinResponse { data: checkin }))
}
