use axum::{routing, Router};

mod tickets;
mod users;

use self::{
    tickets::{create_handler, delete_handler, get_handler, update_handler},
    users::login_handler,
};

pub fn users_router() -> Router {
    Router::new().route("/users/login", routing::post(login_handler))
}

pub fn tickets_router() -> Router {
    Router::new()
        .route("/", routing::get(tickets::list_handler))
        .route("/:id", routing::get(get_handler))
        .route("/", routing::post(create_handler))
        .route("/:id", routing::delete(delete_handler))
        .route("/:id", routing::patch(update_handler))
}
