use super::{LoginResult, RawLoansPage};
use crate::configuration::Configuration;
use crate::error::PaperError;
use crate::model::Loans;
use crate::scrapers::public_hamburg::LoansScraper;
use crate::token_scraper::TokenScraper;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::collections::HashMap;

pub(crate) struct PublicHamburgAuthenticator {
    pub(crate) configuration: Configuration,
}

impl PublicHamburgAuthenticator {
    pub(crate) async fn verify_credentials_public_hamburg(&self) -> Result<String, PaperError> {
        let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
        self.public_hamburg_authenticate_and_get_request_access_token(&client)
            .await
    }

    async fn authenticate_public_hamburg_via_cookies(
        &self,
        client: &Client,
        request_token: String,
    ) -> Result<RawLoansPage, PaperError> {
        if let (Some(username), Some(password)) = (
            self.configuration.username.clone(),
            self.configuration.password.clone(),
        ) {
            if username == "" || password == "" {
                return Err(PaperError::CredentialsBadInput);
            }
            let mut params = HashMap::new();
            params.insert("FORM_SUBMIT", "tl_login".to_string());
            params.insert("REQUEST_TOKEN", request_token);
            params.insert("username", username);
            params.insert("password", password);

            let mut headers = HeaderMap::new();

            if let Ok(header) = HeaderValue::from_str("en-us") {
                headers.append("Accept-Language", header);
            }

            if let Ok(header) = HeaderValue::from_str("the quick sloth climbs the tree") {
                headers.append("User-Agent", header);
            }

            let response = client
                .post("https://www.buecherhallen.de/login.html")
                .headers(headers)
                .form(&params)
                .send()
                .await?;

            let html = response.text().await?;

            return LoginResult::from_public_hamburg_html(html);
        } else {
            return Err(PaperError::CredentialsBadInput);
        }
    }

    pub(crate) async fn authenticate_public_hamburg_and_get_loans(
        &self,
        client: &Client,
    ) -> Result<Loans, PaperError> {
        let token_scraper = TokenScraper {
            api: self.configuration.api_configuration.api.clone(),
        };

        let token = token_scraper.get_request_token(&client).await?;
        let login_result = self
            .authenticate_public_hamburg_via_cookies(&client, token)
            .await;

        return match login_result {
            Ok(s) => LoansScraper::loans_from_html(s),
            Err(error) => Err(error),
        };
    }

    /// Authenticates using the credentials in the configuration and returns an access token
    /// that can be used to _renew_ an item
    pub(crate) async fn public_hamburg_authenticate_and_get_request_access_token(
        &self,
        client: &Client,
    ) -> Result<String, PaperError> {
        let token_scraper = TokenScraper {
            api: self.configuration.api_configuration.api.clone(),
        };

        let request_token = token_scraper.get_request_token(&client).await?;
        let result = self
            .authenticate_public_hamburg_via_cookies(&client, request_token.clone())
            .await;

        return match result {
            Ok(_) => Ok(request_token),
            _ => Err(PaperError::GeneralError),
        };
    }
}
