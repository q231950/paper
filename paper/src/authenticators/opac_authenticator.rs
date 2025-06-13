use reqwest::Client;

use crate::scrapers::opc4v2_13vzg6::Opc4v2_13Vzg6AccountScraper;
use crate::{configuration::Configuration, error::PaperError};
pub(crate) struct OpacAuthenticator {
    pub(crate) configuration: Configuration,
}

impl OpacAuthenticator {
    pub(crate) async fn authenticate(&self, client: &Client) -> Result<bool, PaperError> {
        println!("`OpacAuthenticator::authenticate`");
        let username = self.configuration.username.clone().unwrap();
        let password = self.configuration.password.clone().unwrap();
        let login_url = self.configuration.login_url();
        // println!("{:?}, {:?}, {:?}", username, password, login_url);
        let html_string = client
            .post(login_url)
            .query(&[
                ("USR", "1022"),
                ("BES", "1"),
                ("LAN", "DU"),
                ("username", username.as_str()),
                ("password", password.as_str()),
            ])
            .send()
            .await?
            .text()
            .await?;
        let scraper = Opc4v2_13Vzg6AccountScraper {
            configuration: self.configuration.clone(),
        };

        return scraper.sign_in_account(html_string.as_str());
    }
}
