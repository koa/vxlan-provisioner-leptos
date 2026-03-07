use cynic::http::CynicReqwestError;
use cynic::GraphQlError;
use mikrotik_model::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetboxError {
    #[error("accessing netbox API {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("errors from netbox {0:?}")]
    Graphql(#[from] CynicReqwestError),
    #[error("errors from netbox {0:?}")]
    ErrorFromServer(Box<[GraphQlError]>),
    #[error("cannot call netbox {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("no data from netbox")]
    EmptyResult,
}
#[derive(Debug, Error)]
pub enum CommunicationError {
    #[error("missing mikrotik credentials")]
    MissingCredentials,
    #[error("mikrotik error {0}")]
    Mikrotik(#[from] mikrotik_model::mikrotik_api::Error),
    #[error("mikrotik model error {0}")]
    MikrotikModel(#[from] mikrotik_model::error::Error),
}
