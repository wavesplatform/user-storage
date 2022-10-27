use super::{Key, Repo};
use crate::db::PgAsyncPool;
use crate::error::Error;
use crate::models::{dto::Entry, UserStorageEntry};
use crate::schema::user_storage;
use diesel::{prelude::*, PgConnection};

pub struct PgRepo {
    pool: PgAsyncPool,
}

impl PgRepo {
    pub async fn interact<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut PgConnection) -> Result<R, Error>,
        F: Send + 'static,
        R: Send + 'static,
    {
        let conn = self.pool.get().await?;
        conn.interact(f).await.expect("deadpool interaction failed")
    }
}

pub fn new(pool: PgAsyncPool) -> PgRepo {
    PgRepo { pool }
}

#[async_trait]
impl Repo for PgRepo {
    async fn get(&self, key: impl Key) -> Result<Option<UserStorageEntry>, Error> {
        let key = key.to_string();
        self.interact(|conn| {
            user_storage::table
                .filter(user_storage::key.eq(key))
                .first(conn)
                .optional()
                .map_err(Error::from)
        })
        .await
    }

    async fn mget(&self, keys: &[impl Key]) -> Result<Vec<UserStorageEntry>, Error> {
        let keys = keys.into_iter().map(|k| k.to_string()).collect::<Vec<_>>();
        self.interact(|conn| {
            user_storage::table
                .filter(user_storage::key.eq_any(keys))
                .load(conn)
                .map_err(Error::from)
        })
        .await
    }

    async fn set(&self, key: impl Key, entry: Entry) -> Result<(), Error> {
        let key = key.to_string();
        let entry = UserStorageEntry::from((key, entry));
        self.interact(|conn| {
            diesel::insert_into(user_storage::table)
                .values(entry)
                .execute(conn)
                .map_err(Error::from)
        })
        .await?;
        Ok(())
    }

    async fn mset(&self, items: &[(impl Key, Entry)]) -> Result<(), Error> {
        let entries = items
            .into_iter()
            .map(|(key, entry)| {
                let key = key.to_string();
                UserStorageEntry::from((key, entry.clone()))
            })
            .collect::<Vec<_>>();

        self.interact(|conn| {
            diesel::insert_into(user_storage::table)
                .values(entries)
                .execute(conn)
                .map_err(Error::from)
        })
        .await?;
        Ok(())
    }

    async fn mdel(&self, keys: &[impl Key]) -> Result<(), Error> {
        let keys = keys.into_iter().map(|k| k.to_string()).collect::<Vec<_>>();
        self.interact(|conn| {
            diesel::delete(user_storage::table.filter(user_storage::key.eq_any(keys)))
                .execute(conn)
                .map_err(Error::from)
        })
        .await?;
        Ok(())
    }
}
