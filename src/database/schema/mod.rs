table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Text,
    }
}

table! {
    users_status (id) {
        id -> Uuid,
        status -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    users_status,
);
