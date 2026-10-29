use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use crate::model::{Availability, DataEntry, ItemAvailability, Location, SearchResultDetail};
use crate::scrapers::text_provider::TextProvider;

pub(crate) struct VuFindSearchDetailScraper {
    pub(crate) base_url: String,
}

impl VuFindSearchDetailScraper {
    /// Creates a SearchResultDetail from a VuFind record page
    ///
    /// https://stabikat.de/Record/1909924415
    pub(crate) fn search_detail_from(&self, html: Html) -> Result<SearchResultDetail, PaperError> {
        let mut detail = SearchResultDetail::new();

        detail.full_title = html
            .get_text(r#"h1[property="name"]"#)
            .or_else(|| html.get_text(".media-body h1"));
        let author_selector = Selector::parse(".media-body .author-data a")?;
        detail.medium_author = html
            .select(&author_selector)
            .next()
            .map(HTMLResource::get_text_from_element);

        // covers that VuFind loads via AJAX have no src
        detail.small_image_url = html
            .get_attribute("src", "img.recordcover")
            .and_then(|src| Url::parse(&self.base_url).ok()?.join(&src).ok())
            .filter(|url| !url.path().ends_with("/Cover/Unavailable"))
            .map(|url| url.to_string());

        // the call number of the first item in the holdings tab
        detail.signature = html.get_attribute("content", r#"meta[property="sku"]"#);

        let row_selector = Selector::parse(".media-body table tr")?;
        detail.data_entries = html
            .select(&row_selector)
            .filter_map(|row| {
                let label = row.get_text("th")?;
                let value = row.get_text("td")?;

                Some(DataEntry {
                    label: label.trim_end_matches(':').trim().to_string(),
                    value,
                })
            })
            .collect();

        detail.availability = ItemAvailability::with(Self::availabilities_from(&html)?);

        Ok(detail)
    }

    /// The items of the holdings tab, which VuFind only renders when it is the active tab
    fn availabilities_from(html: &Html) -> Result<Vec<Availability>, PaperError> {
        let item_selector = Selector::parse(r#"tr.holding-row, tr[typeof="Offer"]"#)?;

        Ok(html
            .select(&item_selector)
            .map(|item| {
                let location = Location {
                    name: Self::location_of(item).unwrap_or_default(),
                };

                let schema_availability = item
                    .get_attribute("href", r#"link[property="availability"]"#)
                    .unwrap_or_default();

                if item.get_text(".text-success").is_some()
                    || schema_availability.ends_with("/InStock")
                {
                    Availability::Available(location)
                } else if item.get_text(".text-danger").is_some()
                    || schema_availability.ends_with("/OutOfStock")
                {
                    Availability::NotAvailable(location)
                } else {
                    Availability::Unknown(location)
                }
            })
            .collect())
    }

    /// The heading above the holdings table of the item or the item's schema.org seller
    fn location_of(item: ElementRef) -> Option<String> {
        item.ancestors()
            .filter_map(ElementRef::wrap)
            .find(|element| element.value().name() == "table")
            .and_then(|table| {
                table
                    .prev_siblings()
                    .filter_map(ElementRef::wrap)
                    .find(|element| element.value().name() == "h2")
            })
            .map(HTMLResource::get_text_from_element)
            .filter(|location| !location.is_empty())
            .or_else(|| item.get_attribute("content", r#"meta[property="seller"]"#))
    }
}

#[cfg(test)]
mod tests {
    use super::VuFindSearchDetailScraper;
    use crate::model::{Availability, DataEntry, Location};
    use std::fs;

    fn search_detail_from(fixture: &str) -> crate::model::SearchResultDetail {
        let sut = VuFindSearchDetailScraper {
            base_url: "https://stabikat.de".to_string(),
        };
        let html_string =
            fs::read_to_string(fixture).expect("Something went wrong reading the file");
        let html = scraper::Html::parse_document(html_string.as_str());

        sut.search_detail_from(html).unwrap()
    }

    #[test]
    fn it_parses_the_record() {
        let search_detail = search_detail_from("src/fixtures/vufind/record/record_holdings.html");

        assert_eq!(
            search_detail.full_title,
            Some("Trauriger Tiger : Roman".to_string())
        );
        assert_eq!(
            search_detail.medium_author,
            Some("Sinno, Neige".to_string())
        );
        assert_eq!(
            search_detail.small_image_url,
            Some("https://stabikat.de/Cover/Show?size=medium&isbn=9783630877662".to_string())
        );
        assert_eq!(search_detail.signature, Some("1 A 1234567".to_string()));
    }

    #[test]
    fn it_parses_the_bibliographic_details() {
        let search_detail = search_detail_from("src/fixtures/vufind/record/record_holdings.html");

        assert_eq!(search_detail.data_entries.len(), 5);
        assert_eq!(
            search_detail.data_entries[0],
            DataEntry {
                label: "Main Author".to_string(),
                value: "Sinno, Neige (VerfasserIn)".to_string()
            }
        );
        assert_eq!(
            search_detail.data_entries[4],
            DataEntry {
                label: "ISBN".to_string(),
                value: "9783630877662".to_string()
            }
        );
    }

    #[test]
    fn it_parses_the_holdings() {
        let search_detail = search_detail_from("src/fixtures/vufind/record/record_holdings.html");

        assert_eq!(
            search_detail.availability.availabilities,
            vec![
                Availability::Available(Location {
                    name: "Haus Potsdamer Straße".to_string()
                }),
                Availability::NotAvailable(Location {
                    name: "Haus Potsdamer Straße".to_string()
                }),
                Availability::Unknown(Location {
                    name: "Haus Unter den Linden".to_string()
                }),
            ]
        );
    }

    #[test]
    fn it_parses_a_record_without_holdings() {
        let search_detail =
            search_detail_from("src/fixtures/vufind/record/record_without_holdings.html");

        assert_eq!(
            search_detail.full_title,
            Some(
                "Animals and epidemics : interspecies entanglements in historical perspective"
                    .to_string()
            )
        );
        assert_eq!(search_detail.small_image_url, None);
        assert_eq!(search_detail.signature, None);
        assert_eq!(search_detail.data_entries.len(), 2);
        assert!(search_detail.availability.availabilities.is_empty());
    }
}
