use reqwest::Client;
use scraper::Selector;

use super::opc4v2_13vzg6_loan_scraper::LoanScraper;
use crate::{api::APIClient, configuration::Configuration, error::PaperError, model::Account};

pub(crate) struct Opc4v2_13Vzg6AccountScraper {
    pub(crate) configuration: Configuration,
}

impl Opc4v2_13Vzg6AccountScraper {
    pub(crate) async fn scrape_account(&self, client: Client) -> Result<Account, PaperError> {
        let mut account = Account::new();
        println!("Scraping account (loans + balance + account info)…");

        let api = APIClient::new_with_network_client(
            client,
            self.configuration.api_configuration.base_url.clone(),
        );

        let html = api
            .get_html_at_path("LBS_WEB/borrower/loans.htm".to_string())
            .await?;
        let loans = LoanScraper::new().scrape_loans(html);
        account.loans = loans;

        //      > get balance

        return Ok(account);
    }

    pub(crate) fn sign_in_account(&self, html_string: &str) -> Result<bool, PaperError> {
        println!("Signing in account…");

        let html = scraper::Html::parse_document(html_string);

        let check_authenticated_selector = Selector::parse(
            r#"body > div.lrmargin > div > table > tbody > tr > td.left10 > a[href="/LBS_WEB/logout"]"#,
        ).unwrap();

        let signed_in = html.select(&check_authenticated_selector).next().is_some();

        println!("signed_in: {:?}", signed_in);

        return Ok(signed_in);
    }
}

#[cfg(test)]
mod tests {
    use crate::scrapers::opc4v2_13vzg6::Opc4v2_13Vzg6AccountScraper;
    use std::fs;

    #[tokio::test]
    async fn test_opac_authenticator_sign_in_success() {
        let scraper = Opc4v2_13Vzg6AccountScraper {
            configuration: crate::configuration::Configuration {
                username: Some("".to_string()),
                password: Some("".to_string()),
                api_configuration: crate::model::APIConfiguration {
                    api: crate::model::API::Opc4v2_13Vzg6,
                    base_url: "".to_string(),
                    catalog_url: "".to_string(),
                },
            },
        };
        let html = fs::read_to_string("src/fixtures/opc4v2_13Vzg6/login/borrower_gbv_sbb.html")
            .expect("Something went wrong reading the file");
        let result = scraper.sign_in_account(html.as_str());
        assert_eq!(result, Ok(true));
    }

    #[tokio::test]
    async fn test_opac_authenticator_sign_in_incorrect_credentials() {
        let scraper = Opc4v2_13Vzg6AccountScraper {
            configuration: crate::configuration::Configuration {
                username: Some("".to_string()),
                password: Some("".to_string()),
                api_configuration: crate::model::APIConfiguration {
                    api: crate::model::API::Opc4v2_13Vzg6,
                    base_url: "".to_string(),
                    catalog_url: "".to_string(),
                },
            },
        };
        let html =
            fs::read_to_string("src/fixtures/opc4v2_13Vzg6/login/login_incorrect_credentials.html")
                .expect("Something went wrong reading the file");
        let result = scraper.sign_in_account(html.as_str());
        assert_eq!(result, Ok(false));
    }
}
