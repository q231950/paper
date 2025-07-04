use crate::error::PaperError;
use async_openai::{
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, uniffi::Record)]
pub struct BookRecommendation {
    pub title: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, uniffi::Record)]
pub struct Recommendation {
    pub recommendations: Vec<BookRecommendation>,
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

            // Format all titles as bullet points
            let titles_bullets = titles.iter()
                .map(|title| format!("• {}", title))
                .collect::<Vec<String>>()
                .join("\n");

            let json_format = r#"
                {
                    "recommendations": [
                        {"title": "Title 1", "author": "Author 1"},
                        {"title": "Title 2", "author": "Author 2"},
                        {"title": "Title 3", "author": "Author 3"}
                    ]
                }"#;
            let content = format!(r#"
                            You are a helpful librarian making book recommendations.
                            Recommend 3 books similar to these titles:
                            {}
                            The books should be localized in the same language as the samples.
                            Always respond with valid JSON in the format: `{}`.
                            The response itself should be valid json.
                            Please do not include any additional text like markdown or explanations."#, titles_bullets, json_format);

            println!("Request content: {}", content);

            let request = CreateChatCompletionRequestArgs::default()
                .model("openai/gpt-4.1")
                .max_tokens(500_u16)
                .messages([
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(content)
                        .build()
                        .expect("Should be able to create ChatCompletionRequestUserMessageArgs")
                        .into(),
                ])
                .build()
                .expect("Should be able to create CreateChatCompletionRequestArgs");

            match client.chat().create(request).await {
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
        })
    }
}
