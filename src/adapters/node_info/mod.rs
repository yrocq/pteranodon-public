use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub version: String,
    pub software: Software,
    pub protocols: Vec<String>

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Software {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WellKnownNodeInfo {
    pub links: Vec<Link>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String
}