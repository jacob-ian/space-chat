use askama::Template;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "chat/message.html")]
struct ChatMessageTemplate {
    is_bot: bool,
    message: String,
}

#[derive(Deserialize, Serialize)]
struct HtmxWSMessage {
    message: String,
}

pub fn router() -> Router {
    return Router::new().route("/", get(connect));
}

async fn connect(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if socket
        .send(Message::Text(
            ChatMessageTemplate {
                is_bot: true,
                message: "Welcome! This is a chat with a robot!".to_string(),
            }
            .render()
            .unwrap(),
        ))
        .await
        .is_err()
    {
        println!("Error happened");
    }

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        let msg = serde_json::from_str::<HtmxWSMessage>(&msg.to_text().unwrap()).unwrap();

        if socket
            .send(Message::Text(
                ChatMessageTemplate {
                    is_bot: false,
                    message: msg.message,
                }
                .render()
                .unwrap(),
            ))
            .await
            .is_err()
        {
            // client disconnected
            return;
        }
    }
}
