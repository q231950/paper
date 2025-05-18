use crate::model::APIConfiguration;

#[derive(Debug, uniffi::Record, Clone)]
pub struct Configuration {
    pub username: Option<String>,
    pub password: Option<String>,
    pub api_configuration: APIConfiguration,
}

impl Configuration {
    pub fn login_url(&self) -> String {
        format!("{}/LBS_WEB/login", self.api_configuration.base_url)
    }

    pub fn base_url(&self) -> String {
        self.api_configuration.base_url.clone()
    }

    pub fn session_url(&self) -> String {
        //https://kataloge.hh.gbv.de/LBS_WEB/borrower/loans.htm
        format!(
            "{}/LBS_WEB/borrower/loans.htm",
            self.api_configuration.base_url
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::model::API;

    use super::*;

    #[test]
    fn test_login_url() {
        let api_config = APIConfiguration {
            api: API::Opc4v2_13Vzg6,
            base_url: String::from("https://example.com"),
            catalog_url: String::from("https://example.com"),
        };
        let config = Configuration {
            username: Some(String::from("user")),
            password: Some(String::from("pass")),
            api_configuration: api_config,
        };

        let expected_url = "https://example.com/LBS_WEB/login";
        assert_eq!(config.login_url(), expected_url);
    }
}
