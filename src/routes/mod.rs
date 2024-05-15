use axum::{
    routing,
    Router,
};

use self::tickets::{create_handler, delete_handler, get_handler, update_handler};

mod tickets;

pub fn tickets_router() -> Router {
    Router::new()
        .route("/", routing::get(tickets::list_handler))
        .route("/:id", routing::get(get_handler))
        .route("/", routing::post(create_handler))
        .route("/:id", routing::delete(delete_handler))
        .route("/:id", routing::patch(update_handler))
}
