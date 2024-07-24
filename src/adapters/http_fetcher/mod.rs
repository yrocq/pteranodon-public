use reqwest::Client;
use serde::de::DeserializeOwned;
use std::time::Duration;
pub struct HttpFetcher {}

impl<'a> HttpFetcher {
    pub fn new() -> Self {
        return HttpFetcher {};
    }
    pub async fn get_json_for_url<T>(&self, url: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let client = Client::new();

        let response = client
            .get(url)
            .timeout(Duration::from_secs(5))
            .header("accept", "application/activity+json")
            .send()
            .await?;

        response.json::<T>().await
    }

    pub async fn get_content_for_url(&self, url: String) -> Result<String, reqwest::Error> {
        let client = Client::new();

        let response = client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .header("accept", "application/activity+json")
            .send()
            .await?;

        response.text().await
    }
}
