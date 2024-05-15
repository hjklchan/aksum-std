use std::{env, net::SocketAddr};

use axum::Router;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use tokio::net::TcpListener;

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
    let db = MySqlPoolOptions::new()
        .connect(&db_dsn)
        .await
        .unwrap();

    // initialize state and app instance
    let app_state = AppState { db };
    let app = Router::new()
        .with_state(app_state)
        .nest("/tickets", routes::tickets_router());

    // create a tcp listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 8899));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    // run http server
    axum::serve(tcp_listener, app).await.unwrap();
}
