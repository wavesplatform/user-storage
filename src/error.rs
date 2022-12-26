use std::collections::HashMap;
use warp::reject::Reject;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("LoadConfigFailed: {0}")]
    LoadConfigFailed(#[from] envy::Error),

    #[error("ValidationError: {0}")]
    ValidationError(String, Option<HashMap<String, String>>),

    #[error("KeyNotFound: {0}")]
    KeyNotFound(String),

    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("DbDieselError: {0}")]
    DbDieselError(#[from] diesel::result::Error),

    #[error("PoolError")]
    PoolError(#[from] deadpool_diesel::PoolError),

    #[error("GeneralError: {0}")]
    GeneralError(String),
}

impl Reject for Error {}
