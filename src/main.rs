use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/tickets", routes::tickets_router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8899));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    axum::serve(tcp_listener, app).await.unwrap();
}
