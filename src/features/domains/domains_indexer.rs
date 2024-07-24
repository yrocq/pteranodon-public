use crate::{
    adapters::{http_fetcher::HttpFetcher, nodeinfo_fetcher::NodeInfoFetcher},
    platforms::mastodon::Mastodon,
    repositories::postgresql::Postgres,
};

use super::domains_directory::DomainsDirectory;

pub struct DomainsIndexer<'a> {
    pub nodeinfo_fetcher: NodeInfoFetcher,
    domains_directory: DomainsDirectory<'a>,
    index: &'a Postgres,
    scraper: Mastodon,
}

impl<'a> DomainsIndexer<'a> {
    pub fn new(index: &'a Postgres) -> DomainsIndexer<'a> {
        DomainsIndexer {
            nodeinfo_fetcher: NodeInfoFetcher {
                http_fetcher: HttpFetcher::new(),
            },
            domains_directory: DomainsDirectory::new(&index),
            index,
            scraper: Mastodon {
                http_fetcher: HttpFetcher::new(),
            },
        }
    }
    pub async fn discover_domains(&mut self) {
        let mastodon_domains = self.index.get_domains_for_tag(&"quebec".to_string()).await;
        for domain in mastodon_domains {
            match self.scraper.get_peers_domains_from_domain(&domain).await {
                Ok(peer_domains) => {
                    println!("*** Peers for domain {}***", domain);
                    for peer_domain in peer_domains {
                        println!("Peer domain: {}", peer_domain);
                        self.domains_directory.fetch(&peer_domain).await;
                    }
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }
    }
}
