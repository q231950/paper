use super::loan::Loan;

pub struct LoanBuilder {
    pub title: Option<String>,
    pub author: Option<String>,
    pub can_renew: Option<bool>,
    pub date_due: Option<String>,
    pub item_number: Option<String>,
}

impl LoanBuilder {
    pub fn new() -> LoanBuilder {
        LoanBuilder {
            title: None,
            author: None,
            can_renew: None,
            date_due: None,
            item_number: None,
        }
    }

    pub fn build_loan(&self) -> Loan {
        Loan {
            title: self.title.as_ref().unwrap_or(&"".to_string()).to_string(),
            author: self.author.as_ref().unwrap_or(&"".to_string()).to_string(),
            can_renew: self.can_renew.unwrap_or(false),
            date_due: self.date_due.as_ref().unwrap_or(&"".to_string()).to_string(),
            item_number: self.item_number.as_ref().unwrap_or(&"".to_string()).to_string(),
        }
    }

    pub fn clear(&mut self) {
        self.title = None;
        self.author = None;
        self.can_renew = None;
        self.date_due = None;
        self.item_number = None;
    }
}
