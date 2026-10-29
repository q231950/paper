use std::cmp;

use reqwest::{Client, Url};
use scraper::{ElementRef, Html, Selector};

use super::vufind_url;
use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use crate::model::SearchResultList;
use crate::model::SearchResultListItem;
use crate::scrapers::text_provider::TextProvider;

pub(crate) struct VuFindSearchScraper {
    pub(crate) base_url: String,
}

impl VuFindSearchScraper {
    /// https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields
    /// https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields&page=2
    pub async fn search(
        &self,
        text: &str,
        next_page_url: Option<String>,
    ) -> Result<SearchResultList, PaperError> {
        let base_url = self.base_url.trim_end_matches('/');
        let url = match next_page_url {
            Some(next_page_url) => Url::parse(&format!("{base_url}/{next_page_url}")),
            None => Url::parse_with_params(
                &format!("{base_url}/Search/Results"),
                &[("lookfor", text), ("type", "AllFields")],
            ),
        }
        .map_err(|_| PaperError::ErrorParsingUrl)?;

        let html = HTMLResource {
            client: Client::new(),
            url: url.to_string(),
        }
        .load()
        .await?;

        self.search_result_list_from(text.to_string(), &html, &url)
    }
}

impl VuFindSearchScraper {
    /// Creates a SearchResultList from a VuFind result list loaded from `page_url`
    ///
    /// Relies on markup that both the bootstrap3 and bootstrap5 themes of VuFind share:
    /// - `.search-stats[data-record-total]` for the result count
    /// - `.result[data-record-number]` for each result
    /// - `li.page-next > a` (bootstrap5) or `a.page-next` (bootstrap3) for the next page
    fn search_result_list_from(
        &self,
        text: String,
        html: &str,
        page_url: &Url,
    ) -> Result<SearchResultList, PaperError> {
        let document = Html::parse_document(html);
        let result_selector = Selector::parse(".result")?;

        let mut last_record_number = 0;
        let items: Vec<SearchResultListItem> = document
            .select(&result_selector)
            .filter_map(|result| {
                let record_number = result
                    .attr("data-record-number")
                    .and_then(|number| number.trim().parse::<u32>().ok())
                    .unwrap_or(0);
                last_record_number = cmp::max(last_record_number, record_number);

                self.search_result_list_item_from(result, page_url)
            })
            .collect();

        let record_total = document
            .get_attribute("data-record-total", ".search-stats")
            .and_then(|total| total.trim().parse::<u32>().ok());

        // VuFind renders the search stats even without hits, so this is not a result list
        if record_total.is_none() && items.is_empty() {
            return Err(PaperError::SearchFailed);
        }

        let result_count = record_total.unwrap_or(cmp::max(last_record_number, items.len() as u32));

        let next_page_url = document
            .get_attribute("href", "li.page-next > a, a.page-next")
            .and_then(|href| page_url.join(&href).ok())
            .or_else(|| {
                // fall back to the record numbers in case the pagination is missing
                (last_record_number > 0 && last_record_number < result_count)
                    .then(|| vufind_url::page(page_url, vufind_url::current_page(page_url) + 1))
            })
            .and_then(|url| vufind_url::relative_path(&self.base_url, &url));

        Ok(SearchResultList {
            text,
            next_page_url,
            result_count,
            items,
        })
    }

