use std::sync::Arc;

use crate::{models::UserModel, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRep {
    access_token: String,
    token_type: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> impl IntoResponse {
    println!("email: {}\tpassword: {}", payload.email, payload.password);
    // if the user was exist, direct to login and gen token
    let _result = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM `users` WHERE `email` = ? ) AS `exists!`"#,
        &payload.email
    )
    .fetch_one(&state.db)
    .await;

    // Response to be designed
    ()
}
