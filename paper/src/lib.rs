uniffi::include_scaffolding!("paper");
pub mod api;
pub mod authenticators;
pub mod configuration;
pub mod error;
pub mod html_resource;
pub mod model;
pub mod scrapers;
pub mod token_scraper;

use crate::configuration::Configuration;
use crate::model::Account;

extern crate indicatif;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use scrapers::LibraryScraper;

pub struct Paper {
    configuration: Configuration,
    term: Term,
}

impl Paper {
    pub fn with_config(configuration: Configuration) -> Paper {
        Paper {
            configuration,
            term: Term::stdout(),
        }
    }

    pub async fn initiate_commands(&self) {
        self.term.set_title("Paper");

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(5);
        pb.set_message("Loading everything ⚗️");
        let scraper = LibraryScraper::new(self.configuration.clone());
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();
        let account = scraper
            .public_hamburg_fetch_on_current_runtime(&client)
            .await
            .unwrap();

        pb.finish_with_message("Done ✅");

        self.accept_command(account).await;
    }

    async fn accept_command(&self, account: Account) {
        loop {
            let selections = &["👩🏻‍💼👨🏼‍💼 account", "📚 loans", "🏦 balance", "❓ help"];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick an item")
                .default(0)
                .items(&selections[..])
                .interact_opt()
                .unwrap();

            if let Some(selection) = selection {
                match selection {
                    0 => self.account(&account).await, // account
                    1 => self.loans(&account).await,   // loans
                    2 => self.balance(&account).await, // balance
                    3 => self.help(),                  // help
                    _ => (),
                }
            } else {
                break;
            }
        }
    }

    async fn loans(&self, account: &Account) {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(5);
        pb.set_message("Fetching loans.");

        println!("account: {:?}", account.loans);
    }

    async fn balance(&self, account: &Account) {
        println!("{:?}", account.balance)
    }

    async fn account(&self, account: &Account) {
        println!("{:?}", account.name)
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

    use model::{APIConfiguration, API};

    use super::*;
    #[test]
    fn test_with_config() {
        let config = Configuration {
            username: Some("abc".to_string()),
            password: Some("123".to_string()),
            api_configuration: APIConfiguration {
                api: API::HamburgPublic,
                base_url: "https://www.buecherhallen.de".to_string(),
                catalog_url: "https://catalog.buecherhallen.de".to_string(),
            },
        };
        let paper = Paper::with_config(config);
        assert_eq!(paper.configuration.username, Some("abc".to_string()));
        assert_eq!(paper.configuration.password, Some("123".to_string()));
    }
}
