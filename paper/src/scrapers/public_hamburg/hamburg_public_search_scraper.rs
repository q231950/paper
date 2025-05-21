use crate::api::APIClient;
use crate::error::PaperError;
use crate::model::SearchResultList;
use crate::model::SearchResultListItem;
use crate::scrapers::text_provider::TextProvider;
use futures::future;
use reqwest::Client;
use scraper::Selector;
use tokio::runtime::Builder;
use uuid::Uuid;

pub(crate) struct HamburgPublicSearchScraper {}

impl HamburgPublicSearchScraper {
    /// https://www.buecherhallen.de/katalog-suchergebnisse.html?suchbegriff=extra+terrestrial&seite-m37=2
    pub async fn search(
        &self,
        text: &str,
        next_page_url: Option<String>,
    ) -> Result<SearchResultList, PaperError> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(5)
            .thread_name("search")
            .enable_io()
            .enable_time()
            .build()?;

        return runtime
            .block_on(async { self.search_on_current_runtime(text, next_page_url).await });
    }

    pub(crate) async fn search_on_current_runtime(
        &self,
        text: &str,
        next_page_url: Option<String>,
    ) -> Result<SearchResultList, PaperError> {
        let next_page_url = next_page_url.or_else(|| {
            Some(format!(
                "katalog-suchergebnisse.html?suchbegriff={text}&seite-m37=1"
            ))
        });

        let client = Client::new();
        let api_client =
            APIClient::new_with_network_client(client, "https://www.buecherhallen.de".to_string());

        if let Some(next_page_url) = next_page_url {
            let html = api_client.get_html_at_path(next_page_url).await?;

            return self.search_result_list_from(text.to_string(), html).await;
        }

        return Err(PaperError::SearchFailed);
    }
}

