use axum::{extract::Path, response::IntoResponse};

pub async fn get_handler(Path(id): Path<u64>) -> impl IntoResponse {
    format!("Get ticket by id#{id}")
}