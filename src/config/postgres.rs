use crate::error::Error;
use serde::Deserialize;

fn default_pgport() -> u16 {
    5432
}

fn default_pg_poolsize() -> u8 {
    4
}

#[derive(Deserialize)]
struct ConfigFlat {
    host: String,
    #[serde(default = "default_pgport")]
    port: u16,
    database: String,
    user: String,
    password: String,
    #[serde(default = "default_pg_poolsize")]
    poolsize: u8,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub poolsize: u8,
}

pub fn load() -> Result<Config, Error> {
    let config_flat = envy::prefixed("PG").from_env::<ConfigFlat>()?;

    Ok(Config {
        host: config_flat.host,
        port: config_flat.port,
        user: config_flat.user,
        database: config_flat.database,
        password: config_flat.password,
        poolsize: config_flat.poolsize,
    })
}
