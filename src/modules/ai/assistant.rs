/// Now it's not more that test version, it hasn't been published to using.
use async_openai::Client;
use std::env;

pub struct AIClient {
    client: Client,
    model: String,
}

impl AIClient {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").expect("Set OPENAI_API_KEY in .env");

        Self {
            client: Client::new().with_api_key(api_key),
            model: "gpt-4-1106-preview".to_string(),
        }
    }

    pub async fn ask(&self, question: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self
            .client
            .chat()
            .create(|c| {
                c.model(&self.model)
                    .message(|m| m.role("user").content(question))
            })
            .await?;

        Ok(response.choices[0].message.content.clone())
    }
}
