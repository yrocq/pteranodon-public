use crate::features::accounts::account::Account;
use crate::models;
use crate::models::Instance;
use crate::{adapters::node_info::NodeInfo, features::domains::Domain};
use std::collections::HashMap;
use tokio_postgres::{Client, NoTls};

pub struct Postgres {
    pub client: Client,
    known_domains: HashMap<String, bool>,
}

impl Postgres {
    pub async fn new() -> Self {
        let known_domains = HashMap::new();

        let (client, connection) =
            tokio_postgres::connect(&std::env::var("DATABASE_URL").unwrap(), NoTls)
                .await
                .unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let postgres = Self {
            client,
            known_domains,
        };

        postgres
    }

    // Domains

    pub async fn get_domains_for_tag(&self, tag: &String) -> Vec<String> {
        let instances = self
            .client
            .query("SELECT domain_name FROM tags where tag=$1", &[tag])
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.get(0))
            .collect();
        instances
    }

    pub async fn get_domains_for_application(&self, application: &str) -> Vec<String> {
        self.client.query(
        "SELECT d.name FROM domains d LEFT JOIN instances i ON d.name = i.domain_name WHERE application=$1 AND i.id IS NULL",
        &[&application],
    ).await.unwrap().into_iter().map(|row| row.get(0)).collect()
    }

    pub async fn get_known_domains(&mut self) {
        self.client
            .query("SELECT name FROM domains", &[])
            .await
            .unwrap()
            .into_iter()
            .for_each(|row| {
                let domain: &str = row.get(0);
                self.known_domains.insert(String::from(domain), true);
            });
    }

    pub async fn store_domain(&self, domain: &str, node_info: &NodeInfo) {
        self.client
            .execute(
                "INSERT INTO domains (name, application, status) VALUES ($1, $2, 'ok')",
                &[&domain, &node_info.software.name],
            )
            .await
            .expect("error");
    }

    pub async fn store_domain_error(&self, domain: &str, error: &str) {
        self.client
            .execute(
                "INSERT INTO domains (name, status, meta) VALUES ($1, 'error', $2)",
                &[&domain, &error],
            )
            .await
            .expect("error");
    }

    pub async fn get_domain(&self, domain: &str) -> Option<Domain> {
        let result = self
            .client
            .query(
                "SELECT
            name,
            application,
            status
            FROM domains WHERE name=$1",
                &[&domain],
            )
            .await;

        match result {
            Ok(rows) => {
                if rows.len() == 0 {
                    println!("No domain for {}", domain);
                    None
                } else {
                    match rows.get(0) {
                        Some(row) => Some(Domain {
                            name: row.get("name"),
                            application: row.get("application"),
                            status: row.get("status"),
                        }),
                        None => {
                            println!("No domain for {}", domain);
                            None
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error : {}", error);
                None
            }
        }
    }

    pub async fn purge_domain_error_cache(&self) {
        let result = self
            .client
            .execute(
                "DELETE FROM domains
            WHERE status='error' AND EXTRACT(DAY FROM NOW() - created_at) > 30",
                &[],
            )
            .await;

        match result {
            Ok(n) => {
                println!("{} domains(s) deleted", n);
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    // Évaluer https://crates.io/crates/sql-builder

    pub async fn update_instance_info(&self, domain: &String, instance: Instance) {
        self.client
            .query(
                "UPDATE instances SET 
                        title = $2,
                        short_description = $3,
                        description = $4,
                        user_count = $5,
                        status_count = $6,
                        domain_count = $7,
                        thumbnail = $8,
                        registrations = $9,
                        updated_at = now() WHERE domain_name=$1",
                &[
                    &domain,
                    &instance.title,
                    &instance.short_description,
                    &instance.description,
                    &instance.user_count,
                    &instance.status_count,
                    &instance.domain_count,
                    &instance.thumbnail,
                    &instance.registrations,
                ],
            )
            .await
            .expect("Update error");
    }

    pub async fn store_instance(&self, domain: &str, instance: models::Instance) {
        println!("{}", instance.title);
        // https://vasilakisfil.social/blog/2020/05/09/rust-diesel-jsonb/

        self.client
            .execute(
                "INSERT INTO instances (
                    domain_name,
                    title,
                    short_description,
                    description,
                    user_count,
                    status_count,
                    domain_count,
                    thumbnail,
                    registrations,
                    invites_enabled,
                    approval_required,
                    status
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'ok')",
                &[
                    &domain,
                    &instance.title,
                    &instance.short_description,
                    &instance.description,
                    &instance.user_count,
                    &instance.status_count,
                    &instance.domain_count,
                    &instance.thumbnail,
                    &instance.registrations,
                    &instance.invites_enabled,
                    &instance.approval_required,
                ],
            )
            .await
            .expect("error");
    }

    pub async fn store_account(&self, account: Account) {
        self.client
            .execute(
                "INSERT INTO accounts(
                    id,
                    name,
                    preferred_username,
                    url,
                    description
                ) VALUES($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING;",
                &[
                    &account.id,
                    &account.name,
                    &account.preferred_username,
                    &account.url,
                    &account.description,
                ],
            )
            .await
            .expect("error");
    }

    pub async fn get_accounts(&mut self) -> Vec<String> {
        let accounts = self
            .client
            .query("SELECT url FROM accounts ORDER BY random()", &[])
            .await
            .unwrap()
            .iter()
            .map(|row| row.get(0))
            .collect();
        accounts
    }
    pub async fn get_account_for_id(&self, id: &str) -> Option<Account> {
        let result = self
            .client
            .query("SELECT * FROM accounts WHERE id=$1", &[&id])
            .await;

        match result {
            Ok(rows) => match rows.get(0) {
                Some(row) => Some(Account {
                    id: row.get("id"),
                    name: row.get("name"),
                    preferred_username: row.get("preferred_username"),
                    discoverable: true,
                    url: row.get("url"),
                    moved_to: "".to_string(),
                    description: "".to_string(),
                }),
                None => None,
            },
            Err(error) => {
                println!("Error : {}", error);
                None
            }
        }
    }

    pub async fn get_ignored_users(&self) -> Vec<String> {
        let accounts = self
            .client
            .query(
                "SELECT url FROM accounts WHERE status='blacklist' OR status='ignored'",
                &[],
            )
            .await
            .unwrap()
            .iter()
            .map(|row| row.get(0))
            .collect();
        accounts
    }

    pub async fn get_instance_for_domain(&self, domain: &String) -> Option<Instance> {
        let result = self
            .client
            .query(
                "SELECT
        domain_name,
        title,
        description,
        short_description,
        thumbnail,
        registrations,
        invites_enabled,
        approval_required,
        user_count,
        status_count,
        domain_count
        FROM domains d INNER JOIN instances i ON d.name = i.domain_name WHERE d.name=$1",
                &[&domain],
            )
            .await;

        match result {
            Ok(rows) => {
                if rows.len() == 0 {
                    println!("No instance for {}", domain);
                    None
                } else {
                    match rows.get(0) {
                        Some(row) => Some(Instance {
                            domain_name: row.get("domain_name"),
                            title: row.get("title"),
                            description: row.get("description"),
                            short_description: row.get("short_description"),
                            thumbnail: row.get("thumbnail"),
                            registrations: row.get("registrations"),
                            invites_enabled: row.get("invites_enabled"),
                            approval_required: row.get("approval_required"),
                            user_count: row.get("user_count"),
                            status_count: row.get("status_count"),
                            domain_count: row.get("domain_count"),
                        }),
                        None => {
                            println!("No instance for {}", domain);
                            None
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error : {}", error);
                None
            }
        }
    }

    pub async fn purge_dead_instances(&self) {
        let result = self
            .client
            .execute(
                "DELETE FROM instances
            WHERE EXTRACT(DAY FROM NOW() - updated_at) > 7",
                &[],
            )
            .await;

        match result {
            Ok(n) => {
                println!("{} instance(s) deleted", n);
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }
}
