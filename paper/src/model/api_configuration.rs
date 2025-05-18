use super::API;

#[derive(uniffi::Record, Clone, Debug)]
pub struct APIConfiguration {
    pub api: API,
    pub base_url: String,
    pub catalog_url: String,
}
