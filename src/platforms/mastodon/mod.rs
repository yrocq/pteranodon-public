mod mapper;
pub mod models;

pub use models::*;

use crate::adapters::handle::Handle;
use crate::adapters::http_fetcher::HttpFetcher;
use crate::features::accounts::account;
use crate::models::Instance as InstanceModel;
use crate::platforms::mastodon::map_instance_to_index;

use core::result::Result::Ok;
pub struct Mastodon {
    pub http_fetcher: HttpFetcher,
}
use mapper::*;

impl Mastodon {
    pub fn new() -> Mastodon {
        Mastodon {
            http_fetcher: HttpFetcher::new(),
        }
    }

    fn get_instance_info_link_for_domain(&self, domain: &String) -> String {
        let url = format!("https://{}/api/v1/instance", domain);
        url
    }

    pub async fn get_peers_domains_from_domain(
        &self,
        domain: &str,
    ) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("https://{}/api/v1/instance/peers", domain).to_string();

        let r = self
            .http_fetcher
            .get_json_for_url::<Vec<String>>(&url)
            .await?;

        Ok(r)
    }

    pub async fn get_followers(&self, id: &str) -> Result<Vec<String>, reqwest::Error> {
        self.get_users_list(id, "followers").await
    }

    pub async fn get_following(&self, id: &str) -> Result<Vec<String>, reqwest::Error> {
        self.get_users_list(&id, "following").await
    }

    pub async fn get_users_list(
        &self,
        id: &str,
        endpoint: &str,
    ) -> Result<Vec<String>, reqwest::Error> {
        let mut url = format!("{}/{}?page=1", id, endpoint).to_string();
        let mut followers = Vec::new();
        let mut current_followers: Followers;
        let mut i = 0;
        let max = 50;

        loop {
            println!("Fetching: {}", url);
            current_followers = self.http_fetcher.get_json_for_url(&url.to_string()).await?;
            current_followers.ordered_items.into_iter().for_each(|id| {
                followers.push(id);
            });

            i += 1;

            if current_followers.next != None && i < max {
                url = current_followers.next.unwrap();
            } else {
                break;
            }
        }

        Ok(followers)
    }

    pub async fn get_account(&self, id: &str) -> Result<account::Account, String> {
        let account = self.http_fetcher.get_json_for_url(&id).await;
        match account {
            Ok(account) => match mapper::map_account_to_index(account) {
                Ok(account) => Ok(account),
                Err(error) => Err(error),
            },
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn get_instance(&self, domain: &String) -> Result<InstanceModel, reqwest::Error> {
        let instance_info_link = self.get_instance_info_link_for_domain(domain);

        match self
            .http_fetcher
            .get_json_for_url(&instance_info_link)
            .await
        {
            Ok(instance) => Ok(map_instance_to_index(instance)),
            Err(error) => Err(error),
        }
    }

    pub fn account_to_model(&self, account: &Account) -> Handle {
        let handle = Handle::from_string(&account.url).expect("Invalid handle");
        handle
    }
}
