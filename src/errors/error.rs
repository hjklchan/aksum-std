use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // Database error
    #[error("an error occurred with the database, error: {0}")]
    DatabaseError(String),

    // User error
    #[error("user not found")]
    UserNotFound,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("invalid password")]
    UserInvalidPassword,

    // Auth error
    #[error("invalid token")]
    InvalidToken,
    #[error("token has expired")]
    TokenExpired,
    #[error("missing bearer token")]
    MissingToken,
    #[error("generate token error: {0}")]
    GenTokenError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // matches
        let c = match self {
            // Database error
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            // User error
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::UserAlreadyExists => StatusCode::BAD_REQUEST,
            Self::UserInvalidPassword => StatusCode::BAD_REQUEST,

            // Token error
            Self::InvalidToken | Self::TokenExpired | Self::MissingToken => {
                StatusCode::UNAUTHORIZED
            }
            Self::GenTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        c.into_response()
    }
}
