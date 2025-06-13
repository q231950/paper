use crate::error::PaperError;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures::future;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, uniffi::Record)]
pub struct Recommendation {
    pub book_titles: Vec<String>,
}

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
    ) -> Result<Recommendation, PaperError> {
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
                                .content("You are a helpful librarian making book recommendations. Always respond with valid JSON in the format: {\"book_titles\": [\"Title 1\", \"Title 2\", \"Title 3\"]}")
                                .build()
                                .expect("msg")
                                .into(),
                            ChatCompletionRequestUserMessageArgs::default()
                                .content(format!("Recommend 3-5 books similar to '{}'. Return only JSON with book_titles array.", title))
                                .build()
                                .expect("msg")
                                .into(),
                        ])
                        .build()
                        .expect("msg");

                    match value.chat().create(request).await {
                        Ok(response) => {
                            if let Some(content) = response.choices.first().and_then(|choice| choice.message.content.as_ref()) {
                                match serde_json::from_str::<Recommendation>(content.trim()) {
                                    Ok(recommendation) => Ok(recommendation),
                                    Err(e) => {
                                        println!("JSON parsing error: {}", e);
                                        println!("Raw content: {}", content);
                                        Err(PaperError::GeneralError)
                                    }
                                }
                            } else {
                                println!("No content in response");
                                Err(PaperError::GeneralError)
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

            // Combine all recommendations into a single Recommendation struct
            let mut all_book_titles = Vec::new();
            for result in recommendations {
                match result {
                    Ok(recommendation) => all_book_titles.extend(recommendation.book_titles),
                    Err(e) => return Err(e),
                }
            }
            
            Ok(Recommendation {
                book_titles: all_book_titles,
            })
        })
    }
}
