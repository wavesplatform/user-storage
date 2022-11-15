use lib::{api, config, db, error::Error, repo};
use wavesexchange_log::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config::load()?;

    info!(
        "Starting user-storage service with config: {:?}",
        config.api
    );

    let pg_pool = db::async_pool(&config.pg)?;
    let storage_repo = repo::postgres::new(pg_pool);

    api::start(config.api.port, config.api.metrics_port, storage_repo).await;

    Ok(())
}
