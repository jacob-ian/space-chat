use std::time::Duration;

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
use tokio::time::sleep;

use crate::openai::{
    chat_completion::{self, ChatCompletionArgs, ChatCompletionMessage},
    OpenAIClient,
};

#[derive(Template)]
#[template(path = "chat/message.html")]
struct ChatMessageTemplate {
    is_bot: bool,
    message: String,
}

#[derive(Template)]
#[template(path = "chat/thinking.html")]
struct ThinkingTemplate {}

#[derive(Template)]
#[template(
    source = r#"<div id="thinker" hx-swap-oob="delete"></div>"#,
    ext = "html"
)]
struct RemoveThinkingTemplate {}

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

        let msg = if let Ok(m) = serde_json::from_str::<HtmxWSMessage>(&msg.to_text().unwrap()) {
            m
        } else {
            println!("message error happened");
            return;
        };

        if socket
            .send(Message::Text(
                ChatMessageTemplate {
                    is_bot: false,
                    message: msg.message.clone(),
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

        if socket
            .send(Message::Text(ThinkingTemplate {}.render().unwrap()))
            .await
            .is_err()
        {
            return;
        }

        let client = OpenAIClient::new(String::from(
            "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",
        ));
        let completion = match chat_completion::create(
            &client,
            ChatCompletionArgs {
                response_format: None,
                model: String::from("gpt-3.5-turbo"),
                messages: vec![ChatCompletionMessage {
                    role: String::from("system"),
                    content: String::from("You are a psychoanalyst"),
                }],
            },
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);
                socket
                    .send(Message::Text(RemoveThinkingTemplate {}.render().unwrap()))
                    .await
                    .unwrap();
                return;
            }
        };

        if socket
            .send(Message::Text(RemoveThinkingTemplate {}.render().unwrap()))
            .await
            .is_err()
        {
            return;
        }

        if socket
            .send(Message::Text(
                ChatMessageTemplate {
                    is_bot: true,
                    message: completion.choices[0].message.content.clone(),
                }
                .render()
                .unwrap(),
            ))
            .await
            .is_err()
        {
            return;
        }
    }
}
