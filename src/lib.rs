use std::sync::Arc;

use openai::chat_completion::ChatCompletionMessage;
use tokio::sync::Mutex;

pub mod assets;
pub mod chatroom;
pub mod errors;
pub mod openai;

#[derive(Clone)]
pub struct AppState {
    pub messages: Arc<Mutex<Vec<ChatCompletionMessage>>>,
}
