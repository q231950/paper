pub struct APIClient {
    network_client: reqwest::Client,
}

impl APIClient {
    pub fn new_with_network_client(network_client: reqwest::Client) -> APIClient {
        APIClient {
            network_client: network_client,
        }
    }

    pub fn post(&self, body: String) -> Result<reqwest::Response, reqwest::Error> {
        return self.network_client
            .post("https://zones.buecherhallen.de/app_webuser/WebUserSvc.asmx")
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("Accept", "*/*")
            .header("Accept-Language", "en-us")
            .header("Accept-Encoding", "br, gzip, deflate")
            .header("User-Agent", "Flying Penguin")
            .body(body)
            .send();
    }
}
