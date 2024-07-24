use crate::adapters::http_fetcher::HttpFetcher;
use crate::adapters::node_info::NodeInfo;
use crate::adapters::node_info::WellKnownNodeInfo;

pub struct NodeInfoFetcher {
    pub http_fetcher: HttpFetcher,
}

impl NodeInfoFetcher {
    pub async fn get_node_info(&self, domain: &str) -> Result<NodeInfo, String> {
        let nodeinfo_link = self
            .get_nodeinfo_link_for_domain(domain)
            .await
            .unwrap_or_else(|_| "Unknown".to_string());
        println!("node info: {}", nodeinfo_link);
        let instance_info_result: Result<NodeInfo, reqwest::Error> =
            self.http_fetcher.get_json_for_url(&nodeinfo_link).await;
        match instance_info_result {
            Ok(instance_info_result) => Ok(instance_info_result),
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn get_nodeinfo_link_for_domain(
        &self,
        domain: &str,
    ) -> Result<std::string::String, reqwest::Error> {
        let url = format!("https://{}/.well-known/nodeinfo", domain);
        let node_info: WellKnownNodeInfo = self.http_fetcher.get_json_for_url(&url).await?;
        let s = String::from(&node_info.links[0].href);
        Ok(s)
    }
}
