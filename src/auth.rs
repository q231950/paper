use super::configuration::Configuration;
use super::xml::AuthXmlParser;
use crate::api::APIClient;
use std::result::Result;

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

    /// Generate a session token for a given configuration
    pub async fn session_token_for_config(
        &self,
        configuration: &Configuration,
    ) -> Result<SessionToken, &'static str> {
        let body = self.session_token_request_body(
            configuration
                .username
                .as_ref()
                .unwrap_or(&"".to_string())
                .as_str(),
            configuration
                .password
                .as_ref()
                .unwrap_or(&"".to_string())
                .as_str(),
        );
        let response = self.client.post(body).await;

        match response {
            Ok(r) => {
                let parser = AuthXmlParser::new();
                match r.text().await {
                    Ok(content) => parser.session_token_from_xml(content.as_bytes()),
                    Err(_) => Err("Error reading session token response body"),
                }
            }
            Err(_) => Err("Error getting session token response"),
        }
    }

    /// Generates the session token request body for the given username and password
    pub fn session_token_request_body(&self, username: &str, password: &str) -> String {
        let x = format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
        <soap12:Envelope xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance' xmlns:xsd='http://www.w3.org/2001/XMLSchema' xmlns:soap12='http://www.w3.org/2003/05/soap-envelope'>
            <soap12:Body>
                <CheckBorrower xmlns='http://bibliomondo.com/websevices/webuser'>
                <borrowerNumber>{}</borrowerNumber>
                <pin>{}</pin></CheckBorrower>
            </soap12:Body>
        </soap12:Envelope>"#,
            username, password
        );
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_token_request_body() {
        let authenticator = Authenticator::new();
        let body = authenticator.session_token_request_body(&"abc", &"123");
        assert_eq!(body.len(), 480);
    }
}
