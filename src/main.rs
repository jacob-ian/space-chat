use std::net::SocketAddr;

use askama::Template;
use axum::{routing::get, Router};
use chat::{assets, chatroom};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/assets", assets::router())
        .route("/", get(chat))
        .nest("/chat", chatroom::router());

    let addr: SocketAddr = "0.0.0.0:4000".parse().unwrap();
    println!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "chat.html")]
struct ChatTemplate {}

async fn chat() -> ChatTemplate {
    return ChatTemplate {};
}
