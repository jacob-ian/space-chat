use reqwest::Client;

pub mod chat_completion;

pub struct OpenAIClient {
    api_key: String,
    client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> OpenAIClient {
        return OpenAIClient {
            api_key,
            client: Client::new(),
        };
    }
}
