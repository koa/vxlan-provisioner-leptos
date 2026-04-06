use crate::{model::jwt::Claims, server::error::AuthError, server::google_issuer};
use cached::proc_macro::cached;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use log::info;
use serde::Deserialize;
use std::{sync::Arc, time::Duration};

#[derive(Debug, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String, // Key ID
    n: String,   // RSA Modulus
    e: String,   // RSA Exponent
}
#[derive(Debug, Deserialize)]
struct OpenIdConfig {
    issuer: String,
    jwks_uri: String,
}
pub async fn validate_token(token: &str) -> Result<Claims, AuthError> {
    // 1. Header dekodieren, um die Key-ID (kid) zu finden
    let header = decode_header(token)?;
    let kid = header.kid.ok_or(AuthError::MissingHeaderKid)?;

    let decoding_key = decode_key(kid).await?;

    let validation = validation();

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

#[cached(time = 600, result = true)]
async fn decode_key(kid: String) -> Result<DecodingKey, AuthError> {
    let jwks = fetching_jwks().await?;

    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or(AuthError::MissingMatchingKey)?;

    Ok(DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?)
}

#[cached(time = 600)]
fn validation() -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    let issuer = google_issuer();
    validation.set_issuer(&[issuer]);
    validation.validate_aud = false;
    validation
}

#[cached(time = 600, result = true)]
async fn fetching_jwks() -> Result<Arc<Jwks>, AuthError> {
    let issuer = google_issuer();
    let config_url = format!(
        "{}/.well-known/openid-configuration",
        issuer.trim_end_matches('/')
    );
    let client = reqwest::Client::new();
    let config: OpenIdConfig = client.get(&config_url).send().await?.json().await?;
    Ok(Arc::new(reqwest::get(config.jwks_uri).await?.json().await?))
}
