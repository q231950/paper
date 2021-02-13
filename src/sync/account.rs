use crate::api::APIClient;
use crate::auth::SessionToken;
use crate::xml::AccountInfoXmlParser;

pub struct AccountManager {
    token: SessionToken,
    api_client: APIClient,
}

impl AccountManager {
    pub fn new(token: SessionToken) -> AccountManager {
        let network_client = reqwest::Client::new();
        AccountManager {
            token,
            api_client: APIClient::new_with_network_client(network_client),
        }
    }

    pub async fn account_info(&self) {
        let response = self.api_client.account_info(&self.token).await;

        match response {
            Ok(r) => match r.text().await {
                Ok(content) => {
                    let parser = AccountInfoXmlParser::new();
                    let account_info = parser.account_info_from_xml(content.as_bytes());
                    match account_info {
                        Ok(account) => match account.to_json() {
                            Ok(json) => println!("{}", json),
                            Err(_) => (),
                        },
                        Err(_) => (),
                    }
                }
                Err(_) => println!("Error getting account response content"),
            },
            Err(_) => println!("Error getting account response"),
        }
    }
}
