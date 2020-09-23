use crate::config;
use crate::database::DbConn;
// use crate::graphql::{Mutations, Query, Schema};
use crate::handler::auth::*;
// use crate::handler::graphql::*;
use crate::handler::ping::*;

use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use rocket::http::hyper::header;
use std::collections::HashMap;

pub struct Server {
    config: Config,
}

use rocket::http::Method; // 1.

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

fn make_cors() -> Cors {
    let port: &str = &config::port();
    let allowed_origins = AllowedOrigins::some_exact(&[
        // "https://talos-blinfoldking.herokuapp.com",
        // "https://blinfoldking.dev",
        // "https://www.blinfoldking.dev",
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://0.0.0.0:3000",
        &format!("http://localhost:{}", port),
        &format!("http://127.0.0.1:{}", port),
        &format!("http://0.0.0.0:{}", port),
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

impl Server {
    pub fn new() -> Self {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(config::database_url()));
        databases.insert("postgres_db", Value::from(database_config));

        let config = Config::build(Environment::Staging)
            .port(config::port().parse().unwrap())
            .extra("databases", databases)
            .finalize()
            .unwrap();

        Server { config }
    }

    pub fn init(self) -> rocket::Rocket {
        rocket::custom(self.config)
            .attach(DbConn::fairing())
            .attach(make_cors())
            // .manage(Schema::new(Query, Mutations))
            .mount("/ping", routes![ping])
            .mount("/auth", routes![register, login])
            // .mount("/graphql", rocket::routes![post_graphql_handler, graphiql])
    }
}
