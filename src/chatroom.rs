use askama::Template;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    openai::{
        chat_completion::{self, ChatCompletionArgs, ChatCompletionMessage},
        OpenAIClient,
    },
    AppState,
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

pub fn router() -> Router<AppState> {
    return Router::new().route("/", get(connect));
}

async fn connect(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|s| handle_socket(state, s))
}

async fn handle_socket(state: AppState, mut socket: WebSocket) {
    if socket
        .send(Message::Text(
            ChatMessageTemplate {
                is_bot: true,
                message: "What's on your mind?".to_string(),
            }
            .render()
            .unwrap(),
        ))
        .await
        .is_err()
    {
        println!("Error happened");
    }

    state.messages.lock().await.push(ChatCompletionMessage {
        role: "assistant".to_string(),
        content: "What's on your mind?".to_string(),
    });

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

        state.messages.lock().await.push(ChatCompletionMessage {
            role: "user".to_string(),
            content: msg.message.clone(),
        });

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

        let mut messages = state.messages.lock().await.to_vec();
        messages.reverse();
        messages.push(ChatCompletionMessage {
            role: "system".to_string(),
            content: "You are a therapist that will try to get the user to open up about their situation. Ask them clarifying questions but don't suggest solutions unless asked. Use casual language like two friends talking and don't be verbose in your responses.".to_string(),
        });
        messages.reverse();

        let completion = match chat_completion::create(
            &client,
            ChatCompletionArgs {
                messages,
                response_format: None,
                model: String::from("gpt-3.5-turbo"),
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

        state
            .messages
            .lock()
            .await
            .push(completion.choices[0].message.clone());

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
