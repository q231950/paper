use regex::Regex;
use reqwest::Client;
use scraper::ElementRef;

use crate::error::PaperError;

pub struct HTMLResource {
    pub client: Client,
    pub url: String,
}

impl HTMLResource {
    pub async fn load(&self) -> Result<String, PaperError> {
        let response = self.client.get(self.url.clone()).send().await?;
        return Ok(response.text().await?);
    }

    pub fn get_text_from_element(elem: ElementRef) -> String {
        let duplicated_whitespaces = Regex::new(r"\s\s+").expect("Regex must always be correct");

        let element_text = elem.text().collect::<String>();
        let rendered_text = duplicated_whitespaces.replace_all(element_text.trim(), " ");

        rendered_text.into_owned()
    }
}
