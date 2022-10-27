pub mod api;
pub mod postgres;

use crate::error::Error;

#[derive(Debug)]
pub struct Config {
    pub api: api::Config,
    pub pg: postgres::Config,
}

pub fn load() -> Result<Config, Error> {
    Ok(Config {
        api: api::load()?,
        pg: postgres::load()?,
    })
}
