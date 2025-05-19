use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Card, Hand};

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default)]
    pub offset: usize,
}

#[derive(Serialize)]
pub struct CreateDeckResponse {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct ListHandsResponse {
    pub hand: Hand,
    pub next_offset: Option<usize>,
}

#[derive(Deserialize)]
pub struct CompareHandsRequest {
    pub hands: Vec<HandDto>,
}

#[derive(Serialize)]
pub struct CompareHandsResponse {
    pub winners: Vec<HandDto>,
}

#[derive(Serialize)]
pub struct HistoryResponse {
    pub items: Vec<HistoryItem>,
    pub next_offset: Option<usize>,
}

#[derive(Serialize)]
pub struct HistoryItem {
    pub deck: Uuid,
    pub offset: i64,
    pub time: u64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct HandDto {
    pub external_id: String,
    pub hand: [Card; 5],
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiError {
    UserInput { description: String },
    InternalServer,
    JsonParsing { description: String },
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonParsing {
            description: rejection.body_text(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::UserInput { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::JsonParsing { .. } => StatusCode::BAD_REQUEST,
        };

        (status, axum::Json(self)).into_response()
    }
}
