#![allow(unused)]
use axum::response::Html;
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Starting Rust web server...");
    // let routes_hello = Router::new().route(
    //     "/hello",
    //     get(|| async { Html("Hello <strong>World.</strong>") }),
    // );

    //          ---- Start server
    let routes_hello = Router::new().route("/hello", get(handler_hello));

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

    //          ---- Start server

    //          ---- Handler Hello
    async fn handler_hello() -> impl IntoResponse {
        println!("->> {:<12} - handler_hello", "HANDLER");
        Html("Hello <strong>World!</strong>")
    }
    //          ---- Handler Hello
}
