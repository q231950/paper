use reqwest::Client;

use super::daia::DaiaResponse;
use super::marc_record::{Holding, MarcRecord};
use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use crate::model::{Availability, DataEntry, ItemAvailability, Location, SearchResultDetail};

pub(crate) struct K10plusSruSearchDetailScraper {
    pub(crate) database: String,
}

impl K10plusSruSearchDetailScraper {
    /// Creates the SearchResultDetail of the SRU record in `xml` with the availability of its
    /// copies from DAIA, the availability is unknown when DAIA fails
    pub(crate) async fn search_detail(
        &self,
        client: &Client,
        xml: String,
    ) -> Result<SearchResultDetail, PaperError> {
        let ppn = Self::ppn_from(&xml)?;
        let daia = self.daia(client, &ppn).await;

        self.search_detail_from(&xml, daia.as_ref())
    }

    async fn daia(&self, client: &Client, ppn: &str) -> Option<DaiaResponse> {
        let json = HTMLResource {
            client: client.to_owned(),
            url: DaiaResponse::url(&self.database, ppn),
        }
        .load()
        .await
        .ok()?;

        serde_json::from_str(&json).ok()
    }

    fn ppn_from(xml: &str) -> Result<String, PaperError> {
        let document = roxmltree::Document::parse(xml)
            .map_err(|_| PaperError::ParseErrorSearchResultDetail)?;

        MarcRecord::records_in(&document)
            .first()
            .and_then(|record| record.ppn())
            .ok_or(PaperError::ParseErrorSearchResultDetail)
    }

    fn search_detail_from(
        &self,
        xml: &str,
        daia: Option<&DaiaResponse>,
    ) -> Result<SearchResultDetail, PaperError> {
        let document = roxmltree::Document::parse(xml)
            .map_err(|_| PaperError::ParseErrorSearchResultDetail)?;
        let records = MarcRecord::records_in(&document);
        let record = records
            .first()
            .ok_or(PaperError::ParseErrorSearchResultDetail)?;
        let holdings = record.holdings();

        let mut detail = SearchResultDetail::new();
        detail.full_title = record.full_title();
        detail.medium_author = record.author();
        detail.small_image_url = record.cover_url();
        detail.signature = holdings
            .iter()
            .find_map(|holding| holding.call_number.clone());
        detail.data_entries = Self::data_entries_from(record, &holdings);
        detail.availability = ItemAvailability::with(
            holdings
                .iter()
                .map(|holding| {
                    let location = Location {
                        name: holding.name(),
                    };

                    match daia {
                        Some(daia) => daia.availability(holding.epn.as_deref(), location),
                        None => Availability::Unknown(location),
                    }
                })
                .collect(),
        );

        Ok(detail)
    }

