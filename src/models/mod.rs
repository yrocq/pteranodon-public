pub struct Instance {
    pub domain_name: String,
    pub title: String,
    pub description: String,
    pub short_description: String,
    pub thumbnail: String,
    pub registrations: bool,
    pub invites_enabled: bool,
    pub approval_required: bool,
    pub user_count: i32,
    pub status_count: i32,
    pub domain_count: i32,
}
