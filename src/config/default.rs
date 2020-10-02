pub fn port() -> String {
    "8000".to_owned()
}

pub fn database_url() -> String {
    "postgres://postgres:secret@localhost/vanderlinde_db".to_owned()
}

pub fn secret() -> String {
    "secret".to_owned()
}

pub fn frontend_whitelist() -> String {
    "https://teknofest.proclub.tech".to_owned()
}
