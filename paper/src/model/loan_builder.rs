use super::loan::Loan;

pub struct LoanBuilder {
    pub title: Option<String>,
    pub author: Option<String>,
    pub can_renew: Option<bool>,
    pub renewal_token: Option<String>,
    pub renewals_count: Option<u8>,
    pub date_due: Option<String>,
    pub item_number: Option<String>,
    pub locked_by_preorder: Option<bool>,
}

impl LoanBuilder {
    pub fn new() -> LoanBuilder {
        LoanBuilder {
            title: None,
            author: None,
            can_renew: None,
            renewal_token: None,
            renewals_count: None,
            date_due: None,
            item_number: None,
            locked_by_preorder: None,
        }
    }

    pub fn build_loan(&self) -> Loan {
        Loan {
            title: self.title.as_ref().unwrap_or(&"".to_string()).to_string(),
            author: self.author.as_ref().unwrap_or(&"".to_string()).to_string(),
            can_renew: self.can_renew.unwrap_or(false),
            renewal_token: self.renewal_token.clone(),
            renewals_count: self.renewals_count.unwrap_or(0),
            date_due: self
                .date_due
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
            borrowed_at: "".to_string(), // todo
            item_number: self
                .item_number
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
            locked_by_preorder: self.locked_by_preorder.unwrap_or(false),
            details: super::SearchResultDetail::new(), // todo: get the details??
            search_result_detail_url: None,
        }
    }

    pub fn clear(&mut self) {
        self.title = None;
        self.author = None;
        self.can_renew = None;
        self.renewal_token = None;
        self.renewals_count = None;
        self.date_due = None;
        self.item_number = None;
    }
}
