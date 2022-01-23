use crate::auth::SessionToken;
use crate::model::Resource;
use reqwest::Error;
use reqwest::Response;

use crate::xml::AuthXmlParser;

pub struct APIClient {
    network_client: reqwest::Client,
}

impl APIClient {
    pub fn new_with_network_client(network_client: reqwest::Client) -> APIClient {
        APIClient {
            network_client: network_client,
        }
    }

    pub async fn post(&self, body: String) -> Result<Response, Error> {
        return self
            .network_client
            .post("https://zones.buecherhallen.de/app_webuser/WebUserSvc.asmx")
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("Accept", "*/*")
            .header("Accept-Language", "en-us")
            .header("Accept-Encoding", "br, gzip, deflate")
            .header("User-Agent", "Flying Penguin")
            .body(body)
            .send()
            .await;
    }
}

impl APIClient {

    pub async fn load_resource<P, R: Resource<P>>(&self, resource: &R, token: &SessionToken) -> Result<Response, Error> {
        self.post(resource.request_body(token)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_token_request_body() {
        let network_client = reqwest::Client::new();
        let api_client = APIClient::new_with_network_client(network_client);
        let body = api_client.session_token_request_body(&"abc", &"123");
        assert_eq!(body.len(), 480);
    }
}

impl APIClient {

    /// Generate a session token for a given configuration
    pub async fn session_token_for_config(
        &self,
        username: &String,
        password: &String
    ) -> Result<SessionToken, &'static str> {
        let body = self.session_token_request_body(
            username,
            password
        );
        let response = self.post(body).await;

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
    fn session_token_request_body(&self, username: &str, password: &str) -> String {
        format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
        <soap12:Envelope xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance' xmlns:xsd='http://www.w3.org/2001/XMLSchema' xmlns:soap12='http://www.w3.org/2003/05/soap-envelope'>
            <soap12:Body>
                <CheckBorrower xmlns='http://bibliomondo.com/websevices/webuser'>
                <borrowerNumber>{}</borrowerNumber>
                <pin>{}</pin></CheckBorrower>
            </soap12:Body>
        </soap12:Envelope>"#,
            username, password
        )
    }
}
