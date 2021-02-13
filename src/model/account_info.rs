use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub readable_full_name: Option<String>,
    pub category_name: Option<String>,
    pub account_balance: Option<String>,
    pub badge_replacement_charge: Option<String>,
    pub credit_balance: Option<String>,
    pub mandatory_credit_balance: Option<String>,
    pub pseudo_forename: Option<String>,
    pub creation_date: Option<String>,
    pub full_name: Option<String>,
    pub birth_date: Option<String>,
    pub branch_name: Option<String>,
    pub amount: Option<String>, // the monetary amount of the subscription
    pub change: Option<String>, // the last modification date of the subscription
    pub start: Option<String>,  // the start date of the current subscription
    pub expiry: Option<String>, // the expiry date of the current subscription
    pub expiry_month: Option<String>, // the month of the next expiration
    pub expiry_year: Option<String>, // the year of the next expiration
    pub postcode: Option<String>,
    pub surname: Option<String>,
    pub forename: Option<String>,
    pub email_address: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub acronym: Option<String>,
}

impl AccountInfo {
    pub fn new() -> AccountInfo {
        AccountInfo {
            readable_full_name: None,
            category_name: None,
            account_balance: None,
            badge_replacement_charge: None,
            credit_balance: None,
            mandatory_credit_balance: None,
            pseudo_forename: None,
            creation_date: None,
            full_name: None,
            birth_date: None,
            branch_name: None,
            amount: None,
            change: None,
            start: None,
            expiry: None,
            expiry_month: None,
            expiry_year: None,
            postcode: None,
            surname: None,
            forename: None,
            email_address: None,
            address_line1: None,
            address_line2: None,
            acronym: None,
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let account = serde_json::to_string_pretty(&self);
        match account {
            Ok(json) => Ok(format!("account: {}", json.as_str())),
            Err(err) => Err(err),
        }
    }
}