    fn search_result_list_item_from(
        &self,
        result: ElementRef,
        page_url: &Url,
    ) -> Option<SearchResultListItem> {
        let record_id = result.get_attribute("value", "input.hiddenId");
        let href = result.get_attribute("href", "a.title");

        // not a record, VuFind only lists records with a title link or an id
        if record_id.is_none() && href.is_none() {
            return None;
        }

        let mut list_item = SearchResultListItem::new();
        list_item.title = result.get_text("a.title");

        let author_selector = Selector::parse(".result-author").ok()?;
        let authors: Vec<String> = result
            .select(&author_selector)
            .map(HTMLResource::get_text_from_element)
            .filter(|author| !author.is_empty())
            .collect();
        if !authors.is_empty() {
            list_item.subtitle = Some(authors.join("; "));
        }

        list_item.detail_url = href
            .as_ref()
            .and_then(|href| vufind_url::record_path(&self.base_url, page_url, href))
            .or_else(|| {
                record_id.as_ref().and_then(|id| {
                    let mut url = Url::parse(&format!("{}/", self.base_url)).ok()?;
                    url.path_segments_mut()
                        .ok()?
                        .pop_if_empty()
                        .push("Record")
                        .push(id);
                    vufind_url::relative_path(&self.base_url, &url)
                })
            });

        // covers that VuFind loads via AJAX have no src
        list_item.cover_image_url = result
            .get_attribute("src", "img.recordcover")
            .and_then(|src| page_url.join(&src).ok())
            .filter(|url| !url.path().ends_with("/Cover/Unavailable"))
            .map(|url| url.to_string());

        if let Some(record_id) = record_id {
            list_item.identifier = record_id.clone();
            list_item.item_number = Some(record_id);
        }

        Some(list_item)
    }
}

#[cfg(test)]
mod tests {
    use super::VuFindSearchScraper;
    use crate::error::PaperError;
    use reqwest::Url;
    use std::fs;

    fn search_result_list_from(
        base_url: &str,
        fixture: &str,
        page_url: &str,
    ) -> crate::model::SearchResultList {
        let sut = VuFindSearchScraper {
            base_url: base_url.to_string(),
        };
        let html = fs::read_to_string(fixture).expect("Something went wrong reading the file");

        sut.search_result_list_from("tiger".to_string(), &html, &Url::parse(page_url).unwrap())
            .unwrap()
    }

    #[test]
    fn it_parses_search_results() {
        let search_result_list = search_result_list_from(
            "https://stabikat.de",
            "src/fixtures/vufind/search/results_page1.html",
            "https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields",
        );

        assert_eq!(search_result_list.text, "tiger");
        assert_eq!(search_result_list.result_count, 1234);
        assert_eq!(search_result_list.items.len(), 3);

        let item = &search_result_list.items[0];
        assert_eq!(item.identifier, "1909924415");
        assert_eq!(item.item_number, Some("1909924415".to_string()));
        assert_eq!(item.title, Some("Trauriger Tiger : Roman".to_string()));
        assert_eq!(item.subtitle, Some("Sinno, Neige".to_string()));
        assert_eq!(item.detail_url, Some("Record/1909924415".to_string()));
        assert_eq!(
            item.cover_image_url,
            Some(
                "https://stabikat.de/Cover/Show?author=Sinno%2C+Neige&callnumber=&size=small&title=Trauriger+Tiger&recordid=1909924415&source=Solr&isbns%5B0%5D=9783630877662"
                    .to_string()
            )
        );
    }

    #[test]
    fn it_parses_multiple_authors() {
        let search_result_list = search_result_list_from(
            "https://stabikat.de",
            "src/fixtures/vufind/search/results_page1.html",
            "https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields",
        );

        assert_eq!(
            search_result_list.items[1].subtitle,
            Some("Hüntelmann, Axel C.; Kiechle, Christian".to_string())
        );
    }

    #[test]
    fn it_skips_ajax_and_unavailable_covers() {
        let search_result_list = search_result_list_from(
            "https://stabikat.de",
            "src/fixtures/vufind/search/results_page1.html",
            "https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields",
        );

        // loaded via AJAX
        assert_eq!(search_result_list.items[1].cover_image_url, None);
        // /Cover/Unavailable
        assert_eq!(search_result_list.items[2].cover_image_url, None);
        assert_eq!(search_result_list.items[2].subtitle, None);
    }

    #[test]
    fn it_parses_the_next_page_url() {
        let search_result_list = search_result_list_from(
            "https://stabikat.de",
            "src/fixtures/vufind/search/results_page1.html",
            "https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields",
        );

        assert_eq!(
            search_result_list.next_page_url,
            Some("Search/Results?lookfor=tiger&type=AllFields&page=2".to_string())
        );
    }

