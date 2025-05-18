use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error, uniffi::Error, PartialEq)]
pub enum PaperError {
    #[error("Not Implemented Error")]
    NotImplementedError,
    #[error("Some error")]
    GeneralError,
    #[error("Unsupported library type")]
    LibraryNotSupportedError,
    #[error("Failed to search")]
    SearchFailed,
    #[error("Failed to renew the item")]
    FailedToRenew,
    #[error("Failed to parse renewal token")]
    RenewalTokenParserFailedToParseToken,
    #[error("Missing renewal token")]
    MissingRenewalToken,
    #[error("Failed to parse renewed loan")]
    FailedToParseRenewedLoan,
    #[error("Failed to parse loans")]
    FailedToParseLoans,
    #[error("Failed to renew because the item is not loaned")]
    FailedToRenewLoanBecauseItIsNotLoaned,
    #[error("Signing in is currently not possible")]
    LoginCurrentlyNotPossible,
    #[error("Sign in failed due to incorrect credentials")]
    IncorrectCredentials,
    #[error("Sign in failed due to bad input for credentials")]
    CredentialsBadInput, // for example None username or empty string as password
    #[error("Error reading session token response body")]
    FailedToReadSessionTokenResponseBody,
    #[error("Error getting session token response")]
    FailedToGetSessionTokenResponse,
    #[error("Error getting request token to perform requests")]
    FailedToGetRequestToken,
    #[error("Error getting resource response content")]
    FailedToGetResourceResponseContent,
    #[error("Error getting resource response")]
    ErrorGettingResourceResponse,
    #[error("Unable to create account info from xml")]
    FailedToCreateAccountInfoFromXml,
    #[error("Invalid borrower number")]
    IsInvalidBrwrNum,
    #[error("Failed to parse name in account info")]
    ParseErrorAccountInfoName,
    #[error("Failed to parse account id in account info")]
    ParseErrorAccountInfoAccountId,
    #[error("Failed to parse address in account info")]
    ParseErrorAccountInfoAddress,
    #[error("Failed to parse email in account info")]
    ParseErrorAccountInfoEmail,
    #[error("Failed to parse phone in account info")]
    ParseErrorAccountInfoPhone,
    #[error("Failed to parse status in account info service")]
    ParseErrorAccountInfoServiceStatus,
    #[error("Failed to parse search result detail")]
    ParseErrorSearchResultDetail,
    #[error("Failed to parse charge info in account info service")]
    ParseErrorAccountInfoServiceChargeInfo,
    #[error("Failed to parse charge amount in account info service")]
    ParseErrorAccountInfoServiceChargeAmount,
    #[error("Failed to parse balance in account info")]
    ParseErrorAccountInfoBalance,
    #[error("Failed to parse url")]
    ErrorParsingUrl,
    #[error("Reqwest error")]
    ReqwestError(String),
    #[error("io error")]
    IOError(String),
    #[error("Parser error")]
    ParserError(String),
    #[error("Custom error with message")]
    ErrorWithMessage(String),
}

impl From<reqwest::Error> for PaperError {
    /// Converts a `reqwest::Error` into a `PaperError`.
    ///
    /// This conversion is used to handle errors that occur when making requests using the `reqwest`
    /// crate. The resulting `PaperError` will contain the original error message from `reqwest::Error`.
    fn from(value: reqwest::Error) -> Self {
        PaperError::ReqwestError(value.to_string())
    }
}

impl From<InvalidHeaderValue> for PaperError {
    fn from(value: InvalidHeaderValue) -> Self {
        PaperError::ReqwestError(value.to_string())
    }
}

impl From<scraper::error::SelectorErrorKind<'static>> for PaperError {
    /// Converts a `scraper::error::SelectorErrorKind` into a `PaperError`.
    ///
    /// This conversion is used to handle errors that occur when parsing HTML using the `scraper`
    /// crate. The resulting `PaperError` will contain the original error message from
    /// `scraper::error::SelectorErrorKind`.
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        PaperError::ParserError(value.to_string())
    }
}

impl From<std::io::Error> for PaperError {
    /// Converts a `std::io::Error` into a `PaperError`.
    ///
    /// This conversion is used to handle errors that occur when reading or writing data.
    /// The resulting `PaperError` will contain the original error message from
    /// `std::io::Error`.
    fn from(value: std::io::Error) -> Self {
        PaperError::IOError(value.to_string())
    }
}

impl PaperError {
    /// Creates a new `PaperError` with a custom error message.
    ///
    /// This method is used to create a new instance of `PaperError` with a specific error
    /// message. The resulting error will be an `ErrorWithMessage`.
    pub fn make_error_with_message(msg: &'static str) -> PaperError {
        PaperError::ErrorWithMessage(msg.to_string())
    }
}
