use crate::adapters::node_info::NodeInfo;

#[derive(Clone)]
pub struct Domain {
    pub name: String,
    pub application: Option<String>,
    pub status: String,
}

impl Domain {
    pub fn from_node_info(domain: &str, node_info: &NodeInfo, status: &str) -> Domain {
        let application = if node_info.software.name.is_empty() {
            None
        } else {
            Some(node_info.software.name.clone())
        };
        Domain {
            name: domain.to_string(),
            application,
            status: status.to_string(),
        }
    }

    pub fn new_domain_error(name: &str) -> Domain {
        Domain {
            name: name.to_string(),
            application: None,
            status: "error".to_string(),
        }
    }
}
