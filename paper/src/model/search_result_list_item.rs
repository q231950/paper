use uuid::Uuid;

#[derive(Debug, uniffi::Record)]
pub struct SearchResultListItem {
    pub identifier: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub item_number: Option<String>,
    pub detail_url: Option<String>,
    pub cover_image_url: Option<String>,
}

impl SearchResultListItem {
    pub fn new() -> Self {
        let identifier = Uuid::new_v4().to_string();
        let title = None;
        let subtitle = None;
        let item_number = None;
        let detail_url = None;
        let img_url = None;

        Self {
            identifier,
            title,
            subtitle,
            item_number,
            detail_url,
            cover_image_url: img_url,
        }
    }
}
