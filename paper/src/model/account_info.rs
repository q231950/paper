use serde::{Deserialize, Serialize};
use serde_json::Result;

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

#[derive(Serialize, Deserialize, uniffi::Record)]
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

    pub fn as_table(&self) -> String {
        let mut table = Table::new();
        let account_balance_string_color = if self.account_balance_string().contains("-") {
            Color::Red
        } else {
            Color::Green
        };
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_table_width(80)
            .add_row(vec![
                Cell::new("Name").add_attribute(Attribute::Bold),
                Cell::new(self.human_readable_name_string()),
            ])
            .add_row(vec![
                Cell::new("Address").add_attribute(Attribute::Bold),
                Cell::new(self.address()),
            ])
            .add_row(vec![
                Cell::new("Email").add_attribute(Attribute::Bold),
                Cell::new(self.email_string()),
            ])
            .add_row(vec![
                Cell::new("Membership").add_attribute(Attribute::Bold),
                Cell::new(format!(
                    "{} - {}",
                    self.creation_date_string(),
                    self.expiry_date_string()
                )),
            ])
            .add_row(vec![
                Cell::new("Annual Fee").add_attribute(Attribute::Bold),
                Cell::new(format!(
                    "{} ({})",
                    self.fee_string(),
                    self.membership_category_string()
                )),
            ])
            .add_row(vec![
                Cell::new("Account Balance").add_attribute(Attribute::Bold),
                Cell::new(self.account_balance_string()).fg(account_balance_string_color),
            ])
            .add_row(vec![
                Cell::new("Account Credit").add_attribute(Attribute::Bold),
                Cell::new(self.account_credit_string()),
            ]);

        format!("\n{}", table)
    }

    fn address(&self) -> String {
        let line1 = self.address_line1.to_owned().unwrap_or("".to_string());
        let post_code = self.postcode.to_owned().unwrap_or("".to_string());
        let line2 = self.address_line2.to_owned().unwrap_or("".to_string());
        format!("{}\n{} {}", line1, post_code, line2)
    }

    fn email_string(&self) -> String {
        self.email_address.to_owned().unwrap_or("".to_string())
    }

    fn fee_string(&self) -> String {
        self.amount.to_owned().unwrap_or("".to_string())
    }

    fn human_readable_name_string(&self) -> String {
        self.readable_full_name.to_owned().unwrap_or("".to_string())
    }

    fn membership_category_string(&self) -> String {
        self.category_name.to_owned().unwrap_or("".to_string())
    }

    fn account_balance_string(&self) -> String {
        self.account_balance.to_owned().unwrap_or("".to_string())
    }

    fn account_credit_string(&self) -> String {
        self.credit_balance.to_owned().unwrap_or("".to_string())
    }

    fn creation_date_string(&self) -> String {
        self.creation_date.to_owned().unwrap_or("".to_string())
    }

    fn expiry_date_string(&self) -> String {
        self.expiry.to_owned().unwrap_or("".to_string())
    }
}
