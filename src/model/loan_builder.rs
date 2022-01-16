use super::loan::Loan;

pub struct LoanBuilder {
    pub title: Option<String>,
    pub author: Option<String>,
}

impl LoanBuilder {
    pub fn new() -> LoanBuilder {
        LoanBuilder {
            title: None,
            author: None
        }
    }

    pub fn build_loan(&self) -> Loan {
        Loan {
            title: self.title.as_ref().unwrap_or(&"".to_string()).to_string(),
            author: self.author.as_ref().unwrap_or(&"".to_string()).to_string()
        }
    }

    pub fn clear(&mut self) {
        self.title = None;
        self.author = None;
    }
}
