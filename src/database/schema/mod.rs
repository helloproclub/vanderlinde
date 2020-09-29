table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        nim -> Varchar,
        name -> Varchar,
        password_hash -> Text,
        ktm_url -> Text,
        cv_url -> Text,
        letter_url -> Text,
        linkedin_url -> Text,
    }
}

table! {
    users_status (id) {
        id -> Uuid,
        status -> Text,
        message -> Text,
        discord_invite -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    users_status,
);
