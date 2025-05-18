use std::cmp;

use crate::api::APIClient;
use crate::error::PaperError;
use crate::model::SearchResultList;
use crate::model::SearchResultListItem;

pub(crate) struct Opc4v2_13Vzg6SearchScraper {}

impl Opc4v2_13Vzg6SearchScraper {
    pub async fn search(
        &self,
        text: &str,
        next_page_url: Option<String>,
        api_client: &APIClient,
    ) -> Result<SearchResultList, PaperError> {
        let next_page_url = next_page_url.or_else(|| {
            Some(format!(
                "/SET=1/TTL=1/CMD?ACT=SRCHA&IKT=1016&SRT=YOP&TRM={text}&XML=1&FRST=1"
            ))
        });

        if let Some(next_page_url) = next_page_url {
            let html = api_client.get_xml(next_page_url).await?;

            return self.search_result_list_from(html, text.to_string()).await;
        }

        return Err(PaperError::SearchFailed);
    }
}

impl Opc4v2_13Vzg6SearchScraper {
    /// Creates a SearchResultList from a scraper::Html document
    ///
    /// https://lbssbb.gbv.de/DB=1/SET=4/TTL=1/CMD?ACT=SRCHA&IKT=1016&SRT=YOP&TRM=tiger&XML=1
    /// https://lbssbb.gbv.de/DB=1/SET=2/TTL=1/CMD?ACT=SRCHA&IKT=1016&SRT=YOP&TRM=tiger&FRST=11
    async fn search_result_list_from(
        &self,
        xml: String,
        search_term: String,
    ) -> Result<SearchResultList, PaperError> {
        println!("Search for {search_term} yielded the folling results:");

        if let Ok(document) = roxmltree::Document::parse(xml.as_str()) {
            let short_title_nodes = document
                .descendants()
                .filter(|n| n.tag_name().name() == "SHORTTITLE");
            let mut max_nr: u32 = 0;
            let mut hits: u32 = 0;
            if let Some(child_node) = document.descendants().find(|n| n.has_tag_name("SET")) {
                if let Some(value) = child_node.attribute("hits") {
                    hits = value.parse::<u32>().unwrap_or(0);
                }
            }

            let items = short_title_nodes
                .into_iter()
                .map(|short_title| {
                    let mut list_item = SearchResultListItem::new();
                    if let Some(identifier) = short_title.attribute("PPN") {
                        list_item.detail_url = Some(format!("DB=1/PPN?PPN={:?}", identifier));
                        list_item.identifier = identifier.to_string();
                        list_item.item_number = Some(identifier.to_string());
                        // the detail path, actually
                    }

                    let mut text_nodes = short_title.children().filter(|n| n.is_text());
                    println!("text_nodes: {:?}", text_nodes);
                    // first text is title
                    text_nodes.next().map(|t| {
                        println!("Title: {:?}", t.text());
                        list_item.title = t.text().map(|t| t.trim().to_string());
                    });

                    // second text is subtitle
                    text_nodes.next().map(|t| {
                        println!("Subtitle: {:?}", t.text());
                        list_item.subtitle = t.text().map(|t| t.trim().to_string());
                    });

                    let nr = short_title
                        .attribute("nr")
                        .unwrap_or("0")
                        .parse::<u32>()
                        .unwrap_or(0);
                    max_nr = cmp::max(max_nr, nr);

                    list_item
                })
                .collect();

            let mut next_page_url = None;

            if hits > max_nr {
                next_page_url = Some(format!(
                    "/SET=2/TTL=1/CMD?ACT=SRCHA&IKT=1016&SRT=YOP&TRM={search_term}&XML=1&FRST={first}", first = max_nr + 1
                ));
                println!("next_page_url: {:?}", next_page_url);
            }

            return Ok(SearchResultList {
                text: search_term,
                next_page_url,
                result_count: hits,
                items,
            });
        } else {
            return Err(PaperError::SearchFailed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Opc4v2_13Vzg6SearchScraper;
    use std::fs;

    #[tokio::test]
    async fn it_parses_search_results_from_page_1() {
        let sut = Opc4v2_13Vzg6SearchScraper {};
        let xml = fs::read_to_string(
            "src/fixtures/opc4v2_13Vzg6/search/katalog_suchergebnisse_page1.xml",
        )
        .expect("Something went wrong reading the file");
        let search_result_list = sut
            .search_result_list_from(xml, "abc".to_string())
            .await
            .unwrap();

        assert_eq!(search_result_list.items.len(), 10);

        assert_eq!(
            search_result_list.items[0].title,
            Some("Original and uncollected stories".to_string())
        );

        assert_eq!(
            search_result_list.items[0].subtitle,
            Some("/ Saki. - Cambridge : Open Book Publishers, [2024]".to_string())
        );

        assert_eq!(
            search_result_list.items[0].item_number,
            Some("1909924415".to_string())
        );
    }

    #[tokio::test]
    async fn it_parses_search_results_from_page_1_sbb() {
        let sut = Opc4v2_13Vzg6SearchScraper {};
        let xml = fs::read_to_string(
            "src/fixtures/opc4v2_13Vzg6/search/katalog_suchergebnisse_page1_sbb.xml",
        )
        .expect("Something went wrong reading the file");
        let search_result_list = sut
            .search_result_list_from(xml, "abc".to_string())
            .await
            .unwrap();

        assert_eq!(search_result_list.items.len(), 10);

        assert_eq!(
            search_result_list.items[0].title,
            Some(
                "Animals and Epidemics : interspecies entanglements in historical perspective"
                    .to_string()
            )
        );

        assert_eq!(
            search_result_list.items[0].subtitle,
            Some("Hüntelmann, Axel C. *1971-*. - Köln : Böhlau, [2024]".to_string())
        );
    }

    #[tokio::test]
    async fn it_parses_next_page_url_sbb() {
        let sut = Opc4v2_13Vzg6SearchScraper {};
        let xml = fs::read_to_string(
            "src/fixtures/opc4v2_13Vzg6/search/katalog_suchergebnisse_page1_sbb.xml",
        )
        .expect("Something went wrong reading the file");
        let search_result_list = sut
            .search_result_list_from(xml, "abc".to_string())
            .await
            .unwrap();

        // assert_eq!(search_result_list.items.len(), 10); max result count

        assert_eq!(
            search_result_list.next_page_url,
            Some("/SET=2/TTL=1/CMD?ACT=SRCHA&IKT=1016&SRT=YOP&TRM=abc&XML=1&FRST=31".to_string())
        );
    }
}
