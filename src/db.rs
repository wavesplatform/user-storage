use deadpool_diesel::{Manager as DManager, Pool as DPool, Runtime};
use diesel::pg::PgConnection;
use std::time::Duration;
use wavesexchange_repos::circuit_breaker::FallibleDataSource;

use crate::config::postgres::Config;
use crate::error::Error;

pub struct PgAsyncPool(pub DPool<DManager<PgConnection>>);

pub fn generate_postgres_url(config: &Config) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database
    )
}

pub fn async_pool(config: &Config) -> Result<PgAsyncPool, Error> {
    let db_url = generate_postgres_url(config);

    let manager = DManager::new(db_url, Runtime::Tokio1);
    let pool = DPool::builder(manager)
        .max_size(config.poolsize as usize)
        .wait_timeout(Some(Duration::from_secs(5)))
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|e| Error::GeneralError(e.to_string()))?;
    Ok(PgAsyncPool(pool))
}

impl FallibleDataSource for PgAsyncPool {
    const REINIT_ON_FAIL: bool = true;
    type Error = Error;

    fn is_countable_err(err: &Self::Error) -> bool {
        //err.to_string().contains("no connection to the server")
        true
    }
}
