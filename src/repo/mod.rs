pub mod postgres;

use crate::error::Error;
use crate::models::UserStorageEntry;

pub trait Key: ToString + Send + Sync {}
impl<K: ToString + Send + Sync> Key for K {}

#[async_trait]
pub trait Repo: Send + Sync + 'static {
    type Operations: RepoOperations;

    async fn interact<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Self::Operations) -> Result<R, Error>,
        F: Send + 'static,
        R: Send + 'static;

    async fn transaction<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Self::Operations) -> Result<R, Error>,
        F: Send + 'static,
        R: Send + 'static;
}

pub trait RepoOperations {
    fn get(&mut self, key: impl Key) -> Result<Option<UserStorageEntry>, Error>;

    fn mget(&mut self, keys: &[impl Key]) -> Result<Vec<UserStorageEntry>, Error>;

    fn set(&mut self, entry: &UserStorageEntry) -> Result<(), Error>;

    fn mset(&mut self, entries: &[UserStorageEntry]) -> Result<(), Error>;

    fn mdel(&mut self, keys: &[impl Key]) -> Result<(), Error>;
}
