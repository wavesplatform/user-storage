use wavesexchange_log::info;

mod api;
mod async_redis;
mod config;
mod error;
mod repo;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let config = config::load()?;

    info!("Starting user-storage service with config: {:?}", config);

    let redis_pool = async_redis::pool(&config.redis).await?;
    let storage_repo = repo::redis::new(redis_pool);

    api::start(config.api.port, config.api.metrics_port, storage_repo).await;

    Ok(())
}
