use wavesexchange_log::info;

mod api;
mod config;
mod db;
mod error;
mod models;
mod repo;
mod schema;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let config = config::load()?;

    info!("Starting user-storage service with config: {:?}", config);

    let pg_pool = db::async_pool(&config.pg).await?;
    let storage_repo = repo::postgres::new(pg_pool);

    api::start(config.api.port, config.api.metrics_port, storage_repo).await;

    Ok(())
}
