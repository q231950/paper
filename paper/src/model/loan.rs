use super::SearchResultDetail;

#[derive(Debug, PartialEq, uniffi::Record)]
pub struct Loan {
    pub title: String,
    pub author: String,
    pub can_renew: bool,
    pub renewal_token: Option<String>,
    pub renewals_count: u8,
    pub date_due: String,
    pub borrowed_at: String,
    pub item_number: String,
    pub locked_by_preorder: bool,
    pub(crate) details: SearchResultDetail,
    pub(crate) search_result_detail_url: Option<String>,
}

impl Loan {
    pub fn new() -> Self {
        Loan {
            title: "".to_string(),
            author: "".to_string(),
            can_renew: false,
            renewal_token: None,
            renewals_count: 0,
            date_due: "".to_string(),
            borrowed_at: "".to_string(),
            item_number: "".to_string(),
            locked_by_preorder: false,
            details: SearchResultDetail::new(),
            search_result_detail_url: None,
        }
    }

    pub(crate) fn with_details(&self, details: SearchResultDetail) -> Self {
        Loan {
            title: self.title.clone(),
            author: self.author.clone(),
            can_renew: self.can_renew.clone(),
            renewal_token: self.renewal_token.clone(),
            renewals_count: self.renewals_count,
            date_due: self.date_due.clone(),
            borrowed_at: self.borrowed_at.clone(),
            item_number: self.item_number.clone(),
            locked_by_preorder: self.locked_by_preorder,
            details,
            search_result_detail_url: None,
        }
    }
}
