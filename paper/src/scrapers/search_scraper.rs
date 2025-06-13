use crate::api::APIClient;
use crate::error::PaperError;
use crate::model::{APIConfiguration, SearchResultList, API};
use crate::scrapers::opc4v2_13vzg6::Opc4v2_13Vzg6SearchScraper;
use crate::scrapers::public_hamburg::HamburgPublicSearchScraper;
use reqwest::Client;

#[derive(uniffi::Object)]
pub struct SearchScraper {
    configuration: APIConfiguration,
}

#[uniffi::export(async_runtime = "tokio")]
impl SearchScraper {
    #[uniffi::constructor]
    pub fn new(configuration: APIConfiguration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub async fn search(
        &self,
        text: &str,
        next_page_url: Option<String>,
    ) -> Result<SearchResultList, PaperError> {
        match self.configuration.api {
            API::HamburgPublic => {
                let search_scraper = HamburgPublicSearchScraper {};
                search_scraper.search(text, next_page_url).await
            }
            API::Opc4v2_13Vzg6 => {
                let client = Client::new();
                let api_client =
                    APIClient::new_with_network_client(client, self.configuration.base_url.clone());

                let search_scraper = Opc4v2_13Vzg6SearchScraper {};
                search_scraper
                    .search(text, next_page_url, &api_client)
                    .await
            }
        }
    }
}
