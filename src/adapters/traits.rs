use crate::platforms::mastodon::models::{Account, Instance};

use super::handle::Handle;

pub trait InstanceRepository {
    fn update_instances_with_tag(&mut self, tag: &str);
    fn get_domains_for_tag(&mut self, tag: &String) -> Vec<String>;
    fn get_instance(&self, domain: &String) -> Result<Instance, std::io::Error>;
    fn get_instance_info_link_for_domain(&self, domain: &String)  -> String;
    fn update_instance(&mut self, domain: &String, instance: Instance);
    fn get_peers_domains_from_domain(&self, domain: &str) -> Result<Vec<String>, std::io::Error>;
    fn index_instances(&mut self, domains: Vec<String>);
    fn index_instance(&mut self, domain: &String);
    fn get_followers(&mut self, handle: &Handle) -> Result<Vec<Handle>, std::io::Error>;
    fn get_account(&mut self, handle: &Handle) -> Result<Account, std::io::Error>;
    // fn store_mastodon_instances(&self);
    // fn store_instance(self, domain: &str, instance_info: String);
}
