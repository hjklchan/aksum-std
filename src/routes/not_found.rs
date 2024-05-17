use axum::response::IntoResponse;

use crate::errors::Error;


pub async fn route_not_found_handler() -> impl IntoResponse {
    Error::RouteNotFound
}