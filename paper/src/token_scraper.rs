use crate::model::API;

use super::error::PaperError;
use super::html_resource::HTMLResource;
use reqwest::Client;
use scraper::Selector;

// Scrapes the REQUEST_TOKEN from a login page
pub struct TokenScraper {
    pub api: API,
}

impl TokenScraper {
    pub async fn get_request_token(&self, client: &Client) -> Result<String, PaperError> {
        match self.api {
            API::HamburgPublic => {
                let resource = HTMLResource {
                    client: client.to_owned(),
                    url: "https://www.buecherhallen.de/login.html".to_string(),
                };
                let html = resource.load().await?;
                let document = scraper::Html::parse_document(html.as_str());
                let selector = Selector::parse(r#"input[name=REQUEST_TOKEN]"#)?;
                let input = document
                    .select(&selector)
                    .next()
                    .ok_or_else(|| PaperError::GeneralError)?;

                return input
                    .value()
                    .attr("value")
                    .map(|x| x.to_string())
                    .ok_or_else(|| PaperError::GeneralError);
            }
            _ => Err(PaperError::FailedToGetRequestToken),
        }
    }
}
