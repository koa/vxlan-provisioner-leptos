use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Settings {
    pub netbox_url: String,
    pub netbox_token: String,
    pub mikrotik_credentials: HashMap<Box<str>, MikrotikCredentials>,
}

#[derive(Deserialize)]
pub struct MikrotikCredentials {
    user: Option<Box<str>>,
    password: Option<Box<str>>,
}

impl MikrotikCredentials {
    pub fn user(&self) -> &str {
        self.user.as_ref().map(Box::as_ref).unwrap_or("admin")
    }
    pub fn password(&self) -> Option<&str> {
        self.password.as_ref().map(Box::as_ref)
    }
}

fn create_settings() -> Result<Settings, ConfigError> {
    let cfg = Config::builder()
        .add_source(File::with_name("config.yaml"))
        .add_source(Environment::with_prefix("app"))
        .build()?;
    let settings: Settings = cfg.get("settings")?;
    Ok(settings)
}

lazy_static! {
    pub static ref CONFIG: Settings = create_settings().expect("Cannot load config.yaml");
}
