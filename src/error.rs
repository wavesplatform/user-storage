use redis::RedisError;
use warp::reject::Reject;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("LoadConfigFailed: {0}")]
    LoadConfigFailed(#[from] envy::Error),

    #[error("ValidationError: {0}")]
    ValidationError(String, Option<std::collections::HashMap<String, String>>),

    #[error("RedisError: {0}")]
    RedisError(#[from] RedisError),

    #[error("Bb8Error: {0}")]
    Bb8Error(String),

    #[error("KeyNotFound: {0}")]
    KeyNotFound(String),

    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
}

impl Reject for Error {}
