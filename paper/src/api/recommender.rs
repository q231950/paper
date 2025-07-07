use crate::error::PaperError;
use async_openai::{
    types::{
        ChatCompletionRequestUserMessageArgs, ChatCompletionResponseFormat,
        ChatCompletionResponseFormatType, CreateChatCompletionRequestArgs,
    },
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

#[derive(uniffi::Record)]
pub struct RecommenderConfig {
    pub api_key: String,
    pub api_base: String,
    pub model: Model,
}

#[derive(uniffi::Enum)]
pub enum Model {
    GPT4_1,
}

impl Model {
    pub fn identifier(&self) -> String {
        match self {
            Model::GPT4_1 => "openai/gpt-4.1".to_string(),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Recommender {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_recommendations(
        &self,
        titles: Vec<String>,
        config: RecommenderConfig,
    ) -> Result<Recommendation, PaperError> {
        let client = Client::with_config(
            async_openai::config::OpenAIConfig::new()
                .with_api_key(config.api_key)
                .with_api_base(config.api_base),
        );

        // Format all titles as bullet points
        let titles_bullets = titles
            .iter()
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
        let content = format!(
            r#"
                            You are a helpful librarian making book recommendations.
                            Recommend 3 books similar to these titles:
                            {}
                            The books should be localized in the same language as the samples.
                            Always respond with valid JSON in the format: `{}`.
                            The response itself should be valid json.
                            Please do not include any additional text like markdown or explanations."#,
            titles_bullets, json_format
        );

        println!("Request content: {}", content);

        let request = CreateChatCompletionRequestArgs::default()
            .model(config.model.identifier())
            .max_tokens(500_u16)
            .response_format(ChatCompletionResponseFormat {
                r#type: ChatCompletionResponseFormatType::JsonObject,
            })
            .messages([ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()
                .expect("Should be able to create ChatCompletionRequestUserMessageArgs")
                .into()])
            .build()
            .expect("Should be able to create CreateChatCompletionRequestArgs");

        match client.chat().create(request).await {
            Ok(response) => {
                if let Some(content) = response
                    .choices
                    .first()
                    .and_then(|choice| choice.message.content.as_ref())
                {
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
}
