use scraper::Selector;

use crate::error::PaperError;

pub(crate) struct RenewalTokenParser {}

impl RenewalTokenParser {
    pub(crate) fn parse(&self, html: String) -> Result<String, PaperError> {
        let document = scraper::Html::parse_document(html.as_ref());
        let renewal_request_token_selector =
            Selector::parse(r#"#footer-newsletter > form > div > input[name="REQUEST_TOKEN"]"#)?;
        let input = document
            .select(&renewal_request_token_selector)
            .next()
            .ok_or_else(|| PaperError::RenewalTokenParserFailedToParseToken)?;

        return input
            .attr("value")
            .map(|s| Ok(s.to_string()))
            .unwrap_or_else(|| Err(PaperError::IncorrectCredentials));
    }

    pub(crate) fn new() -> RenewalTokenParser {
        RenewalTokenParser {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn test_scrapes_token_after_login() {
        let html = fs::read_to_string("src/fixtures/hamburg_public/login/login_success.html")
            .expect("Something went wrong reading the file");
        let parser = RenewalTokenParser::new();
        let result = parser.parse(html);
        assert_eq!(
            result.unwrap(),
            "MXlz3GJVbMU1_cGE1Ol89aRF_fDJOULbvziL5f2l5Rs".to_string()
        );
    }
}
