use super::API;

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct Library {
    pub identifier: String,
    pub name: String,
    pub subtitle: String,
    pub api: API,
    pub enabled_for_search: bool,
    pub enabled_for_login: bool,
}
