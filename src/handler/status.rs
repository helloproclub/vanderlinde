extern crate bcrypt;

use super::response::APIResponse;
use crate::database::DbConn;
use crate::domain::status::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct StatusResponse {
    id: String,
    user_id: String,
    status: i32,
    message: String,
    discord_invite: String,
}

#[derive(Deserialize)]
pub struct DeclineRequest {
    message: String,
    secret: String,
}

#[derive(Deserialize)]
pub struct AcceptRequest {
    discord_invite: String,
    secret: String,
}

#[post("/<userid>/accept", data = "<form>")]
pub fn accepted(
    db: DbConn,
    userid: String,
    form: Json<AcceptRequest>,
) -> Result<APIResponse, APIResponse> {
    if form.secret != crate::config::secret() {
        return Err(APIResponse::error().unauthorized().message("wrong secret"));
    };

    let result = Status::update_by_user_id(&db, userid, 1, None, Some(&*form.discord_invite));

    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(data) => Ok(APIResponse::ok().data(json!(&StatusResponse {
            id: data.id.to_string(),
            user_id: data.user_id.to_string(),
            status: data.status,
            message: data.message.unwrap(),
            discord_invite: data.discord_invite.unwrap()
        }))),
    }
}

#[post("/<userid>/decline", data = "<form>")]
pub fn declined(
    db: DbConn,
    userid: String,
    form: Json<DeclineRequest>,
) -> Result<APIResponse, APIResponse> {
    if form.secret != crate::config::secret() {
        return Err(APIResponse::error().unauthorized().message("wrong secret"));
    };

    let result = Status::update_by_user_id(&db, userid.to_string(), 2, Some(&*form.message), None);

    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(data) => Ok(APIResponse::ok().data(json!(&StatusResponse {
            id: data.id.to_string(),
            user_id: data.user_id.to_string(),
            status: data.status,
            message: data.message.unwrap(),
            discord_invite: data.discord_invite.unwrap()
        }))),
    }
}
