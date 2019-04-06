pub mod auth;
pub mod configuration;
pub mod xml;

use crate::configuration::Configuration;

pub struct Paper<'a, 'b> {
    configuration: Configuration<'a, 'b>,
}

impl<'a, 'b> Paper<'a, 'b> {
    pub fn with_config(configuration: Configuration<'a, 'b>) -> Paper<'a, 'b> {
        Paper { configuration }
    }

    pub fn loans(&self) {
        match self.authenticate() {
            Ok(token) => println!("Loans: {:?}", token),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn authenticate(&self) -> Result<String, &'static str> {
        let authenticator = auth::Authenticator::new();
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
