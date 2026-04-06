use crate::server::error::ServerError;
use crate::{
    model::GoogleCredentials,
    server::{config::CONFIG, graphql::fetch_topology, jwt::validate_token},
};
use log::info;
use std::sync::Arc;

mod config;
mod error;
mod graphql;
pub mod jwt;

pub async fn list_devices(token: &str) -> Result<Vec<(u32, String)>, Arc<ServerError>> {
    let result = validate_token(token).await;
    info!("Token claims: {:?}", result);
    result.map_err(|e| Arc::new(e.into()))?;
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
pub fn google_client_id() -> &'static str {
    CONFIG.google_credentials.client_id.as_str()
}
pub fn google_issuer() -> &'static str {
    CONFIG.google_credentials.issuer.as_str()
}
