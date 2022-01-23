use crate::model::SessionToken;
use crate::resource::AuthenticationResource;
use crate::resource::ResourceLoader;
use super::configuration::Configuration;

use std::result::Result;

pub struct Authenticator {
}

impl Authenticator {

    pub async fn authenticate(&self, configuration: &Configuration) -> Result<SessionToken, &'static str> {
        let resource = AuthenticationResource {
            username: configuration
                .username
                .as_ref()
                .unwrap_or(&"".to_string())
                .clone(),
            password: configuration
                .password
                .as_ref()
                .unwrap_or(&"".to_string())
                .clone()
        };
        let resource_loader = ResourceLoader::<SessionToken, AuthenticationResource>::new(resource);
        resource_loader.load().await
    }
}
