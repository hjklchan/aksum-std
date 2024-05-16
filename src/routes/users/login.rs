use crate::errors::Error;
use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

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
) -> Result<Json<LoginRep>, Error> {
    println!("email: {}\tpassword: {}", payload.email, payload.password);
    // if the user was exist, direct to login and gen token
    let result = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM `users` WHERE `email` = ? ) AS `exists!`"#,
        &payload.email
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| Error::Sqlxxx(err))?;

    // user not found
    if result.exists != 1 {
        return Err(Error::not_found("user"));
    }

    // check if the password is correct
    // TODO: to prepare sql
    let result = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM `users` WHERE `email` = ? AND `password` = ? ) AS `exists!`"#,
        &payload.email,
        &payload.password
    )
        .fetch_one(&state.db)
        .await
        .map_err(|err| Error::Sqlxxx(err))?;
    if result.exists != 1 {
        return Err(Error::BadRequest("the user's password is incorrect".into()));
    }

    // TODO: Generate token here

    // Response to be designed
    Ok(Json(LoginRep {
        access_token: "".into(),
        token_type: "Bearer".into(),
    }))
}
