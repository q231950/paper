use reqwest::Url;

/// Makes `url` relative to `base_url`, such that it can be appended to the base url again
///
/// https://stabikat.de/Record/1909924415 > Record/1909924415
pub(crate) fn relative_path(base_url: &str, url: &Url) -> Option<String> {
    url.as_str()
        .strip_prefix(&format!("{}/", base_url.trim_end_matches('/')))
        .map(|path| path.to_string())
}

/// The path of a record page, without the search id (`sid`) VuFind appends to record links
///
/// /Record/1909924415?sid=4711 > Record/1909924415
pub(crate) fn record_path(base_url: &str, page_url: &Url, href: &str) -> Option<String> {
    let mut url = page_url.join(href).ok()?;
    url.set_query(None);
    url.set_fragment(None);

    relative_path(base_url, &url)
}

/// The url of the given page of the search on `page_url`
///
/// https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields > https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields&page=2
pub(crate) fn page(page_url: &Url, page: u32) -> Url {
    let query_pairs: Vec<(String, String)> = page_url
        .query_pairs()
        .filter(|(key, _)| key != "page")
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();

    let mut url = page_url.clone();
    url.query_pairs_mut()
        .clear()
        .extend_pairs(query_pairs)
        .append_pair("page", &page.to_string());

    url
}

/// The page number of the search on `page_url`, VuFind starts with page 1
pub(crate) fn current_page(page_url: &Url) -> u32 {
    page_url
        .query_pairs()
        .find(|(key, _)| key == "page")
        .and_then(|(_, value)| value.parse::<u32>().ok())
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_makes_urls_relative_to_the_base_url() {
        let url = Url::parse("https://stabikat.de/Record/1909924415").unwrap();

        assert_eq!(
            relative_path("https://stabikat.de", &url),
            Some("Record/1909924415".to_string())
        );
        assert_eq!(
            relative_path("https://stabikat.de/", &url),
            Some("Record/1909924415".to_string())
        );
        assert_eq!(relative_path("https://example.com", &url), None);
    }

    #[test]
    fn it_strips_the_search_id_from_record_paths() {
        let page_url =
            Url::parse("https://example.com/vufind/Search/Results?lookfor=tiger").unwrap();

        assert_eq!(
            record_path(
                "https://example.com/vufind",
                &page_url,
                "/vufind/Record/123%2F4?sid=4711"
            ),
            Some("Record/123%2F4".to_string())
        );
    }

    #[test]
    fn it_replaces_the_page() {
        let page_url =
            Url::parse("https://stabikat.de/Search/Results?lookfor=tiger&page=2&type=AllFields")
                .unwrap();

        assert_eq!(current_page(&page_url), 2);
        assert_eq!(
            page(&page_url, 3).as_str(),
            "https://stabikat.de/Search/Results?lookfor=tiger&type=AllFields&page=3"
        );
    }

    #[test]
    fn it_starts_with_page_1() {
        let page_url = Url::parse("https://stabikat.de/Search/Results?lookfor=tiger").unwrap();

        assert_eq!(current_page(&page_url), 1);
    }
}
