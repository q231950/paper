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
    /// let c = Configuration {username: Some("abc".to_string()), password: None};
    /// let cwa = c.with_username(Some("abc".to_string()));
    /// assert_eq!(cwa.username, Some("abc".to_string()));
    /// ```
    pub fn with_username(self, username: Option<String>) -> Configuration {
        Configuration { username, ..self }
    }

    /// Create a configuration from an existing configuration and a password
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// let c = Configuration {username: None, password: Some("123".to_string())};
    /// let cwa = c.with_password(Some("123".to_string()));
    /// assert_eq!(cwa.password, Some("123".to_string()));
    /// ```
    pub fn with_password(self, password: Option<String>) -> Configuration {
        Configuration { password, ..self }
    }
}
