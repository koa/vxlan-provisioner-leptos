#[cfg(feature = "hydrate")]
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Box<str>,
    pub aud: Box<str>,
    pub iss: Box<str>,
    pub email: Box<str>,
    pub email_verified: bool,
    pub hd: Option<Box<str>>,
    pub name: Box<str>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub iat: DateTime<Utc>,
}

impl Claims {
    #[cfg(feature = "hydrate")]
    pub fn parse_jwt_payload(token: &str) -> Result<Self, String> {
        // 1. JWT in seine 3 Teile zerlegen
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err("Ungültiges JWT-Format".to_string());
        }

        // 2. Den Payload-Teil (Index 1) nehmen
        let payload_b64 = parts[1];

        // 3. Base64 dekodieren (URL_SAFE_NO_PAD ist der Standard für JWTs)
        let decoded_bytes = BASE64_URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(|e| format!("Base64-Dekodierung fehlgeschlagen: {}", e))?;

        // 4. In die Rust-Struktur umwandeln
        let claims: Claims = serde_json::from_slice(&decoded_bytes)
            .map_err(|e| format!("JSON-Parsing fehlgeschlagen: {}", e))?;

        Ok(claims)
    }
    pub fn not_expired(&self) -> bool {
        self.exp > Utc::now()
    }
}
