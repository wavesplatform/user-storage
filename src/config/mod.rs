pub mod api;
pub mod postgres;

use crate::error::Error;
use wavesexchange_repos::circuit_breaker;

#[derive(Debug)]
pub struct Config {
    pub api: api::Config,
    pub pg: postgres::Config,
    pub cb: circuit_breaker::Config,
}

pub fn load() -> Result<Config, Error> {
    Ok(Config {
        api: api::load()?,
        pg: postgres::load()?,
        cb: circuit_breaker::config::load()?,
    })
}
