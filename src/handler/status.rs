extern crate bcrypt;

use super::response::APIResponse;
use crate::database::DbConn;
use crate::domain::status::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct StatusResponse {
    id: String,
    status: String,
    message: String,
    discord_invite: String,
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    message: String,
    discord_invite: String,
    secret: String,
}

#[post("/accepted/<id>", data = "<form>")]
pub fn accepted(
    db: DbConn,
    id: String,
    form: Json<UpdateRequest>,
) -> Result<APIResponse, APIResponse> {
    if form.secret != crate::config::secret() {
        return Err(APIResponse::error().unauthorized().message("wrong secret"));
    };

    let result = Status::update_by_id(
        &db,
        id.to_string(),
        "Accepted".to_string(),
        form.message.to_string(),
        form.discord_invite.to_string(),
    );

    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(datas) => Ok(APIResponse::ok()),
    }
}

#[post("/declined/<id>", data = "<form>")]
pub fn declined(
    db: DbConn,
    id: String,
    form: Json<UpdateRequest>,
) -> Result<APIResponse, APIResponse> {
    if form.secret != crate::config::secret() {
        return Err(APIResponse::error().unauthorized().message("wrong secret"));
    };

    let result = Status::update_by_id(
        &db,
        id.to_string(),
        "Declined".to_string(),
        form.message.to_string(),
        form.discord_invite.to_string(),
    );

    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(datas) => Ok(APIResponse::ok()),
    }
}
