#[macro_use]
extern crate wavesexchange_log;

use lib::{api, config, db, error::Error, repo};
use wavesexchange_repos::circuit_breaker::CircuitBreaker;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config::load()?;

    info!("Starting user-storage service with config: {:?}", config);

    let cbrk = CircuitBreaker::builder_from_cfg(&config.cb)
        .with_init_fn(move || db::async_pool(&config.pg))
        .build()
        .unwrap();
    let storage_repo = repo::postgres::new(cbrk);

    api::start(config.api.port, config.api.metrics_port, storage_repo).await;
    Ok(())
}
