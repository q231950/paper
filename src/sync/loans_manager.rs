use crate::api::APIClient;
use crate::auth::SessionToken;
use crate::xml::LoansInfoXmlParser;
use crate::model::LoansInfo;

pub struct LoansManager {
    token: SessionToken,
    api_client: APIClient,
}

impl LoansManager {
    pub fn new(token: SessionToken) -> LoansManager {
        let network_client = reqwest::Client::new();
        LoansManager {
            token,
            api_client: APIClient::new_with_network_client(network_client),
        }
    }

    pub async fn loans(&self) -> Result<LoansInfo, &str>{
        let response = self.api_client.loans_info(&self.token).await;

        match response {
            Ok(r) => match r.text().await {
                Ok(content) => {
                    let parser = LoansInfoXmlParser::new();
                    parser.loans_info_from_xml(content.as_bytes())
                }
                Err(_) => Err("Error getting loans info response content"),
            },
            Err(_) => Err("Error getting loans info response"),
        }
    }
}
