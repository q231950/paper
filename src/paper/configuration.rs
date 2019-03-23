#[derive(Debug)]
pub struct Configuration {
    pub access_token: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            access_token: "".to_string(),
        }
    }

    pub fn with_access_token(self, token: &str) -> Configuration {
        Configuration {
            access_token: token.to_string(),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Configuration;

    #[test]
    fn default_configuration() {
        let c = Configuration::new();
        assert_eq!(c.access_token, "".to_string())
    }

    #[test]
    fn configuration_with_access_token() {
        let c = Configuration::new();
        let cwa = c.with_access_token("abc");
        assert_eq!(cwa.access_token, "abc".to_string());
    }
}
