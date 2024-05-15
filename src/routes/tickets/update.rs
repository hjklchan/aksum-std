use axum::{extract::Path, response::IntoResponse};

pub async fn update_handler(Path(id): Path<u64>) -> impl IntoResponse {
    format!("Update ticket by id#{id}")
}