use crate::model::API;

#[derive(Debug, uniffi::Record, Clone)]
pub struct Configuration {
    pub username: Option<String>,
    pub password: Option<String>,
    pub api_configuration: API,
}

#[cfg(test)]
mod tests {
    use crate::model::API;

    use super::*;

    #[test]
    fn test_login_url() {
        let api_config = API::Opc4v2_13Vzg6 {
            base_url: String::from("https://example.com"),
            catalog_url: String::from("https://example.com"),
            user_query_key: String::from("1022"),
        };
        let config = Configuration {
            username: Some(String::from("user")),
            password: Some(String::from("pass")),
            api_configuration: api_config,
        };

        assert_eq!(config.username, Some(String::from("user")));
        assert_eq!(config.password, Some(String::from("pass")));
    }
}
