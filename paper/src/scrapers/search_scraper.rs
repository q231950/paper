use crate::api::APIClient;
use crate::error::PaperError;
use crate::model::{APIConfiguration, SearchResultList, API};
use crate::scrapers::opc4v2_13vzg6::Opc4v2_13Vzg6SearchScraper;
use crate::scrapers::public_hamburg::HamburgPublicSearchScraper;
use reqwest::Client;
use tokio::runtime::Builder;

#[derive(uniffi::Object)]
pub struct SearchScraper {
    configuration: APIConfiguration,
}

#[uniffi::export]
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
                let search_result = search_scraper.search(text, next_page_url).await;
                return search_result;
            }
            API::Opc4v2_13Vzg6 => {
                let client = Client::new();
                let api_client =
                    APIClient::new_with_network_client(client, self.configuration.base_url.clone());

                let runtime = Builder::new_multi_thread()
                    .worker_threads(5)
                    .thread_name("search")
                    .enable_io()
                    .enable_time()
                    .build()?;

                return runtime.block_on(async {
                    let search_scraper = Opc4v2_13Vzg6SearchScraper {};
                    let search_result = search_scraper
                        .search(text, next_page_url, &api_client)
                        .await;
                    return search_result;
                });
            }
        }
    }
}
