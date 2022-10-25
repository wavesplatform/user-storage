pub mod redis;

use crate::api::dtos::Entry;
use crate::error::Error;

#[async_trait]
pub trait Repo {
    async fn get(&self, key: &str) -> Result<Option<Entry>, Error>;

    async fn mget(&self, keys: &[&str]) -> Result<Vec<Option<Entry>>, Error>;

    async fn set(&self, key: &str, entry: &Entry) -> Result<(), Error>;

    async fn mset(&self, items: &[(&str, &Entry)]) -> Result<(), Error>;

    async fn del(&self, keys: &[&str]) -> Result<(), Error>;
}
