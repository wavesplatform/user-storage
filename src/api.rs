use crate::error::Error;
use crate::models::dto::{EntriesList, Entry, KeysRequest};
use crate::repo::Repo;
use serde::Serialize;
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, reply, with_status, Json, Reply},
    Filter, Rejection,
};
use wavesexchange_log::{error, info};
use wavesexchange_warp::error::{
    error_handler_with_serde_qs, handler, internal, not_found, validation,
};
use wavesexchange_warp::log::access;
use wavesexchange_warp::MetricsWarpBuilder;

const ERROR_CODES_PREFIX: u16 = 95;

pub async fn start(port: u16, metrics_port: u16, user_storage: impl Repo + Send + Sync + 'static) {
    let error_handler = handler(ERROR_CODES_PREFIX, |err| match err {
        Error::ValidationError(field, error_details) => {
            let mut error_details = error_details.to_owned();
            if let Some(details) = error_details.as_mut() {
                details.insert("parameter".to_owned(), field.to_owned());
            }
            validation::invalid_parameter(ERROR_CODES_PREFIX, error_details)
        }
        Error::KeyNotFound(_) => not_found(ERROR_CODES_PREFIX),

        _ => internal(ERROR_CODES_PREFIX),
    });

    let qs_config = create_serde_qs_config();

    let with_user_storage = {
        let storage = Arc::new(user_storage);
        warp::any().map(move || storage.clone())
    };

    let get_entries = warp::get()
        .and(warp::path::end())
        .and(serde_qs::warp::query::<KeysRequest>(qs_config))
        .and(with_user_storage.clone())
        .and_then(controllers::get_entries)
        .map(to_json);

    let get_entries_post = warp::post()
        .and(warp::path::end())
        .and(warp::body::json::<KeysRequest>())
        .and(with_user_storage.clone())
        .and_then(controllers::get_entries)
        .map(to_json);

    let set_entries = warp::put()
        .and(warp::path::end())
        .and(warp::body::json::<EntriesList>())
        .and(with_user_storage.clone())
        .and_then(controllers::set_entries)
        .map(to_json);

    let delete_entries = warp::delete()
        .and(warp::path::end())
        .and(warp::body::json::<KeysRequest>())
        .and(with_user_storage.clone())
        .and_then(controllers::delete_entries)
        .map(to_json);

    let get_single_entry = warp::path!("key" / String)
        .and(warp::get())
        .and(with_user_storage.clone())
        .and_then(controllers::get_single_entry)
        .map(to_json);

    let set_single_entry = warp::path!("key" / String)
        .and(warp::put())
        .and(warp::body::json::<Entry>())
        .and(with_user_storage.clone())
        .and_then(controllers::set_single_entry)
        .map(|result: Option<Entry>| match result {
            Some(old) => to_json(old).into_response(),
            None => with_status(reply(), StatusCode::CREATED).into_response(),
        });

    let delete_single_entry = warp::path!("key" / String)
        .and(warp::delete())
        .and(with_user_storage.clone())
        .and_then(controllers::delete_single_entry)
        .map(to_json);

    let log = warp::log::custom(access);

    info!("Starting API server at 0.0.0.0:{}", port);

    let routes = get_entries
        .or(get_entries_post)
        .or(set_entries)
        .or(delete_entries)
        .or(get_single_entry)
        .or(set_single_entry)
        .or(delete_single_entry)
        .recover(move |rej| {
            error!("{:?}", rej);
            error_handler_with_serde_qs(ERROR_CODES_PREFIX, error_handler.clone())(rej)
        })
        .with(log);

    MetricsWarpBuilder::new()
        .with_main_routes(routes)
        .with_main_routes_port(port)
        .with_metrics_port(metrics_port)
        .run_blocking()
        .await;
}

mod controllers {
    use super::*;
    use crate::models::dto::KeyEntryPair;
    use std::collections::HashMap;

    pub(super) async fn get_entries<R: Repo>(
        keys: KeysRequest,
        repo: Arc<R>,
    ) -> Result<EntriesList, Rejection> {
        let keys = keys.keys;
        let mut entries: HashMap<String, Entry> = {
            let raw_entries = repo.mget(&keys).await?;
            HashMap::from_iter(
                raw_entries
                    .into_iter()
                    .map(|e| (e.key.clone(), Entry::from(e))),
            )
        };

        Ok(EntriesList {
            entries: keys
                .into_iter()
                .map(|key| match entries.remove(&key) {
                    Some(entry) => Some(KeyEntryPair {
                        key,
                        entry: Some(entry),
                    }),
                    None => None,
                })
                .collect(),
        })
    }

    pub(super) async fn set_entries<R: Repo>(
        entries: EntriesList,
        repo: Arc<R>,
    ) -> Result<EntriesList, Rejection> {
        let key_entry_pairs: Vec<(String, Option<Entry>)> = entries
            .entries
            .into_iter()
            .filter_map(|entry| entry.map(|pair| (pair.key, pair.entry)))
            .collect();

        let keys = key_entry_pairs
            .iter()
            .map(|pair| pair.0.clone())
            .collect::<Vec<_>>();

        let old_entries = get_entries(KeysRequest { keys }, repo.clone()).await?;

        let keys_to_delete = key_entry_pairs
            .iter()
            .filter_map(|pair| match pair.1 {
                Some(_) => None,
                None => Some(&pair.0),
            })
            .collect::<Vec<_>>();

        if !keys_to_delete.is_empty() {
            repo.mdel(&keys_to_delete).await?;
        }

        let pairs_to_update = key_entry_pairs
            .into_iter()
            .filter_map(|pair| pair.1.map(|entry| (pair.0, entry)))
            .collect::<Vec<_>>();

        if !pairs_to_update.is_empty() {
            repo.mset(&pairs_to_update).await?;
        }

        Ok(old_entries)
    }

    pub(super) async fn delete_entries<R: Repo>(
        keys: KeysRequest,
        repo: Arc<R>,
    ) -> Result<EntriesList, Rejection> {
        let old_entries = get_entries(keys.clone(), repo.clone()).await?;

        repo.mdel(&keys.keys).await?;

        Ok(old_entries)
    }

    pub(super) async fn get_single_entry<R: Repo>(
        key: String,
        repo: Arc<R>,
    ) -> Result<Entry, Rejection> {
        repo.get(&key)
            .await?
            .ok_or(Error::KeyNotFound(key).into())
            .map(Entry::from)
    }

    pub(super) async fn set_single_entry<R: Repo>(
        key: String,
        entry: Entry,
        repo: Arc<R>,
    ) -> Result<Option<Entry>, Rejection> {
        let old_entry = repo.get(&key).await?.map(Entry::from);

        repo.set(key, entry).await?;

        Ok(old_entry)
    }

    pub(super) async fn delete_single_entry<R: Repo>(
        key: String,
        repo: Arc<R>,
    ) -> Result<Entry, Rejection> {
        let old_entry = get_single_entry(key.clone(), repo.clone()).await?;

        repo.mdel(&[key]).await?;

        Ok(old_entry)
    }
}

fn to_json<T: Serialize>(data: T) -> Json {
    json(&data)
}

fn create_serde_qs_config() -> serde_qs::Config {
    serde_qs::Config::new(5, false)
}