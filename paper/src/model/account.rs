use crate::model::Balance;
use crate::model::Notification;
use std::collections::HashMap;

use super::Loans;
use super::NotificationType;

#[derive(Debug, uniffi::Record)]
pub struct Account {
    pub account_id: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub charge_info: HashMap<String, String>,
    pub loans: Loans,
    pub balance: Option<Balance>,
    pub notifications: Vec<Notification>,
}

impl Account {
    pub fn new() -> Account {
        Account {
            account_id: "".to_string(),
            name: "".to_string(),
            address: "".to_string(),
            phone: "".to_string(),
            email: "".to_string(),
            charge_info: HashMap::new(),
            loans: Loans::new(),
            balance: None,
            notifications: Vec::new(),
        }
    }

    pub fn error_notification(&self) -> Option<&Notification> {
        self.notifications
            .iter()
            .find(|notification| notification.notification_type == NotificationType::Error)
    }
}
