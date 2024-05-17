use axum::{extract::Path, response::IntoResponse};

pub async fn delete_handler(Path(id): Path<u64>) -> impl IntoResponse {
    format!("Delete ticket by id#{id}")
}
