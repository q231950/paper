use crate::api::APIClient;
use crate::error::PaperError;
use crate::model::APIConfiguration;
use crate::model::Availability;
use crate::model::AvailabilityStatus;
use crate::model::Loan;
use crate::model::SearchResultDetail;

use super::opc4v2_13vzg6::Opc4v2_13Vzg6SearchDetailScraper;
use super::public_hamburg::HamburgPublicSearchDetailScraper;

#[derive(uniffi::Object)]
pub(crate) struct SearchDetailScraper {
    configuration: APIConfiguration,
}

#[uniffi::export(async_runtime = "tokio")]
impl SearchDetailScraper {
    #[uniffi::constructor]
    pub fn new(configuration: APIConfiguration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    async fn details_for_url(&self, url: String) -> Result<SearchResultDetail, PaperError> {
        let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
        self.details_for_url_on_current_runtime(&client, url).await
    }

    pub fn status(&self, availabilities: Vec<Availability>) -> AvailabilityStatus {
        if availabilities.is_empty() {
            return AvailabilityStatus::NoneAvailable {};
        }

        let available_count = availabilities
            .iter()
            .filter(|a| matches!(a, Availability::Available(_)))
            .count();

        match available_count {
            0 => AvailabilityStatus::NoneAvailable {},
            n if n == availabilities.len() => AvailabilityStatus::AllAvailable {},
            _ => AvailabilityStatus::SomeAvailable {},
        }
    }
}

impl SearchDetailScraper {
    pub(crate) async fn details(
        &self,
        for_loan: Loan,
        client: &reqwest::Client,
    ) -> Result<Loan, PaperError> {
        println!("Getting detail for {:?}", for_loan.item_number);

        let detail = self
            .details_for_url_on_current_runtime(
                client,
                for_loan
                    .search_result_detail_url
                    .clone()
                    .ok_or_else(|| PaperError::ParseErrorSearchResultDetail)?,
            )
            .await;

        return match detail {
            Ok(d) => Ok(for_loan.with_details(d)),
            Err(e) => Err(e),
        };
    }

    async fn details_for_url_on_current_runtime(
        &self,
        client: &reqwest::Client,
        url: String,
    ) -> Result<SearchResultDetail, PaperError> {
        let api_client = APIClient::new_with_network_client(
            client.to_owned(),
            self.configuration.catalog_url.to_string(),
        );
        let document = api_client.get_html_at_path(url).await?;

        match self.configuration.api {
            crate::model::API::HamburgPublic => {
                Ok(HamburgPublicSearchDetailScraper::search_result_detail_from(document).await)
            }
            crate::model::API::Opc4v2_13Vzg6 => {
                Opc4v2_13Vzg6SearchDetailScraper {}.search_detail_from(document)
            }
        }
    }
}
