use reqwest::{
    cookie::Jar,
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    authenticators::{OpacAuthenticator, PublicHamburgAuthenticator},
    configuration::Configuration,
    error::PaperError,
    model::{Loan, API},
};

use super::{opc4v2_13vzg6::Opc4v2_13Vzg6RenewalParser, public_hamburg::RenewalLoanParser};

#[derive(uniffi::Object)]
pub struct RenewalService {}

#[uniffi::export(async_runtime = "tokio")]
impl RenewalService {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn renew(
        &self,
        item_number: String,
        renewal_token: Option<String>,
        configuration: Configuration,
    ) -> Result<Loan, PaperError> {
        match configuration.api_configuration.api {
            API::HamburgPublic => self.public_hamburg_renew(item_number, configuration).await,
            API::Opc4v2_13Vzg6 => {
                if let Some(token) = renewal_token {
                    self.opc4v2_13vzg6_renew(token, configuration).await
                } else {
                    Err(PaperError::MissingRenewalToken)
                }
            }
        }
    }

    // POST
    // /LBS_WEB/borrower/loans.htm?volumeNumbersToRenew=7574744&LAN=DU&username=00184808979&password=xxx&renew=Verl%C3%A4ngern
    async fn opc4v2_13vzg6_renew(
        &self,
        renewal_token: String,
        configuration: Configuration,
    ) -> Result<Loan, PaperError> {
        println!("opc4v2_13vzg6_renew start");
        let cookie_store = Arc::new(Jar::default());
        let client_builder = ClientBuilder::new();
        let client = client_builder
            .cookie_store(true)
            .cookie_provider(cookie_store.clone())
            .build()?;

        let authenticator = OpacAuthenticator {
            configuration: configuration.clone(),
        };
        _ = authenticator.authenticate(&client).await;

        let username = configuration.username.clone().unwrap();
        let password = configuration.password.clone().unwrap();

        client.get(configuration.base_url()).send().await?;
        client.get(configuration.session_url()).send().await?;

        let url = format!(
            "{}/LBS_WEB/borrower/loans.htm",
            configuration.api_configuration.base_url
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/x-www-form-urlencoded; charset=utf-8").unwrap(),
        );

        headers.insert(
            "User-Agent",
            HeaderValue::from_str("Flying Penguin").unwrap(),
        );

        let request = client
            .post(url)
            .headers(headers)
            .timeout(Duration::from_secs(20))
            .query(&[
                ("volumeNumbersToRenew", renewal_token.as_str()),
                ("LAN", "DU"),
                ("username", username.as_str()),
                ("password", password.as_str()),
                ("renew", "Verlängern"),
            ]);

        let html = request.send().await?.text().await?;

        Opc4v2_13Vzg6RenewalParser::loan_from(html)
    }

    async fn public_hamburg_renew(
        &self,
        item_number: String,
        configuration: Configuration,
    ) -> Result<Loan, PaperError> {
        let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;

        return tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let request_token = PublicHamburgAuthenticator {
                    configuration: configuration.clone(),
                }
                .public_hamburg_authenticate_and_get_request_access_token(&client)
                .await;

                if let Ok(request_token) = request_token {
                    let mut params = HashMap::new();
                    params.insert("FORM_SUBMIT", "tl_renewal_action".to_string());
                    params.insert("REQUEST_TOKEN", request_token);
                    params.insert("actionType", "renewItem".to_string());
                    params.insert("itemId", item_number.clone());

                    let mut headers = HeaderMap::new();
                    headers.append("Accept-Language", HeaderValue::from_str("en-us")?);
                    headers.append(
                        "User-Agent",
                        HeaderValue::from_str("the quick sloth climbs the tree")?,
                    );

                    let response = client
                        .post("https://www.buecherhallen.de/entliehene-medien.html")
                        .headers(headers)
                        .form(&params)
                        .send()
                        .await?;

                    let html = response.text().await?;

                    return RenewalLoanParser::loan_from(item_number, html);
                }

                return Err(PaperError::FailedToRenew);
            });
    }
}
