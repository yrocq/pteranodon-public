use crate::features::accounts::AccountsIndexer;
use crate::features::domains::domains_indexer::DomainsIndexer;
use crate::platforms::mastodon::Mastodon;
use crate::repositories::postgresql::Postgres as Index;

use super::accounts::accounts_directory::AccountsDirectory;
use super::domains::domains_directory::DomainsDirectory;
pub struct Fediverse<'a> {
    pub scraper: &'a Mastodon, // TODO: supprimer
    pub index: &'a Index,      // TODO: supprimer
    pub domains_indexer: DomainsIndexer<'a>,
    pub accounts_indexer: AccountsIndexer<'a>,
    pub accounts_directory: AccountsDirectory<'a>,
    pub domains_directory: DomainsDirectory<'a>,
}

impl<'a> Fediverse<'a> {
    pub async fn index_mastodon_instances(&mut self) {
        let domains = self.index.get_domains_for_application("mastodon").await;
        self.index_instances(domains).await;
    }

    pub async fn index_instances(&mut self, domains: Vec<String>) {
        for domain in domains {
            println!("{}", domain);
            self.index_instance(&domain).await;
        }
    }

    async fn index_instance(&self, domain: &String) {
        match self.index.get_instance_for_domain(&domain).await {
            Some(_) => {
                println!("Instance {} already indexed", domain);
            }
            None => match self.scraper.get_instance(domain).await {
                Ok(instance) => {
                    self.index.store_instance(&domain, instance).await;
                }
                Err(error) => {
                    println!("Error store instance: {}", error)
                }
            },
        }
    }

    pub async fn update(&mut self) {
        self.update_instances_with_tag("quebec").await;
    }

    pub async fn update_instances_with_tag(&mut self, tag: &str) {
        let instances = self.index.get_domains_for_tag(&tag.to_string()).await;
        for instance in instances {
            let instance_info_result = self.scraper.get_instance(&instance).await;

            match instance_info_result {
                Ok(instance_info) => {
                    println!("Updating: {}", instance_info.title);

                    self.index
                        .update_instance_info(&instance, instance_info)
                        .await
                }
                Err(error) => println!("{}", error),
            }
        }
    }

    pub async fn purge_instances(&self) {
        println!("Purging");
        self.index.purge_dead_instances().await;
        self.index.purge_domain_error_cache().await;
    }

    pub async fn index_accounts(&mut self) {
        self.accounts_indexer.index_accounts().await;
    }

    pub async fn find_domain(&self, name: &str) {
        if let Some(info) = self.index.get_domain(&name.to_string()).await {
            println!("{}", info.name)
        }
    }
}
