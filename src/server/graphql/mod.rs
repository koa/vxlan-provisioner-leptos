use crate::server::{config::CONFIG, error::NetboxError};
use cached::proc_macro::cached;
use cynic::{http::ReqwestExt, QueryBuilder};
use model::{Query, TopologyData};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    RequestBuilder,
};
use std::{sync::Arc, time::Duration};
pub mod model;

#[cached(time = 20)]
pub async fn fetch_topology() -> Result<Arc<TopologyData>, Arc<NetboxError>> {
    let client = netbox_client().map_err(Arc::new)?;
    let operation = Query::build(());
    //println!("Query\n{}", operation.query);

    let builder: RequestBuilder = client.post(netbox_url());
    let result = builder
        .run_graphql(operation)
        .await
        .map_err(|e| Arc::new(e.into()))?;
    if let Some(errors) = result.errors {
        Err(Arc::new(NetboxError::ErrorFromServer(
            errors.into_boxed_slice(),
        )))
    } else if let Some(data) = result.data {
        Ok(Arc::new(data.into()))
    } else {
        Err(Arc::new(NetboxError::EmptyResult))
    }
}

fn netbox_url() -> &'static str {
    CONFIG.netbox_url.as_str()
}

fn netbox_client() -> Result<reqwest::Client, NetboxError> {
    let mut headers = HeaderMap::new();
    let access_token = CONFIG.netbox_token.as_str();
    headers.insert(AUTHORIZATION, format!("Token {access_token}").parse()?);

    Ok(reqwest::Client::builder()
        .default_headers(headers)
        .build()?)
}
