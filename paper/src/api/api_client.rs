use crate::error::PaperError;
use crate::html_resource::HTMLResource;
use reqwest::Client;
use scraper::html::Html;

pub struct APIClient {
    network_client: reqwest::Client,
    base_url: String,
}

impl APIClient {
    /// Performs a HEAD request to the specified URL to check its availability
    ///
    /// # Arguments
    /// * `url` - The URL to ping
    ///
    /// # Returns
    /// * `Result<u16, reqwest::Error>` - The HTTP status code if successful, or a reqwest error if the request fails
    pub(crate) async fn ping_url(url: &str) -> Result<u16, reqwest::Error> {
        let client = Client::new();
        let response = client.head(url).send().await?;

        Ok(response.status().as_u16())
    }

    /// Performs a HEAD request to the specified URLs to check its availability and returns the
    /// first one that returns a status code 200
    ///
    /// # Arguments
    /// * `urls` - The URLs to ping
    ///
    /// # Returns
    /// * `Option<String>` - The first URL that returned 200 or None
    pub(crate) async fn test_urls(
        urls: impl IntoIterator<Item = Option<String>>,
    ) -> Option<String> {
        for url in urls {
            if let Some(url) = url {
                if matches!(APIClient::ping_url(&url).await, Ok(200)) {
                    return Some(url);
                }
            }
        }

        None
    }

    pub fn new_with_network_client(network_client: reqwest::Client, base_url: String) -> APIClient {
        APIClient {
            network_client,
            base_url,
        }
    }

    /// For example the result of calling
    /// path: suchergebnis-detail/medium/T021525989.html
    pub async fn get_html_at_path(&self, path: String) -> Result<Html, PaperError> {
        let url = format!("{base_url}/{path}", base_url = self.base_url);

        let resource = HTMLResource {
            client: self.network_client.to_owned(),
            url,
        };
        let html = resource.load().await?;

        Ok(scraper::Html::parse_document(html.as_str()))
    }

    /// For example the result of calling
    /// https://lbssbb.gbv.de/DB=1/SET=4/TTL=1/CMD?ACT=SRCHA&IKT=1016&SRT=YOP&TRM=tiger&XML=1&FRST=21
    pub async fn get_xml(&self, path: String) -> Result<String, PaperError> {
        let url = format!("{base_url}/{path}", base_url = self.base_url);

        let resource = HTMLResource {
            client: self.network_client.to_owned(),
            url,
        };
        let xml = resource.load().await?;

        Ok(xml)
    }
}
