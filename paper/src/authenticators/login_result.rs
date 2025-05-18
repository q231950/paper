use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use scraper::Selector;

pub type RawLoansPage = String;

#[derive(Debug, PartialEq)]
pub struct LoginResult {}

impl LoginResult {
    pub(crate) fn from_public_hamburg_html(html: String) -> Result<RawLoansPage, PaperError> {
        let document = scraper::Html::parse_document(html.as_ref());
        let login_failed_selector = Selector::parse(r#".login-failed"#)?;

        if let Some(failure_reason_element_ref) = document.select(&login_failed_selector).next() {
            let failure_reason_text =
                HTMLResource::get_text_from_element(failure_reason_element_ref);
            if failure_reason_text.contains(
                "Der Login ist derzeit nicht möglich, bitte versuchen Sie es später noch einmal",
            ) {
                return Err(PaperError::LoginCurrentlyNotPossible);
            } else {
                return Err(PaperError::IncorrectCredentials);
            }
        }

        return Ok(html);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn test_result_when_login_success() {
        let html = fs::read_to_string("src/fixtures/hamburg_public/login/login_success.html")
            .expect("Something went wrong reading the file");
        let result: Result<RawLoansPage, PaperError> = Result::Ok(html.clone());
        assert_eq!(result, LoginResult::from_public_hamburg_html(html));
    }

    #[test]
    fn test_result_when_login_success_multiple_loans() {
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/login/login_success_multiple_loans.html",
        )
        .expect("Something went wrong reading the file");
        let result = LoginResult::from_public_hamburg_html(html.clone());
        assert_eq!(result, Ok(html));
    }

    #[test]
    fn test_result_when_signed_out_bad_credentials() {
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/login/login_incorrect_credentials.html",
        )
        .expect("Something went wrong reading the file");
        let result = LoginResult::from_public_hamburg_html(html.clone());
        assert_eq!(result, Err(PaperError::IncorrectCredentials));
    }

    #[test]
    fn test_result_when_login_currently_not_possible() {
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/login/login_currently_not_possible.html",
        )
        .expect("Something went wrong reading the file");
        let result = LoginResult::from_public_hamburg_html(html.clone());
        assert_eq!(result, Err(PaperError::LoginCurrentlyNotPossible));
    }
}
