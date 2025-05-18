use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use crate::model::{Balance, Charge};
use crate::scrapers::TextProvider;
use chrono::NaiveDate;
use reqwest::Client;
use scraper::Html;
use scraper::{selectable::Selectable, Selector};
pub(crate) struct BalanceScraper<'a> {
    client: &'a Client,
}

impl<'a> BalanceScraper<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        BalanceScraper { client }
    }

    pub(crate) async fn scrape_balance(&self) -> Result<Balance, PaperError> {
        let resource = HTMLResource {
            client: self.client.to_owned(),
            url: "https://www.buecherhallen.de/kontostand.html".to_string(),
        };
        let html = resource.load().await?;
        let document = scraper::Html::parse_document(html.as_str());

        Ok(BalanceScraper::balance_from(document))
    }

    fn balance_from(document: Html) -> Balance {
        let mut total = "0,0".to_string();
        if let Some(parsed_total) =
            document.get_text(r#"[href="kontostand.html"]>.navbar-submenu-account-count"#)
        {
            total = parsed_total;
        }

        let charges_selector = Selector::parse(r#".account-details-list>.account-details-item"#)
            .expect("CSS accessors must always be correct");

        let mut charges: Vec<Charge> = Vec::new();

        document.select(&charges_selector).for_each(|charge_block| {
            let mut charge = Charge::new();

            let charge_selector = Selector::parse(r#".account-details-subitem"#)
                .expect("CSS accessors must always be correct");
            charge_block
                .select(&charge_selector)
                .for_each(|charge_block_row| {
                    if let (Some(key), Some(value)) = (
                        charge_block_row.get_text(r#".account-details-label>strong"#),
                        charge_block_row.get_text(r#".account-details-value"#),
                    ) {
                        println!("{:?}", key);
                        println!("{:?}", value);

                        match key.as_str() {
                            "Erstellt:" => {
                                match BalanceScraper::date_string_to_timestamp(value.as_str()) {
                                    Ok(timestamp) => charge.timestamp = timestamp,
                                    Err(e) => println!("Error parsing timestamp: {}", e),
                                }
                            }
                            "Person(en)/Titel:" => charge.item = value,
                            "Zu bezahlen:" => match parse_euro_string(&value) {
                                Ok(amount) => charge.amount_owed = amount,
                                Err(e) => println!("Error parsing amount owed: {}", e),
                            },
                            "Bezahlt:" => match parse_euro_string(&value) {
                                Ok(amount) => charge.amount_payed = amount,
                                Err(e) => println!("Error parsing amount amount_payed: {}", e),
                            },
                            "Gebührenart:" => charge.reason = value,
                            _ => {}
                        }
                    }
                });

            charges.insert(0, charge);
        });

        charges.reverse();
        Balance { total, charges }
    }

    fn date_string_to_timestamp(date_str: &str) -> Result<i64, chrono::ParseError> {
        // Parse the date string
        let date = NaiveDate::parse_from_str(date_str, "%d.%m.%Y")?;

        // Create a NaiveDateTime at midnight
        let datetime = date.and_hms_opt(0, 0, 0).unwrap();

        // Convert to timestamp (seconds since Unix epoch)
        Ok(datetime.and_utc().timestamp())
    }
}

fn parse_euro_string(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.trim() // Remove leading/trailing whitespace
        .replace('€', "") // Remove the euro symbol
        .replace(',', ".") // Replace comma with dot
        .trim()
        .parse::<f64>() // Trim again in case there was space before/after the €
}

#[cfg(test)]
mod tests {
    use std::{fs, ops::Index};

    use super::BalanceScraper;

    #[test]
    fn it_gets_balance_from_document_when_nothing_owed() {
        let html =
            fs::read_to_string("src/fixtures/hamburg_public/account/balance_nothing_owed.html")
                .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let balance = BalanceScraper::balance_from(document);

        assert_eq!(balance.total, "0,00 €");
        assert_eq!(balance.charges.len(), 0);
    }

    #[test]
    fn it_gets_balance_from_document() {
        let html = fs::read_to_string("src/fixtures/hamburg_public/account/balance_owed.html")
            .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let balance = BalanceScraper::balance_from(document);

        assert_eq!(balance.total, "-2,00 €");
        assert_eq!(balance.charges.len(), 1);

        let charge = balance.charges.index(0);
        assert_eq!(charge.timestamp, 1724716800);
        assert_eq!(charge.item, "Clark, Polly / Tiger");
        assert_eq!(charge.reason, "Vormerkgebühr");
        assert_eq!(charge.amount_owed, 2.0);
    }

    #[test]
    fn it_gets_balance_when_multiple_charges_all_payed() {
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/account/balance_multiple_all_payed.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let balance = BalanceScraper::balance_from(document);

        assert_eq!(balance.total, "0,00 €");
        assert_eq!(balance.charges.len(), 3);

        // Check first charge
        let charge1 = balance.charges.index(0);
        assert_eq!(charge1.timestamp, 1726099200); // 2024-09-12
        assert_eq!(charge1.amount_owed, 0.0);
        assert_eq!(charge1.amount_payed, 1.0);
        assert_eq!(charge1.item, "Carle, Eric / Hallo, roter Fuchs");
        assert_eq!(charge1.reason, "Versäumnisgebühr (Rückgabe)");

        // Check second charge
        let charge2 = balance.charges.index(1);
        assert_eq!(charge2.timestamp, 1725753600); // 2024-09-08
        assert_eq!(charge2.amount_owed, 0.0);
        assert_eq!(charge2.amount_payed, 2.0);
        assert_eq!(
            charge2.item,
            "Weninger, Brigit / Pauli - die schönsten Geschichten für kühle Tag"
        );
        assert_eq!(charge2.reason, "Vormerkgebühr");

        // Check third charge
        let charge3 = balance.charges.index(2);
        assert_eq!(charge3.timestamp, 1724716800); // 2024-08-27
        assert_eq!(charge3.amount_payed, 2.0);
        assert_eq!(charge3.amount_owed, 0.0);
        assert_eq!(charge3.item, "Clark, Polly / Tiger");
        assert_eq!(charge3.reason, "Vormerkgebühr");
    }
}
