use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // Specify the address and port
    let addr = SocketAddr::from(([127, 0, 0, 1], 5002));
    println!("Listening on http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
