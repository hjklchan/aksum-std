use crate::errors::Error;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
    email: String,
    password: String,
    // unused now
    #[allow(unused)]
    captcha: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterRep;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterReq>,
) -> Result<StatusCode, Error> {
    // Check if the email has exists in database
    let exists = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM `users` WHERE `email` = ? ) AS `exists`"#,
        &payload.email
    )
    .fetch_one(&state.db)
    .await
    .map(|record| record.exists == 1)
    .map_err(|err| Error::Sqlxxx(err))?;

    // if not exists
    if exists {
        return Err(Error::AlreadyExist("user"));
    }

    // Create new user
    sqlx::query!(
        "INSERT INTO `users`(`username`, `email`, `password`, `status`) VALUES(?, ?, ?, ?)",
        &payload.email,
        &payload.email,
        // TODO: Should encrypt the password
        &payload.password,
        0
    )
    .execute(&state.db)
    .await
    .map(|_| ())
    .map_err(|err| Error::Sqlxxx(err))?;
    
    Ok(StatusCode::CREATED)
}
