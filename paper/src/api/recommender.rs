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
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("recommendations")
            .enable_io()
            .enable_time()
            .build()?;

        runtime.block_on(async {
            let client = Client::with_config(
                async_openai::config::OpenAIConfig::new()
                    .with_api_key(api_key)
                    .with_api_base("https://openrouter.ai/api/v1"),
            );
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
                            let choices: Vec<String> = response.choices
                                .into_iter()
                                .filter_map(|choice| choice.message.content)
                                .map(|content| content.trim().to_string())
                                .collect();
                            
                            if choices.is_empty() {
                                println!("No valid choices returned");
                                Err(PaperError::GeneralError)
                            } else {
                                Ok(choices.join("\n"))
                            }
                        }
                        Err(e) => {
                            println!("API error: {}", e);
                            Err(PaperError::GeneralError)
                        }
                    }
                }
            }))
            .await;

            recommendations.into_iter().collect()
        })
    }
}
