use rocket::{response::content, State};

use crate::database::DbConn;
use crate::domain::user::User;
use crate::graphql::{GQLContext, Schema};

#[rocket::post("/", data = "<request>")]
pub fn post_graphql_handler(
    database: DbConn,
    request: juniper_rocket::GraphQLRequest,
    auth: crate::lib::auth::Auth,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    // let user = User::get()
    let user = match auth.user_id {
        Some(id) => {
            let id = uuid::Uuid::parse_str(&id).unwrap();
            User::find_by_id(&database, id).unwrap()
        }
        None => None,
    };
    request.execute(&schema, &GQLContext { database, user })
}

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::playground_source("/")
}