    fn data_entries_from(record: &MarcRecord, holdings: &[Holding]) -> Vec<DataEntry> {
        let mut subjects: Vec<String> = Vec::new();
        for subject in record.subfields("650", 'a') {
            if !subjects.contains(&subject) {
                subjects.push(subject);
            }
        }

        let call_numbers: Vec<String> = holdings
            .iter()
            .filter_map(|holding| holding.call_number.clone())
            .collect();

        vec![
            ("Titel", record.full_title()),
            ("Verfasser", Some(record.subfields("100", 'a').join("; "))),
            (
                "Weitere Personen",
                Some(record.subfields("700", 'a').join("; ")),
            ),
            ("Ausgabe", record.subfield("250", 'a')),
            ("Veröffentlicht", record.publication()),
            ("Umfang", record.subfield("300", 'a')),
            ("ISBN", Some(record.subfields("020", 'a').join(", "))),
            ("Sprache", Some(record.subfields("041", 'a').join(", "))),
            ("Schlagwörter", Some(subjects.join("; "))),
            ("Signatur", Some(call_numbers.join(", "))),
            ("PPN", record.ppn()),
        ]
        .into_iter()
        .filter_map(|(label, value)| {
            value
                .filter(|value| !value.is_empty())
                .map(|value| DataEntry {
                    label: label.to_string(),
                    value,
                })
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::K10plusSruSearchDetailScraper;
    use crate::error::PaperError;
    use crate::model::{Availability, DataEntry, Location, SearchResultDetail};
    use crate::scrapers::k10plus_sru::daia::DaiaResponse;
    use std::fs;

    fn search_detail_from(record: &str, daia: Option<&str>) -> SearchResultDetail {
        let sut = K10plusSruSearchDetailScraper {
            database: "opac-de-1".to_string(),
        };
        let xml = fs::read_to_string(format!("src/fixtures/k10plus_sru/record/{record}.xml"))
            .expect("Something went wrong reading the file");
        let daia: Option<DaiaResponse> = daia.map(|daia| {
            let json = fs::read_to_string(format!("src/fixtures/k10plus_sru/daia/{daia}.json"))
                .expect("Something went wrong reading the file");
            serde_json::from_str(&json).unwrap()
        });

        sut.search_detail_from(&xml, daia.as_ref()).unwrap()
    }

    fn location(name: &str) -> Location {
        Location {
            name: name.to_string(),
        }
    }

    #[test]
    fn it_parses_the_record() {
        let detail = search_detail_from("record_print", Some("daia_available"));

        assert_eq!(
            detail.full_title,
            Some(
                "Trauriger Tiger / Neige Sinno ; aus dem Französischen von Michaela Meßner"
                    .to_string()
            )
        );
        assert_eq!(detail.medium_author, Some("Sinno, Neige".to_string()));
        assert_eq!(detail.signature, Some("10 A 186926".to_string()));
        assert_eq!(detail.small_image_url, None);
    }

    #[test]
    fn it_parses_the_data_entries() {
        let detail = search_detail_from("record_print", Some("daia_available"));
        let entry = |label: &str, value: &str| DataEntry {
            label: label.to_string(),
            value: value.to_string(),
        };

        assert!(detail
            .data_entries
            .contains(&entry("Veröffentlicht", "München : dtv, 2024")));
        assert!(detail
            .data_entries
            .contains(&entry("Weitere Personen", "Meßner, Michaela")));
        assert!(detail
            .data_entries
            .contains(&entry("ISBN", "9783423284226")));
        assert!(detail
            .data_entries
            .contains(&entry("Signatur", "10 A 186926")));
        assert!(detail.data_entries.contains(&entry("PPN", "1886167923")));
        // the record has no edition and no subjects
        assert!(!detail
            .data_entries
            .iter()
            .any(|entry| entry.label == "Ausgabe" || entry.label == "Schlagwörter"));
    }

    #[test]
    fn it_lists_each_subject_once() {
        let detail = search_detail_from("record_on_loan", Some("daia_on_loan"));
        let subjects = detail
            .data_entries
            .iter()
            .find(|entry| entry.label == "Schlagwörter")
            .map(|entry| entry.value.split("; ").collect::<Vec<_>>())
            .unwrap();

        assert_eq!(subjects.first(), Some(&"Physics"));
        assert!(subjects.contains(&"Maschinelles Lernen"));
        let mut unique = subjects.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), subjects.len());
    }

    #[test]
    fn it_parses_an_available_copy() {
        let detail = search_detail_from("record_print", Some("daia_available"));

        assert_eq!(
            detail.availability.availabilities,
            vec![Availability::Available(location("10 A 186926"))]
        );
    }

    #[test]
    fn it_parses_a_copy_on_loan_and_the_cover() {
        let detail = search_detail_from("record_on_loan", Some("daia_on_loan"));

        assert_eq!(
            detail.availability.availabilities,
            vec![Availability::NotAvailable(location("1 B 206295"))]
        );
        assert_eq!(
            detail.small_image_url,
            Some("https://www.dietmardreier.de/annot/426F6F6B446174617C7C393738303639313230333932387C7C434F50.jpg?sq=1".to_string())
        );
    }

    #[test]
    fn it_parses_an_online_copy() {
        let detail = search_detail_from("record_online", Some("daia_online"));

        assert_eq!(
            detail.availability.availabilities,
            vec![Availability::Unknown(location("Online"))]
        );
        assert_eq!(detail.signature, None);
    }

    #[test]
    fn it_has_an_unknown_availability_without_daia() {
        let detail = search_detail_from("record_print", None);

        assert_eq!(
            detail.availability.availabilities,
            vec![Availability::Unknown(location("10 A 186926"))]
        );
    }

    #[test]
    fn it_has_an_unknown_availability_when_daia_fails() {
        let detail = search_detail_from("record_print", Some("daia_unknown_ppn"));

        assert_eq!(
            detail.availability.availabilities,
            vec![Availability::Unknown(location("10 A 186926"))]
        );
    }

    #[test]
    fn it_reads_the_ppn() {
        let xml = fs::read_to_string("src/fixtures/k10plus_sru/record/record_print.xml")
            .expect("Something went wrong reading the file");

        assert_eq!(
            K10plusSruSearchDetailScraper::ppn_from(&xml),
            Ok("1886167923".to_string())
        );
        assert_eq!(
            K10plusSruSearchDetailScraper::ppn_from("<html></html>"),
            Err(PaperError::ParseErrorSearchResultDetail)
        );
    }
}
