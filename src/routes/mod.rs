// use axum::{extract::State, routing, Router};

// use crate::AppState;


pub mod tickets;
pub mod users;

// FIXME: Missing `State` in sub-routes
// pub fn users_router() -> Router {
//     Router::new().route("/users/login", routing::post(users::login_handler))
// }

// FIXME: Missing `State` in sub-routes
// pub fn tickets_router() -> Router {
//     Router::new()
//         .route("/", routing::get(tickets::list_handler))
//         .route("/:id", routing::get(get_handler))
//         .route("/", routing::post(create_handler))
//         .route("/:id", routing::delete(delete_handler))
//         .route("/:id", routing::patch(update_handler))
// }
