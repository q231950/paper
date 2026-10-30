use reqwest::{Client, Url};

use super::marc_record::MarcRecord;
use super::sru_request;
use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use crate::model::SearchResultList;
use crate::model::SearchResultListItem;

const SRU_NAMESPACE: &str = "http://www.loc.gov/zing/srw/";

pub(crate) struct K10plusSruSearchScraper {
    pub(crate) base_url: String,
    pub(crate) database: String,
}

impl K10plusSruSearchScraper {
    /// https://sru.k10plus.de/opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xall%3D%22tiger%22&startRecord=1&maximumRecords=20&recordSchema=marcxml
    pub async fn search(
        &self,
        text: &str,
        next_page_url: Option<String>,
    ) -> Result<SearchResultList, PaperError> {
        let url = match next_page_url {
            Some(next_page_url) => Url::parse(&format!("{}/{next_page_url}", self.base_url)).ok(),
            None => sru_request::search_url(&self.base_url, &self.database, text),
        }
        .ok_or(PaperError::ErrorParsingUrl)?;

        let xml = HTMLResource {
            client: Client::new(),
            url: url.to_string(),
        }
        .load()
        .await?;

        self.search_result_list_from(text.to_string(), &xml, &url)
    }
}

impl K10plusSruSearchScraper {
    /// Creates a SearchResultList from an SRU searchRetrieve response loaded from `url`
    fn search_result_list_from(
        &self,
        text: String,
        xml: &str,
        url: &Url,
    ) -> Result<SearchResultList, PaperError> {
        let document = roxmltree::Document::parse(xml).map_err(|_| PaperError::SearchFailed)?;

        // SRU reports errors such as an unsupported query as diagnostics instead of results
        let result_count = document
            .descendants()
            .find(|node| node.has_tag_name((SRU_NAMESPACE, "numberOfRecords")))
            .and_then(|node| node.text())
            .and_then(|text| text.trim().parse::<u32>().ok())
            .ok_or(PaperError::SearchFailed)?;

        let last_record_position = document
            .descendants()
            .filter(|node| node.has_tag_name((SRU_NAMESPACE, "recordPosition")))
            .filter_map(|node| node.text())
            .filter_map(|text| text.trim().parse::<u32>().ok())
            .max();

        let items = MarcRecord::records_in(&document)
            .iter()
            .map(|record| self.search_result_list_item_from(record))
            .collect();

        let next_page_url = last_record_position
            .filter(|position| *position < result_count)
            .map(|position| sru_request::with_start_record(url, position + 1))
            .map(|url| sru_request::relative_path(&url));

        Ok(SearchResultList {
            text,
            next_page_url,
            result_count,
            items,
        })
    }

    fn search_result_list_item_from(&self, record: &MarcRecord) -> SearchResultListItem {
        let mut list_item = SearchResultListItem::new();
        list_item.title = record.title();

        // like the GBV OPACs, e.g. "Sinno, Neige - München : dtv, 2024"
        let subtitle = vec![record.author(), record.publication()]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" - ");
        list_item.subtitle = (!subtitle.is_empty()).then_some(subtitle);
        list_item.cover_image_url = record.cover_url();

        if let Some(ppn) = record.ppn() {
            list_item.detail_url = sru_request::record_url(&self.base_url, &self.database, &ppn)
                .map(|url| sru_request::relative_path(&url));
            list_item.identifier = ppn.clone();
            list_item.item_number = Some(ppn);
        }

        list_item
    }
}

#[cfg(test)]
mod tests {
    use super::K10plusSruSearchScraper;
    use crate::error::PaperError;
    use crate::model::SearchResultList;
    use reqwest::Url;
    use std::fs;

    const SEARCH_URL: &str = "https://sru.k10plus.de/opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xall%3D%22trauriger+tiger%22&startRecord=1&maximumRecords=5&recordSchema=marcxml";

    fn search_result_list_from(fixture: &str, url: &str) -> Result<SearchResultList, PaperError> {
        let sut = K10plusSruSearchScraper {
            base_url: "https://sru.k10plus.de".to_string(),
            database: "opac-de-1".to_string(),
        };
        let xml = fs::read_to_string(fixture).expect("Something went wrong reading the file");

        sut.search_result_list_from(
            "trauriger tiger".to_string(),
            &xml,
            &Url::parse(url).unwrap(),
        )
    }

