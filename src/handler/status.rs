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
    message: Option<String>,
    discord_invite: Option<String>,
}

#[derive(Serialize)]
pub struct StatusResponseList {
    count: i32,
    items: Vec<StatusResponse>,
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

    let _id: &str = &*userid;
    let uuidparse = uuid::Uuid::parse_str(_id);
    if let Err(_) = uuidparse {
        return Err(APIResponse::error().bad_request().message("Invalid id"));
    }

    let id = uuidparse.unwrap();

    let result = Status::update_by_user_id(&db, id, 1, None, Some(&*form.discord_invite));

    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(data) => Ok(APIResponse::ok().data(json!(&StatusResponse {
            id: data.id.to_string(),
            user_id: data.user_id.to_string(),
            status: data.status,
            message: data.message,
            discord_invite: data.discord_invite
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

    let _id: &str = &*userid;
    let uuidparse = uuid::Uuid::parse_str(_id);
    if let Err(_) = uuidparse {
        return Err(APIResponse::error().bad_request().message("Invalid id"));
    }

    let id = uuidparse.unwrap();

    let result = Status::update_by_user_id(&db, id, 2, Some(&*form.message), None);

    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(data) => Ok(APIResponse::ok().data(json!(&StatusResponse {
            id: data.id.to_string(),
            user_id: data.user_id.to_string(),
            status: data.status,
            message: data.message,
            discord_invite: data.discord_invite
        }))),
    }
}

#[get("/?<status>&<limit>&<offset>")]
pub fn get_all_status(
    db: DbConn,
    status: Option<i32>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<APIResponse, APIResponse> {
    let _limit = limit.unwrap_or(10);
    let _offset = offset.unwrap_or(0);
    let result = Status::find_all_by_status(&db, status, _limit, _offset);
    let count = Status::count_by_status(&db, status);
    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(data) => match data {
            Some(statuses) => Ok(APIResponse::ok().data(json!(&StatusResponseList {
                count: count.unwrap() as i32,
                items: statuses
                    .into_iter()
                    .map(|status| StatusResponse {
                        id: status.id.to_string(),
                        status: status.status,
                        user_id: status.user_id.to_string(),
                        message: status.message,
                        discord_invite: status.discord_invite,
                    })
                    .collect()
            }))),
            None => Err(APIResponse::error().not_found()),
        },
    }
}

#[get("/<userid>")]
pub fn get_status_by_userid(db: DbConn, userid: String) -> Result<APIResponse, APIResponse> {
    match uuid::Uuid::parse_str(&*userid) {
        Ok(id) => {
            let result = Status::find_by_userid(&db, id);
            match result {
                Err(_) => Err(APIResponse::error().bad_request()),
                Ok(data) => match data {
                    Some(status) => Ok(APIResponse::ok().data(json!(&StatusResponse {
                        id: status.id.to_string(),
                        user_id: status.user_id.to_string(),
                        status: status.status,
                        message: status.message,
                        discord_invite: status.discord_invite
                    }))),
                    None => Err(APIResponse::error().not_found()),
                },
            }
        }
        Err(_) => Err(APIResponse::error().not_found()),
    }
}
