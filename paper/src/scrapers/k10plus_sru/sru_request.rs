use reqwest::Url;

const PAGE_SIZE: u32 = 20;

/// The SRU search for `text` in all words (`pica.xall`) of the K10plus `database`, sorted by year
/// of publication, newest first
///
/// https://sru.k10plus.de/opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xall%3D%22tiger%22&startRecord=1&maximumRecords=20&recordSchema=marcxml
pub(crate) fn search_url(base_url: &str, database: &str, text: &str) -> Option<Url> {
    // a CQL term in quotes searches all of its words, quotes and backslashes need escaping
    let term = text.trim().replace('\\', "\\\\").replace('"', "\\\"");
    let query = format!("pica.xall=\"{term}\"");
    let page_size = PAGE_SIZE.to_string();

    Url::parse_with_params(
        &format!("{base_url}/{database}"),
        &[
            ("version", "1.1"),
            ("operation", "searchRetrieve"),
            ("query", query.as_str()),
            ("startRecord", "1"),
            ("maximumRecords", page_size.as_str()),
            ("recordSchema", "marcxml"),
        ],
    )
    .ok()
}

/// The SRU request for the record with the given PPN
pub(crate) fn record_url(base_url: &str, database: &str, ppn: &str) -> Option<Url> {
    let query = format!("pica.xppn={ppn}");

    Url::parse_with_params(
        &format!("{base_url}/{database}"),
        &[
            ("version", "1.1"),
            ("operation", "searchRetrieve"),
            ("query", query.as_str()),
            ("maximumRecords", "1"),
            ("recordSchema", "marcxml"),
        ],
    )
    .ok()
}

/// The search of `url` starting at the given record position
pub(crate) fn with_start_record(url: &Url, start_record: u32) -> Url {
    let query_pairs: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(key, _)| key != "startRecord")
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();

    let mut url = url.clone();
    url.query_pairs_mut()
        .clear()
        .extend_pairs(query_pairs)
        .append_pair("startRecord", &start_record.to_string());

    url
}

/// The path of `url` relative to the SRU base URL, e.g. `opac-de-1?version=1.1&operation=…`
pub(crate) fn relative_path(url: &Url) -> String {
    let path = url.path().trim_start_matches('/');

    match url.query() {
        Some(query) => format!("{path}?{query}"),
        None => path.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_searches_all_words_newest_first() {
        let url = search_url("https://sru.k10plus.de", "opac-de-1", " trauriger tiger ").unwrap();

        assert_eq!(
            url.as_str(),
            "https://sru.k10plus.de/opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xall%3D%22trauriger+tiger%22&startRecord=1&maximumRecords=20&recordSchema=marcxml"
        );
    }

    #[test]
    fn it_escapes_quotes_and_backslashes() {
        let url = search_url("https://sru.k10plus.de", "opac-de-1", r#"a "b" \c"#).unwrap();
        let query = url
            .query_pairs()
            .find(|(key, _)| key == "query")
            .unwrap()
            .1
            .into_owned();

        assert_eq!(query, r#"pica.xall="a \"b\" \\c""#);
    }

    #[test]
    fn it_requests_a_record_by_ppn() {
        let url = record_url("https://sru.k10plus.de", "opac-de-1", "505539255X").unwrap();

        assert_eq!(
            relative_path(&url),
            "opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xppn%3D505539255X&maximumRecords=1&recordSchema=marcxml"
        );
    }

    #[test]
    fn it_replaces_the_start_record() {
        let url = search_url("https://sru.k10plus.de", "opac-de-1", "tiger").unwrap();

        assert_eq!(
            relative_path(&with_start_record(&url, 21)),
            "opac-de-1?version=1.1&operation=searchRetrieve&query=pica.xall%3D%22tiger%22&maximumRecords=20&recordSchema=marcxml&startRecord=21"
        );
    }
}
