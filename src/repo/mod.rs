pub mod postgres;

use crate::error::Error;
use crate::models::{dto::Entry, UserStorageEntry};

pub trait Key: ToString + Send + Sync {}
impl<K: ToString + Send + Sync> Key for K {}

#[async_trait]
pub trait Repo {
    async fn get(&self, key: impl Key) -> Result<Option<UserStorageEntry>, Error>;

    async fn mget(&self, keys: &[impl Key]) -> Result<Vec<UserStorageEntry>, Error>;

    async fn set(&self, key: impl Key, entry: Entry) -> Result<(), Error>;

    async fn mset(&self, items: &[(impl Key, Entry)]) -> Result<(), Error>;

    async fn mdel(&self, keys: &[impl Key]) -> Result<(), Error>;
}