    #[test]
    fn it_parses_search_results() {
        let search_result_list = search_result_list_from(
            "src/fixtures/k10plus_sru/search/search_page1.xml",
            SEARCH_URL,
        )
        .unwrap();

        assert_eq!(search_result_list.text, "trauriger tiger");
        assert_eq!(search_result_list.result_count, 11);
        assert_eq!(search_result_list.items.len(), 5);

        let item = &search_result_list.items[0];
        assert_eq!(item.identifier, "1886167923");
        assert_eq!(item.item_number, Some("1886167923".to_string()));
        assert_eq!(item.title, Some("Trauriger Tiger".to_string()));
        assert_eq!(
            item.subtitle,
            Some("Sinno, Neige - München : dtv, 2024".to_string())
        );
        assert_eq!(
            item.detail_url,
            Some("opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xppn%3D1886167923&maximumRecords=1&recordSchema=marcxml".to_string())
        );
    }

    #[test]
    fn it_parses_the_next_page_url() {
        let search_result_list = search_result_list_from(
            "src/fixtures/k10plus_sru/search/search_page1.xml",
            SEARCH_URL,
        )
        .unwrap();

        assert_eq!(
            search_result_list.next_page_url,
            Some("opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xall%3D%22trauriger+tiger%22&maximumRecords=5&recordSchema=marcxml&startRecord=6".to_string())
        );
    }

    #[test]
    fn it_has_no_next_page_on_the_last_page() {
        let search_result_list = search_result_list_from(
            "src/fixtures/k10plus_sru/search/search_last_page.xml",
            "https://sru.k10plus.de/opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xall%3D%22trauriger+tiger%22&maximumRecords=5&recordSchema=marcxml&startRecord=11",
        )
        .unwrap();

        assert_eq!(search_result_list.result_count, 11);
        assert_eq!(search_result_list.items.len(), 1);
        assert_eq!(search_result_list.next_page_url, None);
    }

    #[test]
    fn it_parses_subtitles_and_online_resources() {
        let search_result_list = search_result_list_from(
            "src/fixtures/k10plus_sru/search/search_online.xml",
            SEARCH_URL,
        )
        .unwrap();

        assert_eq!(search_result_list.result_count, 175503);
        assert_eq!(search_result_list.items.len(), 3);
        assert_eq!(
            search_result_list.items[0].title,
            Some("Human-AI collaboration in research : practical applications, ethical frameworks, and future directions".to_string())
        );
        assert_eq!(
            search_result_list.items[0].subtitle,
            Some("Skubis, Ida - Boca Raton : CRC Press, 2027".to_string())
        );
    }

    #[test]
    fn it_parses_no_results() {
        let search_result_list = search_result_list_from(
            "src/fixtures/k10plus_sru/search/search_none.xml",
            SEARCH_URL,
        )
        .unwrap();

        assert_eq!(search_result_list.result_count, 0);
        assert_eq!(search_result_list.items.len(), 0);
        assert_eq!(search_result_list.next_page_url, None);
    }

    #[test]
    fn it_fails_on_diagnostics() {
        let sut = K10plusSruSearchScraper {
            base_url: "https://sru.k10plus.de".to_string(),
            database: "opac-de-1".to_string(),
        };
        // the response of the unsupported index pica.all
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<zs:searchRetrieveResponse xmlns:zs="http://www.loc.gov/zing/srw/"><zs:version>1.1</zs:version><zs:diagnostics xmlns:diag="http://www.loc.gov/zing/srw/diagnostic/"><diag:diagnostic><diag:uri>info:srw/diagnostic/1/48</diag:uri><diag:message>Query feature unsupported</diag:message></diag:diagnostic></zs:diagnostics></zs:searchRetrieveResponse>"#;

        let result =
            sut.search_result_list_from("AI".to_string(), xml, &Url::parse(SEARCH_URL).unwrap());

        assert_eq!(result.unwrap_err(), PaperError::SearchFailed);
    }
}
