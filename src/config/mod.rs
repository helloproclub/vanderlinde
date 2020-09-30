extern crate dotenv;

use std::env;

mod default;

pub fn init() {
    match dotenv::dotenv() {
        Err(_) => println!("no .env file found"),
        Ok(_) => println!(".env found"),
    }
}

pub fn port() -> String {
    env::var("PORT").unwrap_or(default::port())
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or(default::database_url())
}

pub fn secret() -> String {
    env::var("SECRET").unwrap_or(default::secret())
}
