CREATE TABLE IF NOT EXISTS user_storage (
    key TEXT NOT NULL,
    entry_type TEXT NOT NULL CHECK(entry_type IN ('binary', 'boolean', 'integer', 'json', 'string')),
    entry_value_binary TEXT,
    entry_value_boolean BOOLEAN,
    entry_value_integer BIGINT,
    entry_value_json JSONB,
    entry_value_string TEXT,

    PRIMARY KEY (key)
)