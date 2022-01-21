use crate::api::APIClient;
use crate::model::Resource;

use std::marker::PhantomData;

pub struct SyncManager<P, R: Resource<P>> {
    api_client: APIClient,
    resource: R,
    phantom: PhantomData<P>
}

impl<P, R: Resource<P>> SyncManager<P, R> {
    
    pub fn new(resource: R) -> SyncManager<P, R> {
        let network_client = reqwest::Client::new();
        SyncManager {
            api_client: APIClient::new_with_network_client(network_client),
            resource: resource,
            phantom: PhantomData
        }
    }

    pub async fn sync(&self ) -> Result<P, &'static str> {

        let response = self.api_client.load_resource(&self.resource).await;

        match response {
            Ok(r) => match r.text().await {
                Ok(content) => self.resource.parse(content.as_bytes()),
                Err(_) => Err("Error getting loans info response content"),
            },
            Err(_) => Err("Error getting loans info response"),
        }
    } 
}
