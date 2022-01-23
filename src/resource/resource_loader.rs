use crate::api::APIClient;
use crate::resource::Resource;

use std::marker::PhantomData;

pub struct ResourceLoader<P, R: Resource<P>> {
    api_client: APIClient,
    resource: R,
    phantom: PhantomData<P>
}

impl<P, R: Resource<P>> ResourceLoader<P, R> {

    pub fn new(resource: R) -> ResourceLoader<P, R> {
        let network_client = reqwest::Client::new();
        ResourceLoader {
            api_client: APIClient::new_with_network_client(network_client),
            resource: resource,
            phantom: PhantomData
        }
    }

    pub async fn load(&self ) -> Result<P, &'static str> {

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
