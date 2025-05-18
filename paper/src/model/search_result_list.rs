use super::SearchResultListItem;

#[derive(Debug, uniffi::Record)]
pub struct SearchResultList {
    pub text: String,
    pub next_page_url: Option<String>,
    pub result_count: u32,
    pub items: Vec<SearchResultListItem>,
}
