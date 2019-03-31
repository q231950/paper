use super::configuration::Configuration;
use reqwest::{Response, Result};

pub struct Authenticator {
    client: reqwest::Client,
}

impl Authenticator {
    pub fn new() -> Authenticator {
        Authenticator {
            client: reqwest::Client::new(),
        }
    }

    /// Generate a session token for a given configuration
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// use crate::paper::auth;
    /// let config = Configuration::new();
    /// if let Ok(mut response) = auth::session_token_for_config(&config) {
    ///     assert_eq!(response.text(), "abc".to_string());
    /// }
    ///
    pub fn session_token_for_config(&self, configuration: &Configuration) -> Result<Response> {
        let body = self.session_token_request_body(configuration.username, configuration.password);
        self.client
            .post("https://zones.buecherhallen.de/app_webuser/WebUserSvc.asmx")
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("Accept", "*/*")
            .header("Accept-Language", "en-us")
            .header("Accept-Encoding", "br, gzip, deflate")
            .header("User-Agent", "Buecherhallen/38 CFNetwork/976 Darwin/18.2.0")
            .body(body)
            .send()
    }

    fn session_token_request_body(&self, username: &str, password: &str) -> String {
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
