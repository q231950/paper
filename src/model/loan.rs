use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Loan {
    pub title: String,
    pub author: String,
    pub can_renew: bool,
    pub date_due: String,
    pub item_number: String,
}

