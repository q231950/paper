use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Loan {
    pub title: String,
    pub author: String
}

