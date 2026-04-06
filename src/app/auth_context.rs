use crate::model::jwt::Claims;
#[cfg(feature = "ssr")]
use crate::server::google_client_id;
#[cfg(feature = "hydrate")]
use google_signin_client::{prompt_async, IdConfiguration};
use leptos::{
    context::use_context,
    prelude::{provide_context, server, Effect, Get, RwSignal, ServerFnError},
};
#[cfg(feature = "hydrate")]
use leptos::{prelude::Update, task::spawn_local};

#[cfg(feature = "hydrate")]
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuthData {
    pub token: Option<JwtToken>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub token: Box<str>,
    pub claims: Claims,
}

impl JwtToken {
    pub fn is_valid(&self) -> bool {
        self.claims.not_expired()
    }
}

impl AuthData {
    pub fn is_authenticated(&self) -> bool {
        self.token
            .as_ref()
            .map(|t| t.claims.not_expired())
            .unwrap_or(false)
    }
    pub fn valid_token(&self) -> Option<&str> {
        self.token.as_ref().map(|t| t.token.as_ref())
    }
}

// Wir wickeln das in ein Signal ein, damit die UI bei Änderungen reagiert
#[derive(Copy, Clone, Debug)]
pub struct AuthContext(pub RwSignal<AuthData>);

impl AuthContext {
    pub fn valid_token() -> Option<Box<str>> {
        use_context::<AuthContext>()
            .map(|c| c.0.get())
            .and_then(|a| a.valid_token().map(Box::from))
    }
}

pub fn provide_auth() {
    let auth_signal = RwSignal::new(AuthData { token: None });
    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            let auth = auth_signal.get();
            if !auth.is_authenticated() {
                spawn_local(async move {
                    if let Ok(client_id) = login_settings().await {
                        let mut configuration = IdConfiguration::new(client_id);
                        configuration.set_callback(Box::new(move |token| {
                            auth_signal.update(|data| {
                                let credential = token.credential();
                                match Claims::parse_jwt_payload(credential) {
                                    Ok(claims) => {
                                        info!("{:?}", claims);
                                        data.token = Some(JwtToken {
                                            token: Box::from(credential),
                                            claims,
                                        })
                                    }
                                    Err(e) => {
                                        error!("Error on JWT Token: {}", e);
                                    }
                                }
                            });
                        }));
                        google_signin_client::initialize(configuration);
                        let login_result = prompt_async().await;
                        info!("{:?}", login_result);
                    }
                })
            }
        }
    });

    provide_context(AuthContext(auth_signal));
}

#[server]
async fn login_settings() -> Result<String, ServerFnError> {
    Ok(google_client_id().to_string())
}
