use crate::error::PaperError;
use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::future;

#[derive(uniffi::Object)]
pub struct Recommender {}

#[uniffi::export]
impl Recommender {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_recommendations(
        &self,
        titles: Vec<String>,
        api_key: String,
    ) -> Result<Vec<String>, PaperError> {
        let client =
            Client::with_config(async_openai::config::OpenAIConfig::new().with_api_key(api_key));
        let recommendations = future::join_all(titles.into_iter().map(|title| async {
            let messages = vec![
                async_openai::types::ChatCompletionRequestSystemMessage {
                    content: "You are a helpful librarian making book recommendations.".into(),
                    name: None,
                    role: Role::System,
                },
                async_openai::types::ChatCompletionRequestUserMessage {
                    content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                        format!("Suggest one similar book to '{}' and return just the title.", title)
                    ),
                    name: None,
                    role: Role::User,
                },
            ];

            let request = CreateChatCompletionRequestArgs::default()
                .model("gpt-3.5-turbo")
                .messages(messages)
                .max_tokens(50_u16)
                .build()?;

            match client.chat().create(request).await {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.message.content {
                            Ok(content.trim().to_string())
                        } else {
                            Err(PaperError::GeneralError)
                        }
                    } else {
                        Err(PaperError::GeneralError)
                    }
                }
                Err(_) => Err(PaperError::GeneralError),
            }
        }))
        .await;

        recommendations.into_iter().collect()
    }
}
