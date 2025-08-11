use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    let static_service = ServeDir::new("static");
    let static_service = static_service.append_index_html_on_directories(true);
}