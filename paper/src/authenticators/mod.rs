pub use self::scrape_authenticator::Authenticator;
mod scrape_authenticator;

pub(crate) use self::opac_authenticator::OpacAuthenticator;
mod opac_authenticator;

pub(crate) use self::public_hamburg_authenticator::PublicHamburgAuthenticator;
mod public_hamburg_authenticator;

pub use self::login_result::LoginResult;
pub use self::login_result::RawLoansPage;
mod login_result;
