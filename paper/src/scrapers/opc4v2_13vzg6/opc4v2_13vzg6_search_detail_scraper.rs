use scraper::Html;

use crate::model::{Availability, DataEntry, ItemAvailability, Location};
use crate::scrapers::text_provider::TextProvider;

use crate::{error::PaperError, model::SearchResultDetail};

pub(crate) struct Opc4v2_13Vzg6SearchDetailScraper {}

impl Opc4v2_13Vzg6SearchDetailScraper {
    pub(crate) async fn search_detail_from(
        &self,
        html: Html,
    ) -> Result<SearchResultDetail, PaperError> {
        let mut detail = SearchResultDetail::new();
        let content_table_selector = scraper::Selector::parse(r#"body > table > tbody > tr:nth-child(7) > td.cnt > table > tbody > tr:nth-child(4) > td > table > tbody > tr:nth-child(1) > td:nth-child(2) > table > tbody > tr"#).unwrap();
        let rows = html.select(&content_table_selector);

        let mut entries = Vec::new();
        let mut availabilities = Vec::new();

        rows.for_each(|row| {
            let label = row.get_text(".preslabel");
            println!("label: {:?}", label);

            let value = row.get_text(".presvalue");
            println!("value: {:?}", value);

            // set the title of the book
            if label == Some("Titel:".to_string()) {
                detail.full_title = value.clone();
            }

            if label == Some("Standort:".to_string()) {
                println!("Standort: {:?}", value.clone());
                availabilities.push(Availability::Unknown(Location {
                    name: value.clone().unwrap_or("".to_string()),
                }));
            }

            // add all key value pairs to the detail
            if let (Some(label), Some(value)) = (label, value) {
                entries.push(DataEntry { label, value })
            }
        });

        detail.data_entries = entries;
        detail.availability = ItemAvailability { availabilities };

        Ok(detail)
    }
}

#[cfg(test)]
mod tests {
    use super::Opc4v2_13Vzg6SearchDetailScraper;
    use std::fs;

    #[tokio::test]
    async fn it_parses_search_detail_when_book_sbb() {
        let sut = Opc4v2_13Vzg6SearchDetailScraper {};
        let html_string =
            fs::read_to_string("src/fixtures/opc4v2_13Vzg6/search/katalog_detail_book_sbb.html")
                .expect("Something went wrong reading the file");
        let html = scraper::Html::parse_document(html_string.as_str());
        let search_detail = sut.search_detail_from(html).await.unwrap();

        assert_eq!(
            search_detail.full_title,
            Some("Paul Foot : a life in politics / Margaret Renn".to_string())
        );

        println!("{:?}", search_detail.data_entries);

        assert_eq!(search_detail.data_entries.len(), 18);
        assert_eq!(search_detail.availability.availabilities.len(), 1);
    }

    #[tokio::test]
    async fn it_parses_search_detail_when_book_gbv_hh() {
        let sut = Opc4v2_13Vzg6SearchDetailScraper {};
        let html_string =
            fs::read_to_string("src/fixtures/opc4v2_13Vzg6/search/katalog_detail_book_gbv_hh.html")
                .expect("Something went wrong reading the file");
        let html = scraper::Html::parse_document(html_string.as_str());
        let search_detail = sut.search_detail_from(html).await.unwrap();

        assert_eq!(
            search_detail.full_title,
            Some(
                "Trauriger Tiger / Neige Sinno ; aus dem Französischen von Michaela Meßner"
                    .to_string()
            )
        );

        assert_eq!(search_detail.data_entries.len(), 16);

        assert_eq!(search_detail.availability.availabilities.len(), 1);
    }
}
