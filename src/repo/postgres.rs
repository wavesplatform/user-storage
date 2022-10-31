use super::{Key, Repo, RepoOperations};
use crate::db::PgAsyncPool;
use crate::error::Error;
use crate::models::UserStorageEntry;
use crate::schema::user_storage;
use diesel::{prelude::*, upsert::excluded, PgConnection};

pub struct PgRepo {
    pool: PgAsyncPool,
}

#[async_trait]
impl Repo for PgRepo {
    type Operations = PgConnection;

    async fn interact<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Self::Operations) -> Result<R, Error>,
        F: Send + 'static,
        R: Send + 'static,
    {
        let conn = self.pool.get().await?;
        conn.interact(f).await.expect("deadpool interaction failed")
    }

    async fn transaction<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Self::Operations) -> Result<R, Error>,
        F: Send + 'static,
        R: Send + 'static,
    {
        let conn = self.pool.get().await?;
        conn.interact(|conn| conn.transaction(f))
            .await
            .expect("deadpool interaction failed")
    }
}

pub fn new(pool: PgAsyncPool) -> PgRepo {
    PgRepo { pool }
}

impl RepoOperations for PgConnection {
    fn get(&mut self, key: impl Key) -> Result<Option<UserStorageEntry>, Error> {
        let key = key.to_string();
        user_storage::table
            .filter(user_storage::key.eq(key))
            .first(self)
            .optional()
            .map_err(Error::from)
    }

    fn mget(&mut self, keys: &[impl Key]) -> Result<Vec<UserStorageEntry>, Error> {
        let keys = keys.into_iter().map(|k| k.to_string()).collect::<Vec<_>>();
        user_storage::table
            .filter(user_storage::key.eq_any(keys))
            .load(self)
            .map_err(Error::from)
    }

    fn set(&mut self, entry: &UserStorageEntry) -> Result<(), Error> {
        diesel::insert_into(user_storage::table)
            .values(entry)
            .on_conflict(user_storage::key)
            .do_update()
            .set(entry)
            .execute(self)
            .map_err(Error::from)?;
        Ok(())
    }

    fn mset(&mut self, entries: &[UserStorageEntry]) -> Result<(), Error> {
        diesel::insert_into(user_storage::table)
            .values(entries)
            .on_conflict(user_storage::key)
            .do_update()
            .set((
                user_storage::entry_type.eq(excluded(user_storage::entry_type)),
                user_storage::entry_value_binary.eq(excluded(user_storage::entry_value_binary)),
                user_storage::entry_value_boolean.eq(excluded(user_storage::entry_value_boolean)),
                user_storage::entry_value_integer.eq(excluded(user_storage::entry_value_integer)),
                user_storage::entry_value_json.eq(excluded(user_storage::entry_value_json)),
                user_storage::entry_value_string.eq(excluded(user_storage::entry_value_string)),
            ))
            .execute(self)
            .map_err(Error::from)?;
        Ok(())
    }

    fn mdel(&mut self, keys: &[impl Key]) -> Result<(), Error> {
        let keys = keys.into_iter().map(|k| k.to_string()).collect::<Vec<_>>();
        diesel::delete(user_storage::table.filter(user_storage::key.eq_any(keys)))
            .execute(self)
            .map_err(Error::from)?;
        Ok(())
    }
}
