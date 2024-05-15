use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    email: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct LoginRep {
    access_token: String,
    token_type: String,
}

pub async fn login_handler() -> Result<Json<LoginRep>, StatusCode> {
    todo!()
}