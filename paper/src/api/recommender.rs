use crate::error::PaperError;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
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
        let recommendations = future::join_all(titles.into_iter().map(|title| {
            let value = client.clone();
            async move {
                let request = CreateChatCompletionRequestArgs::default()
                    .model("gpt-4o")
                    .max_tokens(50_u16)
                    .messages([
                        ChatCompletionRequestSystemMessageArgs::default()
                            .content("You are a helpful librarian making book recommendations.")
                            .build()
                            .expect("msg")
                            .into(),
                        ChatCompletionRequestUserMessageArgs::default()
                            .content(format!("Recommend books similar to '{}'.", title))
                            .build()
                            .expect("msg")
                            .into(),
                    ])
                    .build()
                    .expect("msg");

                match value.chat().create(request).await {
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
            }
        }))
        .await;

        recommendations.into_iter().collect()
    }
}
