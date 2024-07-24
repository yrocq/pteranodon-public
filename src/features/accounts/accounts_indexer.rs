// use super::{accounts_directory, AccountEntry};
use super::{accounts_directory::AccountsDirectory, Account};
use crate::adapters::analyzer::is_quebec;
use crate::adapters::handle::Handle;
use crate::features::domains::domains_directory::DomainsDirectory;
use crate::platforms::mastodon::Mastodon;

pub struct AccountsIndexer<'a> {
    accounts_directory: &'a mut AccountsDirectory<'a>,
    domains_directory: &'a mut DomainsDirectory<'a>,
    scraper: &'a Mastodon,
}

impl<'a> AccountsIndexer<'a> {
    pub fn new(
        accounts_directory: &'a mut AccountsDirectory<'a>,
        domains_directory: &'a mut DomainsDirectory<'a>,
        scraper: &'a Mastodon,
    ) -> AccountsIndexer<'a> {
        AccountsIndexer {
            accounts_directory,
            domains_directory,
            scraper,
        }
    }

    pub async fn index_accounts(&mut self) {
        panic!("Not implemented")
    }
}
