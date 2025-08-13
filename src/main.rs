use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    let static_service: ServeDir = ServeDir::new("static")
        .append_index_html_on_directories(true);

    let app: Router<()> = Router::new()
        .route("/", get(root_handler))
        .fallback_service(static_service); 
}

async fn root_handler() -> &'static str{
    "Привет! Это мой первй веб-сервер на Rust" 
}