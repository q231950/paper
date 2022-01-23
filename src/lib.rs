pub mod api;
pub mod auth;
pub mod configuration;
pub mod model;
pub mod resource;
pub mod xml;

use crate::auth::Authenticator;
use crate::model::SessionToken;
use crate::configuration::Configuration;
use crate::model::AccountInfo;
use crate::model::LoansInfo;
use crate::resource::AccountInfoResource;
use crate::resource::LoansInfoResource;
use crate::resource::ResourceLoader;

extern crate indicatif;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;

pub struct Paper {
    configuration: Configuration,
    token: Option<SessionToken>,
    term: Term,
}

impl<'a, 'b> Paper {
    pub fn with_config(configuration: Configuration) -> Paper {
        Paper {
            configuration: configuration,
            token: None,
            term: Term::stdout(),
        }
    }

    pub async fn initiate_commands(&self) {
        self.term.set_title("Paper");

        match self.token.clone() {
            None => match self.authenticate().await {
                Ok(token) => self.accept_command(token.clone()).await,
                Err(e) => println!("Error: {:?}", e),
            },
            Some(token) => self.accept_command(token.clone()).await,
        }
    }

    async fn authenticate(&self) -> Result<SessionToken, &'static str> {
        let authenticator = Authenticator {};
        authenticator.authenticate(&self.configuration).await
    }

    async fn accept_command(&self, token: SessionToken) {
        loop {
            let selections = &["👩🏻‍💼👨🏼‍💼 account", "📚 loans", "❓ help"];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick an item")
                .default(0)
                .items(&selections[..])
                .interact_opt()
                .unwrap();

            if let Some(selection) = selection {
                match selection {
                    0 => self.account(token.clone()).await, // account
                    1 => self.loans(token.clone()).await,   // loans
                    2 => self.help(),                       // help
                    _ => (),
                }
            } else {
                break;
            }
        }
    }

    async fn loans(&self, token: SessionToken) {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(5);
        pb.set_message("Fetching loans.");

        let resource = LoansInfoResource {token: token};
        let resource_loader = ResourceLoader::<LoansInfo, LoansInfoResource>::new(resource);

        let loans = resource_loader.load().await;

        match loans {
            Ok(info) => pb.finish_with_message(info.as_table().as_str()),
            _ => (),
        }
    }

    async fn account(&self, token: SessionToken) {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(5);
        pb.set_message("Fetching account.");

        let resource = AccountInfoResource {token: token};
        let resource_loader = ResourceLoader::<AccountInfo, AccountInfoResource>::new(resource);
        let account_info = resource_loader.load().await;
        match account_info {
            Ok(account) => {
                pb.finish_with_message(account.as_table().as_str());
            },
            Err(_) => (),
        }
    }

    fn help(&self) {
        let _ = self.term.write_line(&format!(
                "help: hit {} to quit",
                style(" esc ").white().on_black()
                ));
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
