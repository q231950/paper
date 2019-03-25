#[derive(Debug)]
pub struct Configuration<'a, 'b> {
    pub username: &'a str,
    pub password: &'b str,
}

impl<'a, 'b> Configuration<'a, 'b> {
    /// The default Configuration for paper
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// let c = Configuration::new();
    /// assert_eq!(c.username, "");
    /// assert_eq!(c.password, "");
    /// ```
    pub fn new() -> Configuration<'a, 'b> {
        Configuration {
            username: "",
            password: "",
        }
    }

    /// Create a configuration from an existing configuration and a username
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// let c = Configuration::new();
    /// let cwa = c.with_username("abc");
    /// assert_eq!(cwa.username, "abc");
    /// ```
    pub fn with_username(self, username: &'a str) -> Configuration<'a, 'b> {
        Configuration { username, ..self }
    }

    /// Create a configuration from an existing configuration and a password
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// let c = Configuration::new();
    /// let cwa = c.with_password("123");
    /// assert_eq!(cwa.password, "123");
    /// ```
    pub fn with_password(self, password: &'b str) -> Configuration<'a, 'b> {
        Configuration { password, ..self }
    }
}
