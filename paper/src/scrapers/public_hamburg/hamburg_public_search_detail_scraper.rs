use crate::api::APIClient;
use crate::model::Availability;
use crate::model::DataEntry;
use crate::model::ItemAvailability;
use crate::model::Location;
use crate::model::SearchResultDetail;
use crate::scrapers::text_provider::TextProvider;
use scraper::Selector;

#[derive(uniffi::Object)]
pub(crate) struct HamburgPublicSearchDetailScraper {}

impl HamburgPublicSearchDetailScraper {
    pub(crate) async fn search_result_detail_from(
        document: scraper::Html,
    ) -> Option<SearchResultDetail> {
        let mut detail = SearchResultDetail::new();

        detail.medium_title = document.get_text(r#".medium-detail-title"#);
        detail.medium_author = document.get_text(r#".medium-detail-author > a"#);

        if let Some(url1) =
            document.get_attribute("data-src", r#"img[class="b-lazy img-lazyload"]"#)
        {
            match APIClient::ping_url(url1.as_str()).await {
                Ok(status) => match status {
                    200 => {
                        detail.small_image_url = Some(url1);
                    }
                    _ => {
                        detail.small_image_url = document
                            .get_attribute("data-alt-src", r#"img[class="b-lazy img-lazyload"]"#);
                    }
                },
                Err(e) => println!("Error checking site: {}", e),
            }
        }

        if let Ok(detail_data_selector) =
            Selector::parse(r#"div[class="medium-detail-data medium-detail-data-cols"] > ul > li"#)
        {
            let mut entries = Vec::new();
            let detail_data_entries = document.select(&detail_data_selector);

            for entry in detail_data_entries.into_iter() {
                let label = entry
                    .get_text(r#".medium-detail-item-label"#)
                    .unwrap_or_default();

                let value = entry
                    .get_text(r#".medium-detail-item-value"#)
                    .unwrap_or_default();

                if label != "" {
                    entries.push(DataEntry { label, value })
                } else {
                    if let Some(d) = entries.pop() {
                        let new_value = if value.contains("Achtung!") {
                            detail.hint = Some(value);
                            d.value
                        } else {
                            d.value + " > " + &value
                        };

                        entries.push(DataEntry {
                            label: d.label,
                            value: new_value,
                        })
                    }
                }

                if let Some(d) = entries.last() {
                    if d.label.contains("Signatur") {
                        detail.signature = Some(d.value.clone());
                    }
                }
            }

            detail.data_entries = entries;

            let availabilities = HamburgPublicSearchDetailScraper::parse_availabilities(document);
            detail.availability = ItemAvailability::with(availabilities);
        }
        Some(detail)
    }

    fn parse_availabilities(html: scraper::Html) -> Vec<Availability> {
        let items_selector = Selector::parse("li.medium-availability-item").unwrap();
        let location_selector =
            Selector::parse(".medium-availability-item-title-location").unwrap();

        let mut availabilities = Vec::new();

        for item in html.select(&items_selector) {
            // Get location name
            let location_name = item
                .select(&location_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let location = Location {
                name: location_name,
            };

            // Determine availability based on class
            let availability = if item.value().classes().any(|c| c == "record-available") {
                Availability::Available(location)
            } else if item.value().classes().any(|c| c == "record-not-available") {
                Availability::NotAvailable(location)
            } else {
                Availability::Unknown(location)
            };

            availabilities.push(availability);
        }

        availabilities
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::HamburgPublicSearchDetailScraper;

    #[tokio::test]
    async fn it_parses() {
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/suchergebnis_detail_no_image.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_detail =
            HamburgPublicSearchDetailScraper::search_result_detail_from(document)
                .await
                .unwrap();

        assert_eq!(
            search_result_detail.medium_title,
            Some("Mein Taschenlampen-Entdeckerbuch - wilde Tiere".to_string())
        );
        assert_eq!(
            search_result_detail.medium_author,
            Some("Hessels, Sandra C.".to_string())
        );
    }

    #[tokio::test]
    async fn it_parses_image() {
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/suchergebnis_detail_with_image.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_detail =
            HamburgPublicSearchDetailScraper::search_result_detail_from(document)
                .await
                .unwrap();
        assert_eq!(
            search_result_detail.small_image_url,
            Some("https://www.hugendubel.info/annotstream/9783836961592/COP".to_string())
        );

        assert_eq!(
            search_result_detail.signature,
            Some("Kinder > Sachmedien > Tiere Dinosaurier".to_string())
        );

        assert_eq!(
            search_result_detail.data_entries[1].label,
            "Originaltitel:".to_string()
        );
        assert_eq!(
            search_result_detail.data_entries[1].value,
            "Inventaire illustré des dinosaures".to_string()
        );

        assert_eq!(search_result_detail.data_entries.len(), 12);

        assert_eq!(search_result_detail.hint, Some("Achtung! Neue Medien können einen abweichenden Standort aufweisen. Bitte prüfen Sie den genauen Standort in der Verfügbarkeitsanzeige.".to_string()));
    }
}
