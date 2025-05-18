use std::sync::Arc;

use crate::authenticators::{OpacAuthenticator, PublicHamburgAuthenticator};
use crate::{
    configuration::Configuration, error::PaperError,
    scrapers::public_hamburg::PublicHamburgAccountScraper, scrapers::BalanceScraper,
};

use crate::model::Loan;
use crate::model::Loans;
use futures::future;
use reqwest::cookie::Jar;
use reqwest::ClientBuilder;
use tokio::runtime::Builder;

use super::opc4v2_13vzg6::Opc4v2_13Vzg6AccountScraper;
use super::SearchDetailScraper;

use crate::model::Account;

// Scrapes the library website
// Supports:
//   - account
//   - search
#[derive(uniffi::Object)]
pub struct LibraryScraper {
    configuration: Configuration,
}

#[uniffi::export]
impl LibraryScraper {
    #[uniffi::constructor]
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }

    // Authenticate with the given configuration and
    // return a result with either an account or an error
    pub async fn fetch_account(&self) -> Result<Account, PaperError> {
        let cookie_store = Arc::new(Jar::default());
        let client_builder = ClientBuilder::new();
        let client = client_builder
            .cookie_store(true)
            .cookie_provider(cookie_store.clone())
            .build()?;
        let runtime = Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("fetch account")
            .enable_io()
            .enable_time()
            .build()?;

        return runtime.block_on(async {
            match self.configuration.api_configuration.api {
                crate::model::API::HamburgPublic => {
                    self.public_hamburg_fetch_on_current_runtime(&client).await
                }
                crate::model::API::Opc4v2_13Vzg6 => {
                    client
                        .get(self.configuration.session_url())
                        .query(&[("USR", "1022"), ("LAN", "DU"), ("BES", "1")])
                        .send()
                        .await?;

                    self.opc4v2_13vzg6_fetch_on_current_runtime(&client).await
                }
            }
        });
    }
}

impl LibraryScraper {
    // collects parts of an account into a whole Account
    // - authenticates
    // - gets the loans
    // - gets the balance

    pub async fn opc4v2_13vzg6_fetch_on_current_runtime(
        &self,
        client: &reqwest::Client,
    ) -> Result<Account, PaperError> {
        let authenticator = OpacAuthenticator {
            configuration: self.configuration.clone(),
        };

        let authenticated = authenticator.authenticate(&client).await?;

        if !authenticated {
            return Err(PaperError::IncorrectCredentials);
        }

        let account_scraper = Opc4v2_13Vzg6AccountScraper {
            configuration: self.configuration.clone(),
        };

        account_scraper.scrape_account(client.to_owned()).await
    }
}

impl LibraryScraper {
    // collects parts of an account into a whole Account
    // - authenticates
    // - gets the loans
    // - gets the balance

    pub async fn public_hamburg_fetch_on_current_runtime(
        &self,
        client: &reqwest::Client,
    ) -> Result<Account, PaperError> {
        let loans_result = PublicHamburgAuthenticator {
            configuration: self.configuration.clone(),
        }
        .authenticate_public_hamburg_and_get_loans(client)
        .await;

        match loans_result {
            Ok(loans) => {
                let account_scraper = PublicHamburgAccountScraper {};
                let detail_scraper =
                    SearchDetailScraper::new(self.configuration.api_configuration.clone());
                if let Ok(mut account) = account_scraper.scrape(client).await {
                    let detailed_loans = future::join_all(loans.into_iter().map(|loan| async {
                        match detail_scraper.details(loan, client).await {
                            Ok(detailed_loan) => Ok(detailed_loan),
                            Err(e) => Err(e),
                        }
                    }))
                    .await;

                    let detailed_loans: Result<Vec<Loan>, PaperError> =
                        detailed_loans.into_iter().collect();

                    match detailed_loans {
                        Ok(loans) => {
                            account.loans = Loans { loans };
                        }
                        Err(e) => return Err(e),
                    }

                    // in parallel to loans we should get the balance
                    match BalanceScraper::new(&client).scrape_balance().await {
                        Ok(balance) => account.balance = Some(balance),
                        Err(error) => return Err(error),
                    }

                    return Ok(account);
                }
            }
            Err(error) => return Err(error),
        }

        return Err(PaperError::GeneralError);
    }
}
