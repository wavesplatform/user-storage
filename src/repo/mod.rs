pub mod postgres;

use crate::error::Error;
use crate::models::{UserAddress, UserStorageEntry};

pub trait Key: ToString + Send + Sync {}
impl<K: ToString + Send + Sync> Key for K {}

#[async_trait]
pub trait Repo: Send + Sync + 'static {
    type Operations: RepoOperations;

    async fn interact<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Self::Operations) -> Result<R, Error>,
        F: Send + Sync + 'static,
        R: Send + Sync + 'static;

    async fn transaction<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Self::Operations) -> Result<R, Error>,
        F: Send + Sync + 'static,
        R: Send + Sync + 'static;
}

pub trait RepoOperations {
    fn get(
        &mut self,
        user_addr: &UserAddress,
        key: impl Key,
    ) -> Result<Option<UserStorageEntry>, Error>;

    fn mget(
        &mut self,
        user_addr: &UserAddress,
        keys: &[impl Key],
    ) -> Result<Vec<UserStorageEntry>, Error>;

    fn set(&mut self, entry: &UserStorageEntry) -> Result<(), Error>;

    fn mset(&mut self, entries: &[UserStorageEntry]) -> Result<(), Error>;

    fn mdel(&mut self, user_addr: &UserAddress, keys: &[impl Key]) -> Result<(), Error>;
}
