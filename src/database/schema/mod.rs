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
