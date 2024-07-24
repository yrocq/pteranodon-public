pub mod adapters;
pub mod features;
pub mod models;
pub mod platforms;
pub mod repositories;
use crate::features::accounts::accounts_directory::AccountsDirectory;
use crate::features::accounts::accounts_indexer::AccountsIndexer;
use crate::repositories::postgresql::Postgres as Index;
use adapters::http_fetcher::HttpFetcher;
use clap::{Parser, Subcommand};
use features::{
    domains::{domains_directory::DomainsDirectory, DomainsIndexer},
    fediverse::Fediverse,
};
use platforms::mastodon::Mastodon;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover domains using instances from database
    DiscoverDomains {},
    /// Find a domain by name
    FindDomain { name: Option<String> },
    /// Update instances info in repository
    Update {},
    /// Store new instances information
    StoreInstances {},
    /// Index users from all instances
    IndexUsers {},
    /// Purge or archive expired instances
    PurgeInstances {},
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let scraper = Mastodon {
        http_fetcher: HttpFetcher {},
    };
    let index = Index::new().await;
    let mut accounts_directory = AccountsDirectory::new(&index);
    let mut domains_directory = DomainsDirectory::new(&index);
    let mut fediverse = Fediverse {
        index: &index, // TODO: Supprimer
        scraper: &scraper,
        accounts_indexer: AccountsIndexer::new(
            &mut accounts_directory,
            &mut domains_directory,
            &scraper,
        ),
        domains_indexer: DomainsIndexer::new(&index),
        accounts_directory: AccountsDirectory::new(&index),
        domains_directory: DomainsDirectory::new(&index),
    };

    match &cli.command {
        Commands::DiscoverDomains {} => {
            println!("Discover domains");
            fediverse.domains_indexer.discover_domains().await;
        }
        Commands::FindDomain { name } => {
            fediverse
                .domains_directory
                .find(name.as_ref().unwrap())
                .await;
        }
        Commands::StoreInstances {} => {
            println!("Store instances");
            fediverse.index_mastodon_instances().await;
        }
        Commands::Update {} => {
            fediverse.update().await;
        }

        Commands::IndexUsers {} => {
            fediverse.index_accounts().await;
        }

        Commands::PurgeInstances {} => {
            fediverse.purge_instances().await;
        }
    }
}
