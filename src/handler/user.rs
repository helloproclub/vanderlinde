use super::response::APIResponse;
use crate::database::DbConn;
use crate::domain::user::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    email: String,
    nim: String,
    name: String,
    ktm_url: String,
    cv_url: String,
    letter_url: String,
    linkedin_url: String,
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub email: String,
    pub name: String,
    pub nim: String,
    pub ktm_url: String,
    pub cv_url: String,
    pub letter_url: String,
    pub linkedin_url: String,
}

#[get("/<userid>")]
pub fn get_by_id(userid: String, db: DbConn) -> Result<APIResponse, APIResponse> {
    let id: &str = &*userid;
    let result = User::find_by_id(&db, uuid::Uuid::parse_str(id).unwrap());
    match result {
        Err(_) => Err(APIResponse::error().bad_request()),
        Ok(data) => match data {
            None => Err(APIResponse::error().not_found()),
            Some(user) => Ok(APIResponse::ok().data(json!(UserResponse {
                id: user.id.to_string(),
                email: user.email,
                nim: user.nim,
                name: user.name,
                ktm_url: user.ktm_url,
                cv_url: user.cv_url,
                letter_url: user.letter_url,
                linkedin_url: user.linkedin_url,
            }))),
        },
    }
}

#[get("/me")]
pub fn get_me(db: DbConn, auth: crate::lib::auth::Auth) -> Result<APIResponse, APIResponse> {
    match auth.user_id {
        Some(userid) => {
            let id = &*userid;
            let result = User::find_by_id(&db, uuid::Uuid::parse_str(id).unwrap());
            match result {
                Err(_) => Err(APIResponse::error().bad_request()),
                Ok(data) => match data {
                    None => Err(APIResponse::error().not_found()),
                    Some(user) => Ok(APIResponse::ok().data(json!(UserResponse {
                        id: user.id.to_string(),
                        email: user.email,
                        nim: user.nim,
                        name: user.name,
                        ktm_url: user.ktm_url,
                        cv_url: user.cv_url,
                        letter_url: user.letter_url,
                        linkedin_url: user.linkedin_url,
                    }))),
                },
            }
        }
        None => Err(APIResponse::error().unauthorized()),
    }
}

#[put("/me", data = "<form>")]
pub fn update_me(
    form: Json<UpdateRequest>,
    db: DbConn,
    auth: crate::lib::auth::Auth,
) -> Result<APIResponse, APIResponse> {
    match auth.user_id {
        Some(userid) => {
            let id = &*userid;
            let result = User::update_by_id(
                &db,
                uuid::Uuid::parse_str(id).unwrap(),
                UpdateUser {
                    name: &*form.name,
                    email: &*form.email,
                    nim: &*form.nim,
                    ktm_url: &*form.ktm_url,
                    cv_url: &*form.cv_url,
                    letter_url: &*form.letter_url,
                    linkedin_url: &*form.linkedin_url,
                },
            );
            match result {
                Err(_) => Err(APIResponse::error().bad_request()),
                Ok(user) => Ok(APIResponse::ok().data(json!(UserResponse {
                    id: user.id.to_string(),
                    email: user.email,
                    nim: user.nim,
                    name: user.name,
                    ktm_url: user.ktm_url,
                    cv_url: user.cv_url,
                    letter_url: user.letter_url,
                    linkedin_url: user.linkedin_url,
                }))),
            }
        }
        None => Err(APIResponse::error().unauthorized()),
    }
}
