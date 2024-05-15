use axum::response::IntoResponse;

pub async fn list_handler() -> impl IntoResponse {
    "Get ticket list handler"
}