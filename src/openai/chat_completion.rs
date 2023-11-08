use axum::http::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_string_pretty};

use crate::errors::Error;

use super::OpenAIClient;

#[derive(Deserialize)]
pub struct ChatCompletion {
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatCompletionMessage,
}

#[derive(Deserialize, Serialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatCompletionFormat {
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionArgs {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    pub response_format: Option<ChatCompletionFormat>,
}

pub async fn create(
    client: &OpenAIClient,
    args: ChatCompletionArgs,
) -> Result<ChatCompletion, Error> {
    let res = client
        .client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .bearer_auth(&client.api_key)
        .body(to_string(&args).unwrap()) // TODO: Better erroring
        .send()
        .await
        .map_err(|e| Error::Internal(format!("Could not build request: {}", e.to_string())))?;

    if let Err(_) = res.error_for_status_ref() {
        let text = res
            .text()
            .await
            .map_err(|e| Error::Internal(format!("Request Error: {}", e.to_string())))?;
        return Err(Error::Internal(text));
    }

    return res
        .json::<ChatCompletion>()
        .await
        .map_err(|e| Error::Internal(format!("Could not parse reponse: {}", e.to_string())));
}
