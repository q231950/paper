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
        let re = Regex::new(r"(\n)+\s*\W").expect("Regex must always be correct");

        let mut output_string = String::new();
        for text in elem.text() {
            if re.is_match(text) {
                output_string.push_str("");
            } else if text == "" {
                output_string.push_str(" ");
            } else {
                output_string.push_str(text);
                output_string.push_str(" ");
                output_string = output_string.replace("  ", " ");
            }
        }
        output_string.trim_start().trim_end().to_string()
    }
}
