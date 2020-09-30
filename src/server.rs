use crate::config;
use crate::database::DbConn;
use crate::handler::*;

use rocket::config::{Config, Environment, Value};
// use rocket::fairing::AdHoc;
// use rocket::http::hyper::header;
use std::collections::HashMap;

pub struct Server {
    config: Config,
}

use rocket::http::Method; // 1.

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

fn make_cors() -> Cors {
    let port: &str = &config::port();
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://0.0.0.0:3000",
        &format!("http://localhost:{}", port),
        &format!("http://127.0.0.1:{}", port),
        &format!("http://0.0.0.0:{}", port),
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
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
            .mount("/ping", routes![ping::ping])
            .mount("/auth", routes![auth::register, auth::login])
            .mount(
                "/user",
                routes![user::get_by_id, user::get_me, user::update_me],
            )
            .mount(
                "/status",
                routes![
                    status::accepted,
                    status::declined,
                    status::get_status_by_userid,
                    status::get_all_status,
                ],
            )
        // .mount("/graphql", rocket::routes![post_graphql_handler, graphiql])
    }
}
