use crate::error::PaperError;
use futures::future;

#[derive(uniffi::Object)]
pub struct Recommender {
    client: reqwest::Client,
}

#[uniffi::export]
impl Recommender {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_recommendations(&self, titles: Vec<String>) -> Result<Vec<String>, PaperError> {
        // Simulate async recommendation generation
        let recommendations = future::join_all(titles.into_iter().map(|title| async move {
            // Here you would make actual API calls to get recommendations
            // For now just return a mock recommendation
            Ok(format!("Recommendation based on: {}", title))
        }))
        .await;

        // Collect results
        recommendations.into_iter().collect()
    }
}
