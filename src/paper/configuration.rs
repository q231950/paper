#[derive(Debug)]
pub struct Configuration<'a, 'b> {
    pub username: &'a str,
    pub password: &'b str,
}

impl<'a, 'b> Configuration<'a, 'b> {
    pub fn new() -> Configuration<'a, 'b> {
        Configuration {
            username: "",
            password: "",
        }
    }

    pub fn with_username(self, username: &'a str) -> Configuration<'a, 'b> {
        Configuration { username, ..self }
    }

    pub fn with_password(self, password: &'b str) -> Configuration<'a, 'b> {
        Configuration { password, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::Configuration;

    #[test]
    fn default_configuration() {
        let c = Configuration::new();
        assert_eq!(c.username, "");
        assert_eq!(c.password, "");
    }

    #[test]
    fn configuration_with_username() {
        let c = Configuration::new();
        let cwa = c.with_username("abc");
        assert_eq!(cwa.username, "abc");
    }
}
