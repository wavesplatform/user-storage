use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntriesList {
    pub entries: Vec<Option<KeyEntryPair>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyEntryPair {
    pub key: String,
    pub entry: Option<Entry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum Entry {
    Binary(String), // base58,
    Boolean(bool),
    Integer(i64),
    Json(Value),
    String(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct KeysRequest {
    pub keys: Vec<String>,
}