    #[test]
    fn it_parses_bootstrap3_search_results_in_a_subdirectory() {
        let search_result_list = search_result_list_from(
            "https://example.com/vufind",
            "src/fixtures/vufind/search/results_page1_bootstrap3.html",
            "https://example.com/vufind/Search/Results?lookfor=tiger&type=AllFields",
        );

        assert_eq!(search_result_list.result_count, 45);
        assert_eq!(search_result_list.items.len(), 2);
        assert_eq!(
            search_result_list.items[0].title,
            Some(
                "Animals and epidemics : interspecies entanglements in historical perspective"
                    .to_string()
            )
        );
        assert_eq!(
            search_result_list.items[0].detail_url,
            Some("Record/1883592396".to_string())
        );
        assert_eq!(
            search_result_list.items[0].cover_image_url,
            Some("https://example.com/vufind/Cover/Show?size=small&isbn=9783412529444".to_string())
        );
        assert_eq!(
            search_result_list.next_page_url,
            Some("Search/Results?lookfor=tiger&type=AllFields&page=2".to_string())
        );
    }

    #[test]
    fn it_has_no_next_page_on_the_last_page() {
        let search_result_list = search_result_list_from(
            "https://example.com/vufind",
            "src/fixtures/vufind/search/results_last_page_bootstrap3.html",
            "https://example.com/vufind/Search/Results?lookfor=tiger&type=AllFields&page=3",
        );

        assert_eq!(search_result_list.result_count, 45);
        assert_eq!(search_result_list.items.len(), 1);
        assert_eq!(search_result_list.next_page_url, None);
    }

    #[test]
    fn it_parses_no_results() {
        let search_result_list = search_result_list_from(
            "https://stabikat.de",
            "src/fixtures/vufind/search/results_none.html",
            "https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields",
        );

        assert_eq!(search_result_list.result_count, 0);
        assert_eq!(search_result_list.items.len(), 0);
        assert_eq!(search_result_list.next_page_url, None);
    }

    #[test]
    fn it_falls_back_to_record_numbers_without_pagination() {
        let sut = VuFindSearchScraper {
            base_url: "https://stabikat.de".to_string(),
        };
        let html = r#"
            <div class="search-stats" data-record-total="45"></div>
            <ol class="record-list">
              <li id="result0" class="result" data-record-number="21">
                <input type="hidden" value="123" class="hiddenId">
                <a href="/Record/123?sid=1" class="title getFull">Tiger</a>
              </li>
            </ol>
        "#;

        let search_result_list = sut
            .search_result_list_from(
                "tiger".to_string(),
                html,
                &Url::parse("https://stabikat.de/Search/Results?lookfor=tiger&page=2").unwrap(),
            )
            .unwrap();

        assert_eq!(
            search_result_list.next_page_url,
            Some("Search/Results?lookfor=tiger&page=3".to_string())
        );
    }

    #[test]
    fn it_builds_the_detail_url_from_the_record_id() {
        let sut = VuFindSearchScraper {
            base_url: "https://stabikat.de".to_string(),
        };
        let html = r#"
            <div class="search-stats" data-record-total="1"></div>
            <div class="result" data-record-number="1">
              <input type="hidden" value="PPN 123/4" class="hiddenId">
            </div>
        "#;

        let search_result_list = sut
            .search_result_list_from(
                "tiger".to_string(),
                html,
                &Url::parse("https://stabikat.de/Search/Results?lookfor=tiger").unwrap(),
            )
            .unwrap();

        assert_eq!(
            search_result_list.items[0].detail_url,
            Some("Record/PPN%20123%2F4".to_string())
        );
        assert_eq!(search_result_list.next_page_url, None);
    }

    #[test]
    fn it_fails_when_the_page_is_no_result_list() {
        let sut = VuFindSearchScraper {
            base_url: "https://stabikat.de".to_string(),
        };
        let html = r#"<h1>An error has occurred</h1>"#;

        let search_result_list = sut.search_result_list_from(
            "tiger".to_string(),
            html,
            &Url::parse("https://stabikat.de/Search/Results?lookfor=tiger").unwrap(),
        );

        assert_eq!(search_result_list.unwrap_err(), PaperError::SearchFailed);
    }
}
