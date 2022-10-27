// @generated automatically by Diesel CLI.

diesel::table! {
    user_storage (key) {
        key -> Text,
        entry_type -> Text,
        entry_value_binary -> Nullable<Text>,
        entry_value_boolean -> Nullable<Bool>,
        entry_value_integer -> Nullable<Int8>,
        entry_value_json -> Nullable<Jsonb>,
        entry_value_string -> Nullable<Text>,
    }
}
