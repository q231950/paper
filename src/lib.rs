pub mod auth;
pub mod configuration;
pub mod model;
pub mod sync;
pub mod xml;

use crate::auth::Authenticator;
use crate::configuration::Configuration;
use crate::sync::AccountManager;

pub struct Paper<'a, 'b> {
    configuration: Configuration<'a, 'b>,
}

impl<'a, 'b> Paper<'a, 'b> {
    pub fn with_config(configuration: Configuration<'a, 'b>) -> Paper<'a, 'b> {
        Paper { configuration }
    }

    pub fn fetch_user(&self) {
        match self.authenticate() {
            Ok(token) => {
                self.loans(token.clone());
                self.borrower(token.clone());
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn borrower(&self, token: String) {
        let account_manager = AccountManager::new(token);
        account_manager.account_info();
    }

    fn loans(&self, token: String) {
        println!("Getting loans for token: {:?}", token)
    }

    fn authenticate(&self) -> Result<String, &'static str> {
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
