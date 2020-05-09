pub mod api;
pub mod auth;
pub mod configuration;
pub mod model;
pub mod sync;
pub mod xml;

use crate::auth::Authenticator;
use crate::auth::SessionToken;
use crate::configuration::Configuration;
use crate::sync::AccountManager;

pub struct Paper<'a, 'b> {
    configuration: Configuration<'a, 'b>,
    token: Option<SessionToken>,
}

impl<'a, 'b> Paper<'a, 'b> {
    pub fn with_config(configuration: Configuration<'a, 'b>) -> Paper<'a, 'b> {
        Paper {
            configuration: configuration,
            token: None,
        }
    }

    pub fn initiate_commands(&self) {
        match self.token.clone() {
            None => match self.authenticate() {
                Ok(token) => self.accept_command(token.clone()),
                Err(e) => println!("Error: {:?}", e),
            },
            Some(token) => self.accept_command(token.clone()),
        }
    }

    fn accept_command(&self, token: SessionToken) {
        loop {
            println!("{}", self.command_table());
            let mut choice = String::new();
            std::io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read the command");

            match choice.as_str() {
                "1\n" => self.account(token.clone()),
                "2\n" => self.loans(token.clone()),
                "3\n" => break,
                "q\n" => break,
                _ => (),
            }
        }
    }

    fn command_table(&self) -> &str {
        r#"
        Please select a command:
        1. show account 👩🏻‍💼👨🏼‍💼
        2. show loans
        3. quit (q)
        "#
    }

    fn account(&self, token: SessionToken) {
        println!("\nGetting your account...\n");
        let account_manager = AccountManager::new(token);
        account_manager.account_info();
        println!("\n---\n");
    }

    fn loans(&self, _token: SessionToken) {
        println!("\nGetting your loans...\n");
        println!("\n---\n");
    }

    fn authenticate(&self) -> Result<SessionToken, &'static str> {
        let authenticator = Authenticator::new();
        let token_result = authenticator.session_token_for_config(&self.configuration);

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
        let config = Configuration::new();
        let paper = Paper::with_config(config);
        assert_eq!(paper.configuration.username, "");
    }
}
