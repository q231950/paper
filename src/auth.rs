use super::configuration::Configuration;
use super::xml::AuthXmlParser;
use std::result::Result;

pub struct Authenticator {
    client: reqwest::Client,
}

impl Authenticator {
    /// Convenience initialiser that returns an `Authenticator` with a default client
    pub fn new() -> Authenticator {
        Authenticator::authenticator_with_client(reqwest::Client::new())
    }

    /// Creates an authenticator using the given client
    fn authenticator_with_client(client: reqwest::Client) -> Authenticator {
        Authenticator { client }
    }

    /// Generate a session token for a given configuration
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// use crate::paper::auth::Authenticator;
    /// let config = Configuration::new();
    /// let client = Authenticator::ReqwestClientMock::new();
    /// let authenticator = Authenticator::authenticatorWithClient(client);
    /// if let Ok(response) = authenticator.session_token_for_config(&config) {
    ///     if let Ok(text) = response.text() {
    ///         assert_eq!(text, "abc".to_string());
    ///     }
    /// }
    /// ```
    pub fn session_token_for_config(
        &self,
        configuration: &Configuration,
    ) -> Result<String, &'static str> {
        let body = self.session_token_request_body(configuration.username, configuration.password);
        let response = self
            .client
            .post("https://zones.buecherhallen.de/app_webuser/WebUserSvc.asmx")
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("Accept", "*/*")
            .header("Accept-Language", "en-us")
            .header("Accept-Encoding", "br, gzip, deflate")
            .header("User-Agent", "Buecherhallen/38 CFNetwork/976 Darwin/18.2.0")
            .body(body)
            .send();

        match response {
            Ok(r) => {
                let parser = AuthXmlParser::new();
                parser.session_token_from_xml(r)
            }
            Err(_) => Err("Error getting session token response"),
        }
    }

    /// Generates the session token request body for the given username and password
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// use crate::paper::auth::Authenticator;
    /// let authenticator = Authenticator::new();
    /// let body = authenticator.session_token_request_body(&"abc", &"123");
    /// assert_eq!(body.length(), 4);
    /// ```
    ///
    /// public just for tests???
    pub fn session_token_request_body(&self, username: &str, password: &str) -> String {
        let x = format!(r#"<?xml version='1.0' encoding='utf-8'?>
        <soap12:Envelope xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance' xmlns:xsd='http://www.w3.org/2001/XMLSchema' xmlns:soap12='http://www.w3.org/2003/05/soap-envelope'>
            <soap12:Body>
                <CheckBorrower xmlns='http://bibliomondo.com/websevices/webuser'>
                <borrowerNumber>{}</borrowerNumber>
                <pin>{}</pin></CheckBorrower>
            </soap12:Body>
        </soap12:Envelope>"#, username, password);
        x
    }
}

/*
pub trait Sender {
fn send() -> Result<Response>;
}

impl Sender for RequestBuilder {}

struct ReqwestClientMock {}

impl Sender for ReqwestClientMock {
pub fn send(self) -> reqwest::Result<reqwest::Response> {
Err("Fake error")
}
}
*/
