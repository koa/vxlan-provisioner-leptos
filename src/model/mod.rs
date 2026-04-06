pub mod device;
pub mod jwt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct GoogleCredentials {
    pub client_id: String,
    pub issuer: String,
    pub auth_token_url: String,
    pub auth_url: String,
}
