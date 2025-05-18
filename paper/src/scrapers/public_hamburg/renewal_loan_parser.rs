use crate::{
    error::PaperError,
    model::{Loan, SearchResultDetail},
};

use super::LoansScraper;

pub(crate) struct RenewalLoanParser {}

impl RenewalLoanParser {
    /// Parses the due date of a renewed loan from the given HTML
    pub(crate) fn loan_from(item_number: String, html: String) -> Result<Loan, PaperError> {
        match LoansScraper::loans_from_html(html) {
            Ok(loans) => {
                return loans
                    .loans
                    .iter()
                    .find(|loan| loan.item_number == item_number)
                    .map(|loan| Loan {
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
                    .ok_or(PaperError::FailedToRenewLoanBecauseItIsNotLoaned);
            }
            _ => return Err(PaperError::FailedToParseRenewedLoan),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::RenewalLoanParser;

    #[test]
    fn it_parses_loan_after_renewal() {
        let html =
            fs::read_to_string("src/fixtures/hamburg_public/renewal/successful_renewal.html")
                .expect("Something went wrong reading the file");
        let loan = RenewalLoanParser::loan_from("M64 070 344 4".to_string(), html).unwrap();
        assert_eq!(loan.date_due, "25.05.2024");
        assert_eq!(loan.can_renew, true);
    }
}
