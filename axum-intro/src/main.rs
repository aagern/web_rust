// #![allow(unused)]
// use axum::response::Html;
// use axum::{Router, routing::get};
// use std::net::SocketAddr;
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() {
//     let routes_hello = Router::new().route(
//         "/hello",
//         get(|| async { Html("Hello <strong>World!</strong>") }),
//     );

//     // region:      --- Start Server
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     let tcp = TcpListener::bind(&addr).await.unwrap();
//     println!("->> LISTENING on {addr}\n");
//     axum::serve(tcp, routes_hello).await.unwrap();
//     // endregion:   --- Start Server
// }

use axum::response::Html;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Starting Rust web server...");

    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!</strong>") }),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Attempting to bind to: {}", addr);

    match TcpListener::bind(&addr).await {
        Ok(tcp) => {
            println!("Successfully bound to {addr}");
            println!("Server starting...");
            axum::serve(tcp, routes_hello).await.unwrap();
        }
        Err(e) => {
            eprintln!("Failed to bind to {addr}: {:?}", e);
            std::process::exit(1);
        }
    }
}
