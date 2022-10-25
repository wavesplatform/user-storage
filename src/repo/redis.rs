use super::Repo;
use crate::api::dtos::Entry;
use crate::async_redis::RedisPool;
use crate::error::Error;
use bb8_redis::{bb8::PooledConnection, RedisConnectionManager};
use redis::AsyncCommands;
use serde_json::{from_str, to_string};

pub struct RedisRepo {
    redis_pool: RedisPool,
}

pub fn new(redis_pool: RedisPool) -> RedisRepo {
    RedisRepo { redis_pool }
}

impl RedisRepo {
    pub async fn conn(&self) -> Result<PooledConnection<RedisConnectionManager>, Error> {
        self.redis_pool
            .get()
            .await
            .map_err(|e| Error::Bb8Error(e.to_string()))
    }
}

#[async_trait]
impl Repo for RedisRepo {
    async fn get(&self, key: &str) -> Result<Option<Entry>, Error> {
        let entry: Option<String> = self.conn().await?.get(key).await?;

        match entry {
            Some(e) => Ok(from_str(&e).map_err(Error::from)?),
            None => Ok(None),
        }
    }

    async fn mget(&self, keys: &[&str]) -> Result<Vec<Option<Entry>>, Error> {
        match keys.len() {
            0 => Ok(vec![]),
            1 => self.get(&keys[0]).await.map(|result| vec![result]),
            _ => {
                let entries: Vec<Option<String>> = self.conn().await?.get(keys).await?;

                entries
                    .into_iter()
                    .map(|entry| match entry {
                        Some(e) => Some(from_str(&e).map_err(Error::from)).transpose(),
                        None => Ok(None),
                    })
                    .collect()
            }
        }
    }

    async fn set(&self, key: &str, entry: &Entry) -> Result<(), Error> {
        let entry = to_string(entry)?;

        self.conn().await?.set(key, entry).await?;

        Ok(())
    }

    async fn mset(&self, items: &[(&str, &Entry)]) -> Result<(), Error> {
        let mut prep_items = vec![];

        for (key, entry) in items {
            prep_items.push((key, to_string(entry)?));
        }

        self.conn().await?.set_multiple(&prep_items).await?;

        Ok(())
    }

    async fn del(&self, key: &[&str]) -> Result<(), Error> {
        self.conn().await?.del(key).await?;
        Ok(())
    }
}
