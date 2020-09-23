#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_json;

mod config;
mod database;
mod domain;
// mod graphql;
mod handler;
mod lib;
mod server;

fn main() {
    config::init();

    use server::Server;
    let err = Server::new().init().launch();
    println!("{}", err);
}
