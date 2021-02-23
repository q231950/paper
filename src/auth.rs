use crate::api::APIClient;
use std::result::Result;
use super::configuration::Configuration;

/// A session token is used for authentication
pub type SessionToken = String;

pub struct Authenticator {
    client: APIClient,
}

impl Authenticator {
    /// Convenience initialiser that returns an `Authenticator` with a default client
    pub fn new() -> Authenticator {
        let network_client = reqwest::Client::new();
        let api_client = APIClient::new_with_network_client(network_client);
        Authenticator::authenticator_with_api_client(api_client)
    }

    /// Creates an authenticator using the given client
    fn authenticator_with_api_client(client: APIClient) -> Authenticator {
        Authenticator { client }
    }

    pub async fn session_token_for_config(
        &self,
        configuration: &Configuration,
    ) -> Result<SessionToken, &'static str> {
        self.client.session_token_for_config(configuration
            .username
            .as_ref()
            .unwrap_or(&"".to_string()), 
            configuration
            .password
            .as_ref()
            .unwrap_or(&"".to_string())).await
    }
}
