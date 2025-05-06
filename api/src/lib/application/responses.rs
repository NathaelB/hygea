use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use success::ApiSuccess;

pub mod error;
pub mod response_body;
pub mod success;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiErrorDetail>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorDetail {
    pub message: String,
    pub rule: String,
    pub field: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiResponseError {
    pub code: String,
    pub status: u16,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response<T: Serialize + PartialEq> {
    Ok(T),
    Created(T),
    Accepted(T),
}

impl<T: Serialize + PartialEq> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Response::Created(data) => (StatusCode::CREATED, Json(data)).into_response(),
            Response::Accepted(data) => (StatusCode::ACCEPTED, Json(data)).into_response(),
        }
    }
}

impl<T: Serialize + PartialEq> Response<T> {
    pub fn into_api_success(self) -> ApiSuccess<T> {
        match self {
            Response::Ok(data) => ApiSuccess::new(StatusCode::OK, data),
            Response::Created(data) => ApiSuccess::new(StatusCode::CREATED, data),
            Response::Accepted(data) => ApiSuccess::new(StatusCode::ACCEPTED, data),
        }
    }
}
