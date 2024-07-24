use crate::features::accounts;
use crate::models;
use crate::platforms::mastodon;
pub fn map_instance_to_index(instance: mastodon::models::Instance) -> crate::models::Instance {
    models::Instance {
        domain_name: instance.uri,
        title: instance.title,
        description: instance.description,
        short_description: instance.short_description,
        approval_required: instance.approval_required,
        thumbnail: instance.thumbnail,
        registrations: instance.registrations,
        invites_enabled: instance.invites_enabled,
        user_count: instance.stats.user_count,
        status_count: instance.stats.status_count,
        domain_count: instance.stats.domain_count,
    }
}

pub fn map_account_to_index(
    account: mastodon::models::Account,
) -> Result<accounts::account::Account, String> {
    Ok(accounts::account::Account {
        id: account.id,
        name: account.name,
        preferred_username: account.preferred_username,
        description: account.summary,
        discoverable: account.discoverable,
        url: account.url,
        moved_to: account.moved_to,
    })
}
