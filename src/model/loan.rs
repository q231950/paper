use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Loan {
    pub title: String,
}

impl Loan {
    pub fn new() -> Loan {
        Loan {
            title: "None".to_string(),
        }
    }
}
