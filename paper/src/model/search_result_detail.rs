use super::{DataEntry, ItemAvailability};

#[derive(Debug, uniffi::Record)]
pub struct SearchResultDetail {
    pub(crate) medium_title: Option<String>,
    pub(crate) medium_author: Option<String>,
    pub(crate) full_title: Option<String>,
    pub(crate) small_image_url: Option<String>,
    pub(crate) signature: Option<String>,
    pub(crate) data_entries: Vec<DataEntry>,
    pub(crate) hint: Option<String>,
    pub(crate) availability: ItemAvailability,
}

impl SearchResultDetail {
    pub(crate) fn new() -> Self {
        SearchResultDetail {
            medium_title: None,
            medium_author: None,
            full_title: None,
            small_image_url: None,
            signature: None,
            data_entries: Vec::new(),
            hint: None,
            availability: ItemAvailability::new(),
        }
    }
}
