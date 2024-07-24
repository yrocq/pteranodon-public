use super::{Account, AccountEntry};
use crate::{adapters::cache::Cache, repositories::postgresql::Postgres};

pub struct AccountsDirectory<'a> {
    cache: Cache<AccountEntry>,
    index: &'a Postgres,
}

impl<'a> AccountsDirectory<'a> {
    pub fn new(index: &Postgres) -> AccountsDirectory {
        AccountsDirectory {
            cache: Cache::new(),
            index,
        }
    }

    pub fn cache_as_followed(&mut self, id: &str) {
        self.cache.insert(
            id,
            AccountEntry {
                is_indexed: false,
                is_followed: true,
            },
        )
    }

    pub async fn is_followed(&mut self, id: &str) -> bool {
        match self.cache.get(id) {
            Some(account_entry) => account_entry.is_followed,
            None => false,
        }
    }

    pub fn is_known(&self, id: &str) -> bool {
        return self.cache.contains(id);
    }

    pub async fn index(&self, account: Account) {
        self.index.store_account(account).await;
    }
}
