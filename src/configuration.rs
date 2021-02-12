#[derive(Debug)]
pub struct Configuration {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Configuration {
    /// Create a configuration from an existing configuration and a username
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// let c = Configuration::new();
    /// let cwa = c.with_username("abc");
    /// assert_eq!(cwa.username, "abc");
    /// ```
    pub fn with_username(self, username: Option<String>) -> Configuration {
        Configuration { username, ..self }
    }

    /// Create a configuration from an existing configuration and a password
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// let c = Configuration::new();
    /// let cwa = c.with_password("123".toString());
    /// assert_eq!(cwa.password, "123");
    /// ```
    pub fn with_password(self, password: Option<String>) -> Configuration {
        Configuration { password, ..self }
    }
}
