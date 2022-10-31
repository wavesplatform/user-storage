use crate::schema::user_storage;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde_json::Value;

#[derive(Insertable, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = user_storage)]
#[diesel(primary_key(key))]
pub struct UserStorageEntry {
    pub key: String,
    pub entry_type: String,
    pub entry_value_binary: Option<String>, //b58
    pub entry_value_boolean: Option<bool>,
    pub entry_value_integer: Option<i64>,
    pub entry_value_json: Option<Value>,
    pub entry_value_string: Option<String>,
}

pub mod dto {
    use super::Value;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct KeyEntryList {
        pub entries: Vec<KeyEntryPair>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct NullableEntryList {
        pub entries: Vec<Option<Entry>>,
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
    pub struct KeyList {
        pub keys: Vec<String>,
    }
}

impl From<UserStorageEntry> for dto::Entry {
    fn from(entry: UserStorageEntry) -> Self {
        match entry.entry_type.as_str() {
            "binary" => dto::Entry::Binary(entry.entry_value_binary.unwrap()),
            "boolean" => dto::Entry::Boolean(entry.entry_value_boolean.unwrap()),
            "integer" => dto::Entry::Integer(entry.entry_value_integer.unwrap()),
            "json" => dto::Entry::Json(entry.entry_value_json.unwrap()),
            "string" => dto::Entry::String(entry.entry_value_string.unwrap()),
            e => unreachable!("unknown entry type {e}"),
        }
    }
}

impl From<(String, dto::Entry)> for UserStorageEntry {
    fn from((key, entry): (String, dto::Entry)) -> Self {
        match entry {
            dto::Entry::Binary(val) => UserStorageEntry {
                key,
                entry_type: String::from("binary"),
                entry_value_binary: Some(val),
                entry_value_boolean: None,
                entry_value_integer: None,
                entry_value_json: None,
                entry_value_string: None,
            },
            dto::Entry::Boolean(val) => UserStorageEntry {
                key,
                entry_type: String::from("boolean"),
                entry_value_binary: None,
                entry_value_boolean: Some(val),
                entry_value_integer: None,
                entry_value_json: None,
                entry_value_string: None,
            },
            dto::Entry::Integer(val) => UserStorageEntry {
                key,
                entry_type: String::from("integer"),
                entry_value_binary: None,
                entry_value_boolean: None,
                entry_value_integer: Some(val),
                entry_value_json: None,
                entry_value_string: None,
            },
            dto::Entry::Json(val) => UserStorageEntry {
                key,
                entry_type: String::from("json"),
                entry_value_binary: None,
                entry_value_boolean: None,
                entry_value_integer: None,
                entry_value_json: Some(val),
                entry_value_string: None,
            },
            dto::Entry::String(val) => UserStorageEntry {
                key,
                entry_type: String::from("string"),
                entry_value_binary: None,
                entry_value_boolean: None,
                entry_value_integer: None,
                entry_value_json: None,
                entry_value_string: Some(val),
            },
        }
    }
}
