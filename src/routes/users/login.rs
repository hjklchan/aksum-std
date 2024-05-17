use std::env;

use crate::errors::{AuthenticateError, Error};
use crate::jwt::OhMyClaims;
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

/// Type of Fields are from database
#[derive(Debug)]
pub struct User {
    id: usize,
    username: String,
    status: i8,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> Result<Json<LoginRep>, Error> {
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
    let result = sqlx::query!("SELECT `id`, `username`, `status` FROM `users` WHERE `email` = ? AND `password` = ? LIMIT 1", &payload.email, &payload.password).fetch_one(&state.db).await.map(|row| User {
        id: row.id as usize,
        username: row.username,
        status: row.status,
    }).map_err(|err| Error::Sqlxxx(err))?;

    // ? I don't know what the locked-in numbers are
    if result.status == 1 {
        return Err(Error::Authenticate(AuthenticateError::Locked(
            "user is locked".into(),
        )));
    }

    // TODO: Generate token here
    let claims = OhMyClaims {
        user_id: result.id as usize,
        username: result.username,
        // TODO: Should calculate exp duration
        exp: 10000000000,
    };
    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| Error::Authenticate(AuthenticateError::GenerateToken))?;
    let encoding_key = jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes());
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &encoding_key)
        .map_err(|_| Error::Authenticate(AuthenticateError::GenerateToken))?;

    // Response to be designed
    Ok(Json(LoginRep {
        access_token: token,
        token_type: "Bearer".into(),
    }))
}
