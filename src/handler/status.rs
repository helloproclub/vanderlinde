extern crate bcrypt;

use super::response::APIResponse;
use crate::database::DbConn;
use crate::domain::status::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct UpdateRequest {
    secret: String,
}

#[post("/accepted/<id>", data = "<form>")]
pub fn accepted(db: DbConn, id: String, form: Json<UpdateRequest>) -> Result<APIResponse, APIResponse> {
    if form.secret != crate::config::secret() {
        return Err(APIResponse::error().unauthorized().message("wrong secret"));
    };
    
    Status::update_by_id(&db, id.to_string(), "Accepted".to_string())?;
 
    let message = format!("{} Accepted", id);
    Ok(APIResponse::ok().message(&message))
}

#[post("/declined/<id>", data = "<form>")]
pub fn declined(db: DbConn, id: String, form: Json<UpdateRequest>) -> Result<APIResponse, APIResponse> {
    if form.secret != crate::config::secret() {
        return Err(APIResponse::error().unauthorized().message("wrong secret"));
    };
    
    Status::update_by_id(&db, id.to_string(), "Declined".to_string())?;
 
    let message = format!("{} Declined", id);
    Ok(APIResponse::ok().message(&message))
}