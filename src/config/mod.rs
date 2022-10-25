pub mod api;
pub mod redis;

use crate::error::Error;

#[derive(Debug)]
pub struct Config {
    pub api: api::Config,
    pub redis: redis::Config,
}

pub fn load() -> Result<Config, Error> {
    Ok(Config {
        api: api::load()?,
        redis: redis::load()?,
    })
}
