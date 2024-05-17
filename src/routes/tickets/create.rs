use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TicketStatus {
    Todo,
    InProgress,
    UnderReview,
    Done,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct CreateReq {
    title: String,
    description: String,
    body: String,
    assignee: String,
    status: TicketStatus,
}

#[allow(dead_code)]
struct CreateRep {}

pub async fn create_handler(Json(req): Json<CreateReq>) -> impl IntoResponse {
    println!("{:#?}", req);
    // Repo
    "Create ticket"
}
