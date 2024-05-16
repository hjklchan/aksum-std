use std::{env, net::SocketAddr};

use axum::{extract::State, response::IntoResponse, routing, Router};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    db: Pool<MySql>,
}

#[tokio::main]
async fn main() {
    // initialize env file
    dotenv::dotenv().expect("The .env file does not exist");

    let db_dsn = env::var("DATABASE_DSN").unwrap_or_default();
    // initialize database connect pool
    let db = MySqlPoolOptions::new().connect(&db_dsn).await.unwrap();

    // initialize state and app instance
    let app_state = AppState { db };
    // ! Should move the catalogue for management
    let app: Router = Router::new()
        .layer(cors_layer())
        .nest(
            "/users",
            Router::new().route("/login", routing::post(routes::users::login_handler)),
        )
        .nest(
            "tickets",
            Router::new()
                .route("/", routing::get(routes::tickets::list_handler))
                .route("/:id", routing::get(routes::tickets::get_handler))
                .route("/", routing::post(routes::tickets::create_handler))
                .route("/:id", routing::delete(routes::tickets::delete_handler))
                .route("/:id", routing::patch(routes::tickets::update_handler)),
        )
        .with_state(app_state);

    // create a tcp listener
    let port: u16 = env::var("PORT").unwrap_or("9000".into()).parse().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    // run http server
    axum::serve(tcp_listener, app).await.unwrap();
}

// CORS layer
fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods(cors::Any)
        .allow_origin(cors::Any)
}
