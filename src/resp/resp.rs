use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OkResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> Default for OkResponse<T> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<T> OkResponse<T> {
    fn new(data: Option<T>) -> Self {
        Self { data }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrResponse {
    code: u16,
    message: String,
}

impl IntoResponse for ErrResponse {
    fn into_response(self) -> axum::response::Response {
        let sc = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (sc, Json(self)).into_response()
    }
}

impl ErrResponse {
    // pub(crate) fn send() -> Response {
    //     // return
    // }
}
