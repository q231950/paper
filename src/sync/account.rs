use crate::xml::AccountInfoXmlParser;

pub struct AccountManager {
    token: String,
    client: reqwest::Client,
}

impl AccountManager {
    pub fn new(token: String) -> AccountManager {
        AccountManager {
            token,
            client: reqwest::Client::new(),
        }
    }

    pub fn account_info(&self) {
        println!("Getting account info for token: {:?}", self.token.clone());
        let body = self.account_info_request_body(self.token.clone());
        let response = self
            .client
            .post("https://zones.buecherhallen.de/app_webuser/WebUserSvc.asmx")
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("Accept", "*/*")
            .header("Accept-Language", "en-us")
            .header("Accept-Encoding", "br, gzip, deflate")
            .header("User-Agent", "Flying Penguin")
            .body(body)
            .send();

        match response {
            Ok(mut r) => {
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
