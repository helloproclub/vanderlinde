extern crate bcrypt;

use super::response::APIResponse;
use crate::database::DbConn;
use crate::domain::status::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

#[get("/accepted/<id>")]
pub fn accepted(db: DbConn, id: Uuid) -> Result<APIResponse, APIResponse> {
    let message = format!("{} Accepted", id);
    Ok(APIResponse::ok().message(&message))
}

#[get("/declined/<id>")]
pub fn declined(db: DbConn, id: Uuid) -> Result<APIResponse, APIResponse> {
    let message = format!("{} Declined", id);
    Ok(APIResponse::ok().message(&message))
}