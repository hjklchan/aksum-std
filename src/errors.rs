use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0} not found")]
    NotFound(&'static str),
    #[error("route not found")]
    RouteNotFound,
    #[error("{0} already exist")]
    AlreadyExist(&'static str),

    #[error("{0}")]
    Sqlxxx(#[from] sqlx::Error),

    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        match *self {
            // 4xx related
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::RouteNotFound => StatusCode::NOT_FOUND,
            Self::AlreadyExist(_) => StatusCode::BAD_REQUEST,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Authenticate(AuthenticateError::Locked(_)) => StatusCode::LOCKED,
            Self::Authenticate(AuthenticateError::InvalidToken) => StatusCode::BAD_REQUEST,

            // 5xx related
            Self::Sqlxxx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Authenticate(AuthenticateError::GenerateToken) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    pub fn bad_request(reason: String) -> Self {
        Self::BadRequest(reason)
    }

    pub fn not_found(resource_name: &'static str) -> Self {
        if resource_name.is_empty() {
            return Self::NotFound("resource");
        }

        Self::NotFound(resource_name)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.get_status_code();
        let message = self.to_string();
        let body = Json(serde_json::json!({"code": "TODO", "message": message}));
        (status_code, body).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum AuthenticateError {
    #[error("failed to generate authentication token")]
    GenerateToken,

    #[error("invalid token")]
    InvalidToken,

    /// Users need to know why they are locked
    /// Fields with an index of 0 can provide a reason for locking
    #[error("{0}")]
    Locked(String),
}
