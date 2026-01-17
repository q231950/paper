#[derive(uniffi::Enum, Clone, Debug, PartialEq)]
pub enum API {
    HamburgPublic,
    Opc4v2_13Vzg6 {
        base_url: String,
        catalog_url: String,
        user_query_key: String,
    },
}

impl API {
    pub fn base_url(&self) -> String {
        match self {
            API::HamburgPublic => "https://www.buecherhallen.de".to_string(),
            API::Opc4v2_13Vzg6 { base_url, .. } => base_url.clone(),
        }
    }

    pub fn catalog_url(&self) -> String {
        match self {
            API::HamburgPublic => "https://buecherhallen.de".to_string(),
            API::Opc4v2_13Vzg6 { catalog_url, .. } => catalog_url.clone(),
        }
    }

    pub fn login_url(&self) -> Option<String> {
        match self {
            API::HamburgPublic => None,
            API::Opc4v2_13Vzg6 { .. } => Some(format!("{}/LBS_WEB/login", self.base_url().clone())),
        }
    }

    pub fn session_url(&self) -> Option<String> {
        //https://kataloge.hh.gbv.de/LBS_WEB/borrower/loans.htm
        match self {
            API::HamburgPublic => None,
            API::Opc4v2_13Vzg6 { .. } => Some(format!(
                "{}/LBS_WEB/borrower/loans.htm",
                self.base_url().clone()
            )),
        }
    }

    pub fn user_query_key(&self) -> Option<String> {
        match self {
            API::HamburgPublic => None,
            API::Opc4v2_13Vzg6 { user_query_key, .. } => Some(user_query_key.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::API;

    #[test]
    fn test_login_url() {
        let api_config = API::Opc4v2_13Vzg6 {
            base_url: String::from("https://example.com"),
            catalog_url: String::from("https://example.com"),
            user_query_key: String::from("1000"),
        };

        let expected_url = "https://example.com/LBS_WEB/login";
        assert_eq!(api_config.login_url(), Some(expected_url.to_string()));
    }

    #[test]
    fn test_user_query_key() {
        let api_config = API::Opc4v2_13Vzg6 {
            base_url: String::from("https://example.com"),
            catalog_url: String::from("https://example.com"),
            user_query_key: String::from("1000"),
        };

        let expected_key = "1000";
        assert_eq!(api_config.user_query_key(), Some(expected_key.to_string()));
    }

    #[test]
    fn test_user_query_key_none() {
        let api_config = API::HamburgPublic;

        assert_eq!(api_config.user_query_key(), None);
    }
}
