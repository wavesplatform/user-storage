CREATE TABLE IF NOT EXISTS user_storage (
    key TEXT NOT NULL,
    user_addr TEXT NOT NULL,
    entry_type TEXT NOT NULL CHECK(entry_type IN ('binary', 'boolean', 'integer', 'json', 'string')),
    entry_value_binary TEXT,
    entry_value_boolean BOOLEAN,
    entry_value_integer BIGINT,
    entry_value_json JSONB,
    entry_value_string TEXT,

    PRIMARY KEY (key, user_addr)
);

CREATE UNIQUE INDEX IF NOT EXISTS user_storage_key_user_addr_idx ON user_storage (key, user_addr);
