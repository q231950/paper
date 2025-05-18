use scraper::Html;

use crate::model::{Loan, Loans};

pub(crate) struct LoanScraper {}

impl LoanScraper {
    pub(crate) fn new() -> Self {
        LoanScraper {}
    }

    // Assuming you will have a method to scrape loans
    pub(crate) fn scrape_loans(&self, html: Html) -> Loans {
        let mut loans = Loans::new();

        // get all 5 tbody's
        let selector = scraper::Selector::parse(
            r#"#loansForm > div.bodydiv > table[class='resultset'] > tbody > tr"#,
        )
        .unwrap();
        // get all table rows
        let rows = html.select(&selector);

        let mut loan_title: Option<String> = None;
        let mut loan_renewal_token: Option<String> = None;
        let mut renew_disabled: Option<String> = None;

        // these are 5 rows, interesting is every odd one
        for row in rows {
            // checkbox td selection
            let checkbox_selector = scraper::Selector::parse(r#"td.rec_checkbox > input"#).unwrap();
            let checkbox: Vec<_> = row.select(&checkbox_selector).collect();

            if let Some(checkbox_td) = checkbox.first() {
                if let Some(renewal_token) = checkbox_td.value().attr("value") {
                    loan_renewal_token = Some(renewal_token.to_string());
                    println!("renew checkbox value: {:?}", renewal_token);
                }

                if let Some(tmp_renew_disabled) = checkbox_td.value().attr("disabled") {
                    renew_disabled = Some(tmp_renew_disabled.to_string());
                }
            }

            let record_selector = scraper::Selector::parse(r#"td.rec_title"#).unwrap();
            let record: Vec<_> = row.select(&record_selector).collect();
            if let Some(record) = record.first() {
                println!("record: {:?}", record.value());

                let record_tbodies = scraper::Selector::parse(r#"div > table > tbody"#).unwrap();
                let tbodies: Vec<_> = record.select(&record_tbodies).collect();

                for tbody in tbodies {
                    let spans_selector =
                        scraper::Selector::parse(r#"tr > td.rec_data > span"#).unwrap();
                    let spans: Vec<_> = tbody.select(&spans_selector).collect();

                    if spans.len() == 1 {
                        if let Some(child) = spans.first() {
                            if let Some(title) = child
                                .first_child()
                                .and_then(|child| child.value().as_text().map(|s| s.to_string()))
                            {
                                println!("title: {:?}", title);
                                loan_title = Some(title);
                            }
                        }
                    }

                    if spans.len() % 2 == 0 {
                        let mut pairs = Vec::new();
                        for chunk in spans.chunks(2) {
                            if let [first, second] = chunk {
                                let first_text = first
                                    .first_child()
                                    .and_then(|child| child.value().as_text());
                                let second_text = second
                                    .first_child()
                                    .and_then(|child| child.value().as_text());
                                println!("first_text: {:?}", first_text);
                                println!("second_text: {:?}", second_text);
                                pairs.push((first_text, second_text));
                            }
                        }

                        if let Some(title) = loan_title.clone() {
                            let mut loan = Loan::new();
                            loan.can_renew = true; // this is later set to false after the value was determined
                            loan.title = title;

                            for (first, second) in pairs {
                                if let Some(first_text) = first {
                                    if first_text.to_lowercase().contains("frist") {
                                        if let Some(second_text) = second {
                                            loan.date_due =
                                                second_text.parse().unwrap_or("".to_string());
                                        }
                                    }
                                    if first_text.to_lowercase().contains("signatur") {
                                        if let Some(second_text) = second {
                                            loan.item_number =
                                                second_text.parse().unwrap_or("".to_string());
                                        }
                                    }

                                    if first_text.to_lowercase().contains("verlängerung") {
                                        if let Some(second_text) = second {
                                            loan.renewals_count = second_text.parse().unwrap_or(0);
                                        }
                                    }
                                }
                            }

                            if let Some(token) = loan_renewal_token.clone() {
                                print!("renewal token {:?}", token);
                                loan.renewal_token = Some(token);
                            }

                            if let Some(renew_disabled) = renew_disabled.clone() {
                                print!("renew disabled: {:?}", renew_disabled);
                                loan.can_renew = false;
                            }
                            loans.loans.push(loan);
                        }
                    }
                }
            }
        }

        return loans;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use scraper::Html;

    use crate::scrapers::opc4v2_13vzg6::opc4v2_13vzg6_loan_scraper::LoanScraper;

    #[tokio::test]
    async fn test_loan_scraper() {
        let html = fs::read_to_string("src/fixtures/opc4v2_13Vzg6/login/loans_gbv_sub_hh.html")
            .expect("Could not read file");
        let html = Html::parse_document(&html);
        let scraper = LoanScraper::new();
        let loans = scraper.scrape_loans(html);

        assert_eq!(loans.loans.len(), 3);
        assert_eq!(loans.loans.first().unwrap().can_renew, true);
        assert_eq!(loans.loans.first().unwrap().title, "Along the road : Aufzeichnungen eines Reisenden / Huxley, Aldous *1894-1963* (Oktober 2024): ");
        assert_eq!(loans.loans.first().unwrap().renewals_count, 2);
        assert_eq!(loans.loans.last().unwrap().title, "Mitbestimmung von Kindern : Grundlagen für Unterricht, Schule und Hochschule / Grüning, Miriam (2022): ");
    }
}
