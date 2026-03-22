use crate::model::GoogleCredentials;
use crate::server::config::CONFIG;
use crate::server::graphql::fetch_topology;
use std::sync::Arc;

mod config;
mod error;
mod graphql;

pub async fn list_devices() -> Result<Vec<(u32, String)>, Arc<error::NetboxError>> {
    let topology = fetch_topology().await?;
    let devices = topology
        .device
        .values()
        .filter_map(|device| device.name.as_ref().map(|name| (device.id.0, name.clone())))
        .collect::<Vec<_>>();
    Ok(devices)
}
pub fn google_credentials() -> GoogleCredentials {
    CONFIG.google_credentials.clone()
}
