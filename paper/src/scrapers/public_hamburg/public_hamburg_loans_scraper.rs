use scraper::Selector;

use crate::{
    error::PaperError,
    model::{Loan, Loans},
    scrapers::TextProvider,
};

pub(crate) struct LoansScraper {}

impl LoansScraper {
    /// Parses the Loans from the given HTML
    pub(crate) fn loans_from_html(html: String) -> Result<Loans, PaperError> {
        let document = scraper::Html::parse_document(html.as_ref());
        let mut loans: Vec<Loan> = Vec::new();

        let loan_row_selector =
            Selector::parse(r#"li[class="search-results-item loans-search-results-item"]"#)?;
        document.select(&loan_row_selector).for_each(|loan_row| {
                let mut loan = Loan::new();

                if let Some(title) = loan_row.get_text(r#".search-results-title > a"#) {
                    loan.title = title;
                }

                if let Some(search_result_detail_url) =
                    loan_row.get_attribute("href", r#".search-results-title > a"#)
                {
                    loan.search_result_detail_url = Some(search_result_detail_url);
                }

                if let Some(due_date) =
                    loan_row.get_text(r#".search-results-actions .loans-actions-info > p > strong"#)
                {
                    loan.date_due = due_date;
                }

                if let Some(renewals_count) =
                    loan_row.get_text(r#".search-results-actions .loans-actions-info > p:nth-child(2)"#)
                {
                    loan.renewals_count = match renewals_count.as_str() {
                        "Einmal verlängert" => 1,
                        "Zweimal verlängert" => 2,
                        "Dreimal verlängert" => 3,
                        _ => 0,
                    }
                }

                // This shows "Heute verlängert oder ausgeliehen." directly after renewal.
                if let Some(renew_button_text) = loan_row.get_text(
                    r#".search-results-actions .loans-actions-form > div > [class="button button-small"]"#
                ) {
                    loan.can_renew = renew_button_text == "Verlängern";
                }

                if let Ok(locked_by_preorder_selector) =
                    Selector::parse(r#".search-results-actions .loans-actions-info > p"#)
                {
                    for info in loan_row.select(&locked_by_preorder_selector) {
                        for child in info.children() {
                            if child.value().is_text() {
                                if let Some(y) = child.value().as_text() {
                                    let x = format!("{:?}", y);
                                    if x.as_str() == "\"Medium vorgemerkt\"" {
                                        loan.locked_by_preorder = true;
                                    }
                                }
                            }
                        }
                    }
                }

                if let Ok(loan_details_selector) =
                    Selector::parse(r#".loans-details > .loans-details-item"#) {

                for detail in loan_row.select(&loan_details_selector) {
                    let key = detail.get_text(r#".loans-details-label"#);
                    let value = detail.get_text(r#".loans-details-value"#);

                    if let (Some(k), Some(v)) = (key, value) {
                        if k == "Mediennummer:" {
                            loan.item_number = v.clone();
                        }

                        if k == "Ausgeliehen am:" {
                            loan.borrowed_at = v.clone();
                        }

                        if k == "Standort:" {
                            // location
                        }

                        //println!("key {:?}", k);
                        //println!("value {:?}", v);
                    }
                }
                    }

                loans.push(loan);
            });

        return Ok(Loans { loans });
    }
}

#[cfg(test)]
mod tests {
    use super::LoansScraper;
    use crate::model::{ItemAvailability, Loan, Loans, SearchResultDetail};
    use std::fs;

    #[test]
    fn it_parses_loans_from_login_success() {
        let html = fs::read_to_string("src/fixtures/hamburg_public/login/login_success.html")
            .expect("Something went wrong reading the file");
        let loans = LoansScraper::loans_from_html(html)
            .expect("Parsing loans should work with the given html");

        assert_eq!(
            loans,
            Loans {
                loans: vec![
                    Loan {
                        title: "Bibi Blocksberg - Die Prinzessinnen von Thunder".to_string(),
                        author: "".to_string(),
                        can_renew: true,
                        renewal_token: None,
                        renewals_count: 0,
                        date_due: "25.05.2024".to_string(),
                        borrowed_at: "27.04.2024".to_string(),
                        item_number: "M62 193 690 6".to_string(),
                        locked_by_preorder: false,
                        details: SearchResultDetail {
                            medium_title: None,
                            medium_author: None,
                            full_title: None,
                            small_image_url: None,
                            signature: None,
                            data_entries: vec![],
                            hint: None,
                            availability: ItemAvailability {
                                availabilities: vec![],
                            },
                        },
                        search_result_detail_url: Some(
                            "suchergebnis-detail/medium/T020062902.html".to_string()
                        ),
                    },
                    Loan {
                        title: "Sternenschweif / 1 Sternenschweif - geheimnisvo".to_string(),
                        author: "".to_string(),
                        can_renew: true,
                        renewal_token: None,
                        renewals_count: 0,
                        date_due: "25.05.2024".to_string(),
                        borrowed_at: "27.04.2024".to_string(),
                        item_number: "M63 130 726 6".to_string(),
                        locked_by_preorder: false,
                        details: SearchResultDetail {
                            medium_title: None,
                            medium_author: None,
                            full_title: None,
                            small_image_url: None,
                            signature: None,
                            data_entries: vec![],
                            hint: None,
                            availability: ItemAvailability {
                                availabilities: vec![],
                            },
                        },
                        search_result_detail_url: Some(
                            "suchergebnis-detail/medium/T021001401.html".to_string()
                        ),
                    },
                    Loan {
                        title: "Der kleine Wassermann".to_string(),
                        author: "".to_string(),
                        can_renew: true,
                        renewal_token: None,
                        renewals_count: 1,
                        date_due: "31.05.2024".to_string(),
                        borrowed_at: "27.04.2024".to_string(),
                        item_number: "M64 058 812 7".to_string(),
                        locked_by_preorder: false,
                        details: SearchResultDetail {
                            medium_title: None,
                            medium_author: None,
                            full_title: None,
                            small_image_url: None,
                            signature: None,
                            data_entries: vec![],
                            hint: None,
                            availability: ItemAvailability {
                                availabilities: vec![],
                            },
                        },
                        search_result_detail_url: Some(
                            "suchergebnis-detail/medium/T019494523.html".to_string()
                        ),
                    },
                    Loan {
                        title: "Bambino-LÜK / [...] Tiere im Zoo : Alter 3 - 5".to_string(),
                        author: "".to_string(),
                        can_renew: true,
                        renewal_token: None,
                        renewals_count: 1,
                        date_due: "04.06.2024".to_string(),
                        borrowed_at: "27.04.2024".to_string(),
                        item_number: "M64 070 344 4".to_string(),
                        locked_by_preorder: false,
                        details: SearchResultDetail {
                            medium_title: None,
                            medium_author: None,
                            full_title: None,
                            small_image_url: None,
                            signature: None,
                            data_entries: vec![],
                            hint: None,
                            availability: ItemAvailability {
                                availabilities: vec![],
                            },
                        },
                        search_result_detail_url: Some(
                            "suchergebnis-detail/medium/T010693899.html".to_string()
                        ),
                    },
                ]
            }
        );
    }

    #[test]
    fn it_parses_loans_from_login_success_info_notice() {
        let html =
            fs::read_to_string("src/fixtures/hamburg_public/login/login_success_info_notice.html")
                .expect("Something went wrong reading the file");
        let loans = LoansScraper::loans_from_html(html)
            .expect("Parsing loans should work with the given html");

        assert_eq!(
            loans,
            Loans {
                loans: vec![Loan {
                    title: "MiniLÜK / […] Lösungsgerät".to_string(),
                    author: "".to_string(),
                    can_renew: true,
                    renewal_token: None,
                    renewals_count: 0,
                    date_due: "10.05.2025".to_string(),
                    borrowed_at: "12.04.2025".to_string(),
                    item_number: "M58 385 945 2".to_string(),
                    locked_by_preorder: false,
                    details: SearchResultDetail::new(),
                    search_result_detail_url: Some(
                        "suchergebnis-detail/medium/T010694188.html".to_string()
                    ),
                }]
            }
        );
    }

    #[test]
    fn it_parses_loans_from_loans_section() {
        let html = fs::read_to_string("src/fixtures/hamburg_public/account/loans.html")
            .expect("Something went wrong reading the file");
        let loans = LoansScraper::loans_from_html(html)
            .expect("Parsing loans should work with the given html");
        assert_eq!(loans.loans.iter().count(), 4)
    }

    #[test]
    fn it_parses_overdue_loans() {
        let html = fs::read_to_string("src/fixtures/hamburg_public/account/loans_overdue.html")
            .expect("Something went wrong reading the file");
        let loans = LoansScraper::loans_from_html(html)
            .expect("Parsing overdue loans should work with the given html");

        assert_eq!(loans.loans.len(), 9);

        let overdue_loan = &loans.loans[0];
        assert_eq!(overdue_loan.title, "Hallo, roter Fuchs");
        assert_eq!(overdue_loan.date_due, "10.09.2024");
        assert_eq!(overdue_loan.renewals_count, 1);
        assert_eq!(overdue_loan.can_renew, true);
        assert_eq!(overdue_loan.locked_by_preorder, false);
        assert_eq!(overdue_loan.item_number, "M63 508 806 1");
        assert_eq!(overdue_loan.borrowed_at, "20.07.2024");
    }

    #[test]
    fn it_parses_preordered_status() {
        let html = fs::read_to_string("src/fixtures/hamburg_public/account/loans_preordered.html")
            .expect("Something went wrong reading the file");
        let loans = LoansScraper::loans_from_html(html)
            .expect("The preordered status of a loan should be parsed");
        assert_eq!(loans.loans[0].locked_by_preorder, true)
    }
}
