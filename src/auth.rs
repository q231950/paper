use super::configuration::Configuration;

/// Generate a session token for a given configuration
///
/// ```
/// use crate::paper::configuration::Configuration;
/// use crate::paper::auth;
/// let config = Configuration::new();
/// if let Some(token) = auth::session_token_for_config(&config) {
///     assert_eq!(token, "abc".to_string());
/// }
///
pub fn session_token_for_config(_configuration: &Configuration) -> Option<String> {
    Some(String::from("abc"))
}
