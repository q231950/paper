use scraper::Html;

use crate::{
    error::PaperError,
    model::{Loan, SearchResultDetail},
};

use super::opc4v2_13vzg6_loan_scraper::LoanScraper;

pub(crate) struct Opc4v2_13Vzg6RenewalParser;

impl Opc4v2_13Vzg6RenewalParser {
    pub(crate) fn loan_from(html: String) -> Result<Loan, PaperError> {
        let document = Html::parse_document(&html);
        let scraped_loan = LoanScraper {}.scrape_loans(document);
        if let Some(loan) = scraped_loan.loans.first() {
            Ok(Loan {
                title: loan.title.clone(),
                author: loan.author.clone(),
                can_renew: loan.can_renew,
                renewal_token: loan.renewal_token.clone(),
                renewals_count: loan.renewals_count,
                date_due: loan.date_due.clone(),
                borrowed_at: loan.borrowed_at.clone(),
                item_number: loan.item_number.clone(),
                locked_by_preorder: loan.locked_by_preorder,
                details: SearchResultDetail::new(), // todo: need to get the details of this loan
                search_result_detail_url: None,
            })
        } else {
            Err(PaperError::FailedToParseRenewedLoan)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::error::PaperError;

    use super::Opc4v2_13Vzg6RenewalParser;

    #[test]
    fn test_parses_renewed_loan() {
        let html = fs::read_to_string("src/fixtures/opc4v2_13Vzg6/renewal/renewed.html")
            .expect("Something went wrong reading the file");
        let loan = Opc4v2_13Vzg6RenewalParser::loan_from(html).unwrap();
        assert_eq!(loan.title, "Along the road : Aufzeichnungen eines Reisenden / Huxley, Aldous *1894-1963* (Oktober 2024): ");
        assert_eq!(loan.author, "");
        assert_eq!(loan.date_due, "15.04.2025");
    }

    #[test]
    fn test_parses_renewed_loan_when_failure() {
        let html = fs::read_to_string("src/fixtures/opc4v2_13Vzg6/renewal/renew_failure.html")
            .expect("Something went wrong reading the file");
        assert!(matches!(
            Opc4v2_13Vzg6RenewalParser::loan_from(html),
            Err(PaperError::FailedToParseRenewedLoan)
        ));
    }
}
