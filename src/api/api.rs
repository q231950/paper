use crate::auth::SessionToken;
use reqwest::Error;
use reqwest::Response;

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
    pub async fn account_info(&self, token: &SessionToken) -> Result<Response, Error> {
        let body = self.account_info_request_body(token);
        self.post(body).await
    }

    fn account_info_request_body(&self, token: &SessionToken) -> String {
        let x = format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
        <soap12:Envelope xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'
    xmlns:xsd='http://www.w3.org/2001/XMLSchema'
    xmlns:soap12='http://www.w3.org/2003/05/soap-envelope'>
            <soap12:Body>
                <GetBorrowerSummary xmlns='http://bibliomondo.com/websevices/webuser'>
                    <sessionId>{}</sessionId>
                </GetBorrowerSummary>
            </soap12:Body>
        </soap12:Envelope>"#,
            token
        );
        x
    }
}
