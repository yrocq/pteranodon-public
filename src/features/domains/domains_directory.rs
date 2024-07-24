use crate::{
    adapters::{cache::Cache, http_fetcher::HttpFetcher, nodeinfo_fetcher::NodeInfoFetcher},
    repositories::postgresql::Postgres as Index,
};
use regex::Regex;

use super::domain::Domain;

pub struct DomainsDirectory<'a> {
    pub nodeinfo_fetcher: NodeInfoFetcher,
    cache: Cache<Domain>,
    index: &'a Index,
}

impl<'a> DomainsDirectory<'a> {
    pub fn new(index: &Index) -> DomainsDirectory {
        DomainsDirectory {
            nodeinfo_fetcher: NodeInfoFetcher {
                http_fetcher: HttpFetcher {},
            },
            cache: Cache::new(),
            index,
        }
    }

    pub async fn domain_application_is(&self, domain: &str, application: &str) -> bool {
        match self.cache.get(domain) {
            Some(domain) => domain
                .application
                .as_ref()
                .is_some_and(|a| a == application),
            None => false,
        }
    }

    pub async fn find(&mut self, name: &str) -> Option<Domain> {
        match self.cache.get(name) {
            Some(domain) => Some(domain.clone()),
            None => match self.index.get_domain(name).await {
                Some(domain) => {
                    self.cache.insert(name, domain.clone());
                    Some(domain)
                }
                None => None,
            },
        }
    }

    pub async fn cache(&self, index: &mut Index, name: &str) -> Option<Domain> {
        return index.get_domain(&(name.to_string())).await;
    }

    pub async fn fetch(&mut self, name: &str) -> Domain {
        match self.find(name).await {
            Some(domain) => {
                println!("Known domain: {}", name);
                domain.clone()
            }
            None => {
                // TODO: Move to list management
                let re = Regex::new(r".*(activitypub-troll.cf)$").unwrap();
                if !re.is_match(name) {
                    println!("Fetch node info for {}: ", name);
                    let node_info = self.nodeinfo_fetcher.get_node_info(name).await;
                    let domain = match &node_info {
                        Ok(node_info) => {
                            let new_domain = Domain::from_node_info(&name, &node_info, "ok");
                            println!("{}", node_info.software.name);
                            // TODO: move to save function
                            self.index.store_domain(name, node_info).await;
                            new_domain
                        }
                        Err(error) => {
                            println!("Domain error for {} : {}", name, error);
                            let new_domain = Domain::new_domain_error(name);
                            self.index.store_domain_error(name, &error).await;
                            new_domain
                        }
                    };
                    self.cache.insert(name, domain.clone());
                    domain
                } else {
                    let domain = Domain {
                        name: name.to_string(),
                        application: None,
                        status: "error".to_string(),
                    };
                    self.cache.insert(name, domain.clone());
                    domain
                }
            }
        }
    }

    pub async fn should_not_index(&mut self, name: &str) -> bool {
        match self.find(&name).await {
            None => false,
            Some(domain) => domain.status == "ignored" || domain.status == "blacklist",
        }
    }

    pub fn is_known(&self, id: &str) -> bool {
        return self.cache.contains(id);
    }

    pub async fn is_error(&mut self, id: &str) -> bool {
        let domain = self.fetch(id).await;
        return domain.status == "error";
    }
}