impl HamburgPublicSearchScraper {
    /// Creates a SearchResultList from a scraper::Html document
    ///
    /// https://www.buecherhallen.de/katalog-suchergebnisse.html?suchbegriff=extra+terrestrial&seite-m37=1
    /// https://www.buecherhallen.de/katalog-suchergebnisse.html?suchbegriff=extra+terrestrial&seite-m37=2
    async fn search_result_list_from(
        &self,
        text: String,
        document: scraper::Html,
    ) -> Result<SearchResultList, PaperError> {
        if let Ok(result_selector) = Selector::parse(r#"li[class="search-results-item"]"#) {
            let result_items = document.select(&result_selector);

            let items = future::join_all(result_items.into_iter().map(|item| async move {
                let mut list_item = SearchResultListItem::new();
                list_item.title = item.get_text("div.search-results-text > h2");

                let details_selector = Selector::parse("div.search-results-details").unwrap();
                if let Some(div) = item.select(&details_selector).next() {
                    // Cleaned up version
                    let cleaned_text: String = div
                        .text()
                        .map(|t| t.trim())
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ");

                    list_item.subtitle = Some(cleaned_text);
                }

                if let Some(url1) = item.get_attribute("data-src", "div.search-results-image > img")
                {
                    match APIClient::ping_url(url1.as_str()).await {
                        Ok(status) => match status {
                            200 => {
                                println!("source exists: {}", status);
                                list_item.cover_image_url = Some(url1);
                            }
                            _ => {
                                println!("source does not exist: {}", status);
                                list_item.cover_image_url = item.get_attribute(
                                    "data-alt-src",
                                    "div.search-results-image > img",
                                );
                            }
                        },
                        Err(e) => println!("Error checking site: {}", e),
                    }
                }

                if let Some(item_number) = item.attr("id").map(|s| s.to_string()) {
                    list_item.detail_url = Some(
                        format!("suchergebnis-detail/medium/{}.html", item_number).to_string(),
                    );

                    // append a uuid to the item
                    list_item.identifier = Uuid::new_v4().to_string();
                    list_item.item_number = Some(item_number);
                }

                list_item
            }))
            .await;

            let next_page_url = document.get_attribute("href", r#"a[class="pagination-next"]"#);
            let result_count = self.search_result_count(document);

            Ok(SearchResultList {
                text,
                next_page_url,
                result_count,
                items,
            })
        } else {
            return Err(PaperError::SearchFailed);
        }
    }

    fn search_result_count(&self, document: scraper::Html) -> u32 {
        let info = document.get_text(r#"p[class="search-results-info"]"#);

        // "3.893 Treffer …" > "3893 Treffer …"
        let sanatized_info = info.map_or("".to_string(), |s| s.replace(".", ""));
        if sanatized_info.contains("Keine Treffer") {
            return 0;
        } else {
            // "3893 Treffer …" > 3893
            let number: Option<u32> = sanatized_info
                .split_whitespace()
                .next()
                .and_then(|s| s.parse().ok());
            return number.unwrap_or(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HamburgPublicSearchScraper;
    use std::fs;

    #[tokio::test]
    async fn it_parses_search_results_when_single_page() {
        let sut = HamburgPublicSearchScraper {};
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/katalog_suchergebnisse_single_page.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_list = sut
            .search_result_list_from("abc".to_string(), document)
            .await
            .unwrap();

        assert_eq!(search_result_list.items.len(), 7);
        assert_eq!(search_result_list.next_page_url, None);

        let first_item = &search_result_list.items[0];
        assert_eq!(
            first_item.title,
            Some("Trouble is my business / 5 Trouble Bubble".to_string())
        );
        assert_eq!(first_item.subtitle, Some("Person(en): Taniguchi, Jirō 2019, Deutsch, 1. Auflage, German language edition Signatur: Comic.0 Medienart: Buch".to_string()))
    }

    #[tokio::test]
    async fn it_parses_search_results_from_page_1() {
        let sut = HamburgPublicSearchScraper {};
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/katalog_suchergebnisse_page1.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_list = sut
            .search_result_list_from("abc".to_string(), document)
            .await
            .unwrap();

        assert_eq!(search_result_list.items.len(), 10);

        assert_eq!(
            search_result_list.next_page_url,
            Some(
                "katalog-suchergebnisse.html?suchbegriff=extra+terrestrial&seite-m37=2".to_string()
            )
        );

        let first_item = &search_result_list.items[0];
        assert_eq!(
            first_item.title,
            Some("E.T The Extra-Terrestrial".to_string())
        );
        assert_eq!(first_item.subtitle, Some("Person(en): Gaines, Caseen 2022, Englisch Signatur: fil 6.2 ET•/21 Englisch fil 6.2 Medienart: Buch".to_string()))
    }

    #[tokio::test]
    async fn it_parses_search_results_from_page_2() {
        let sut = HamburgPublicSearchScraper {};
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/katalog_suchergebnisse_page2.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_list = sut
            .search_result_list_from("abc".to_string(), document)
            .await
            .unwrap();

        assert_eq!(search_result_list.items.len(), 7);

        assert_eq!(search_result_list.next_page_url, None);

        let first_item = &search_result_list.items[6];
        assert_eq!(
            first_item.title,
            Some("E.T. - Der Ausserirdische".to_string())
        );
    }

    #[tokio::test]
    async fn it_parses_search_result_count_when_none() {
        let sut = HamburgPublicSearchScraper {};
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/katalog_suchergebnisse_count_none.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_list = sut
            .search_result_list_from("abc".to_string(), document)
            .await
            .unwrap();

        assert_eq!(search_result_list.result_count, 0);
        assert_eq!(search_result_list.next_page_url, None);
    }

    #[tokio::test]
    async fn it_parses_search_result_count_when_few() {
        let sut = HamburgPublicSearchScraper {};
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/katalog_suchergebnisse_count_few.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_list = sut
            .search_result_list_from("abc".to_string(), document)
            .await
            .unwrap();

        assert_eq!(search_result_list.result_count, 57);
        assert_eq!(
            search_result_list.next_page_url,
            Some(
                "katalog-suchergebnisse.html?suchbegriff=extra+terrestrial&seite-m37=2".to_string()
            )
        );
    }

    #[tokio::test]
    async fn it_parses_search_result_count_when_many() {
        let sut = HamburgPublicSearchScraper {};
        let html = fs::read_to_string(
            "src/fixtures/hamburg_public/search/katalog_suchergebnisse_count_many.html",
        )
        .expect("Something went wrong reading the file");
        let document = scraper::Html::parse_document(html.as_str());
        let search_result_list = sut
            .search_result_list_from("abc".to_string(), document)
            .await
            .unwrap();

        assert_eq!(search_result_list.result_count, 3893);
        assert_eq!(
            search_result_list.next_page_url,
            Some("katalog-suchergebnisse.html?suchbegriff=extra&seite-m37=2".to_string())
        );
    }
}
