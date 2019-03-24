pub mod paper;

use crate::paper::configuration::Configuration;

pub struct Paper<'a, 'b> {
    configuration: Configuration<'a, 'b>,
}

impl<'a, 'b> Paper<'a, 'b> {
    /// Create a Paper object with a given configuration
    ///
    ///
    /// ```
    /// use crate::paper::configuration::Configuration;
    /// let config = Configuration::new();
    /// let paper = with_config(config);
    /// assert_eq!(paper.configuration.username, "");
    /// ```
    pub fn with_config(configuration: Configuration<'a, 'b>) -> Paper<'a, 'b> {
        Paper { configuration }
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    #[test]
//    fn test_with_config() {
//        let config = Configuration::new();
//        let paper = Paper::with_config(config);
//        assert_eq!(paper.configuration.username, "");
//    }
//}
