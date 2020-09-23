table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Text,
    }
}
