use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Instance {
    pub uri: String,
    pub approval_required: bool,
    pub description: String,
    #[serde(default)]
    pub invites_enabled: bool,
    pub registrations: bool,
    pub short_description: String,
    pub stats: Stats,
    pub thumbnail: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub user_count: i32,
    pub status_count: i32,
    pub domain_count: i32,
}

#[derive(Serialize, Deserialize)]
struct Usage {
    users: Users,
}

#[derive(Serialize, Deserialize)]
struct Users {
    active_month: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Followers {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub welcome_type: Option<String>,
    #[serde(rename = "totalItems")]
    pub total_items: Option<i64>,
    pub next: Option<String>,
    pub first: Option<String>,
    #[serde(rename = "partOf")]
    pub part_of: Option<String>,
    #[serde(rename = "orderedItems")]
    #[serde(default)]
    pub ordered_items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "preferredUsername")]
    #[serde(default)]
    pub preferred_username: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub discoverable: bool,
    #[serde(rename = "movedTo")]
    #[serde(default)]
    pub moved_to: String,
}
