pub mod api;
pub mod auth;
pub mod configuration;
pub mod model;
pub mod sync;
pub mod xml;

use std::fmt;

use crate::auth::Authenticator;
use crate::auth::SessionToken;
use crate::configuration::Configuration;
use crate::sync::AccountManager;

extern crate indicatif;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;

pub struct Paper {
    configuration: Configuration,
    token: Option<SessionToken>,
}

impl<'a, 'b> Paper {
    pub fn with_config(configuration: Configuration) -> Paper {
        Paper {
            configuration: configuration,
            token: None,
        }
    }

    pub async fn initiate_commands(&self) {
        match self.token.clone() {
            None => match self.authenticate().await {
                Ok(token) => self.accept_command(token.clone()).await,
                Err(e) => println!("Error: {:?}", e),
            },
            Some(token) => self.accept_command(token.clone()).await,
        }
    }

    async fn accept_command(&self, token: SessionToken) {
        loop {
            let selections = &["account", "loans", "help"];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick an item")
                .default(0)
                .items(&selections[..])
                .interact_opt()
                .unwrap();

            if let Some(selection) = selection {
                match selections[selection] {
                    "account" => self.account(token.clone()).await,
                    "loans" => self.loans(token.clone()).await,
                    "help" => self.help(),
                    _ => (),
                }
            } else {
                break;
            }
        }
    }

    async fn account(&self, token: SessionToken) {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(5);
        pb.set_message("Fetching account.");
        let account_manager = AccountManager::new(token);
        let account_info = account_manager.account_info().await;
        match account_info {
            Ok(account) => match account.to_json() {
                Ok(json) => {
                    let s = fmt::format(format_args!("{}", json));
                    pb.finish_with_message(s.as_str());
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    async fn loans(&self, _token: SessionToken) {
        // let sp = Spinner::new(Spinners::Dots, "Getting your loans.".into());
        // sp.stop();
    }

    fn help(&self) {
        println!("Help: hit `esc` to quit")
    }

    async fn authenticate(&self) -> Result<SessionToken, &'static str> {
        let authenticator = Authenticator::new();
        let token_result = authenticator
            .session_token_for_config(&self.configuration)
            .await;

        match token_result {
            Ok(token) => Ok(token),
            Err(_) => Err("An error occurred when retrieving the session token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_with_config() {
        let config = Configuration {
            username: Some("abc".to_string()),
            password: Some("123".to_string()),
        };
        let paper = Paper::with_config(config);
        assert_eq!(paper.configuration.username, Some("abc".to_string()));
        assert_eq!(paper.configuration.password, Some("123".to_string()));
    }
}
