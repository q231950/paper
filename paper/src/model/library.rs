use super::API;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct Library {
    pub identifier: String,
    pub name: String,
    pub subtitle: String,
    /// The library system used to sign in, for the account, loans and renewals and, unless
    /// `search_api` is set, for search
    pub api: API,
    /// The library system used to search and to load search result details, if it differs
    /// from `api`, e.g. the K10plus SRU interface instead of the library's OPAC
    pub search_api: Option<API>,
    pub enabled_for_search: bool,
    pub enabled_for_login: bool,
}

impl Library {
    /// The library system used to search and to load search result details
    pub fn search_api(&self) -> API {
        self.search_api.clone().unwrap_or_else(|| self.api.clone())
    }
}
