use crate::xml::AccountInfoXmlParser;
use crate::api::APIClient;

pub struct AccountManager {
    token: String,
    api_client: APIClient,
}

impl AccountManager {
    pub fn new(token: String) -> AccountManager {
        let network_client = reqwest::Client::new();
        AccountManager {
            token,
            api_client: APIClient::new_with_network_client(network_client)
        }
    }

    pub fn account_info(&self) {
        println!("Getting account info for token: {:?}", self.token.clone());
        let body = self.account_info_request_body(self.token.clone());
        let response = self
            .api_client
            .post(body);

        match response {
            Ok(r) => {
                let parser = AccountInfoXmlParser::new();
                let account_info = parser.account_info_from_xml(r);
                println!("{:?}", account_info);
            }
            Err(_) => println!("Error getting session token response"),
        }
    }

    pub fn account_info_request_body(&self, token: String) -> String {
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
