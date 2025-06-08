use super::{OpacAuthenticator, PublicHamburgAuthenticator};
use crate::configuration::Configuration;
use crate::error::PaperError;
use crate::model::ValidationStatus;

#[derive(uniffi::Object)]
pub struct Authenticator {
    pub(crate) configuration: Configuration,
}

#[uniffi::export(async_runtime = "tokio")]
impl Authenticator {
    #[uniffi::constructor]
    fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }

    async fn verify_credentials(&self) -> Result<ValidationStatus, PaperError> {
        match self.configuration.api_configuration.api {
            crate::model::API::HamburgPublic => {
                let authenticator = PublicHamburgAuthenticator {
                    configuration: self.configuration.clone(),
                };
                let result = authenticator.verify_credentials_public_hamburg().await;
                return match result {
                    Ok(_) => Ok(ValidationStatus::Valid),
                    Err(err) => match err {
                        PaperError::IncorrectCredentials => Ok(ValidationStatus::Invalid),
                        _ => Ok(ValidationStatus::Error(err.to_string())),
                    },
                };
            }
            crate::model::API::Opc4v2_13Vzg6 => {
                let opac_authenticator = OpacAuthenticator {
                    configuration: self.configuration.clone(),
                };
                let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
                let result = opac_authenticator.authenticate(&client).await;
                return match result {
                    Ok(signed_in) => Ok(if signed_in {
                        ValidationStatus::Valid
                    } else {
                        ValidationStatus::Invalid
                    }),
                    Err(err) => Ok(ValidationStatus::Error(err.to_string())),
                };
            }
        }
    }
}
