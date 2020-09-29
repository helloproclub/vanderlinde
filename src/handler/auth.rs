extern crate bcrypt;

use super::response::APIResponse;
use crate::database::DbConn;
use crate::domain::user::*;
use crate::domain::status::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub nim: String,
    pub password: String,
    pub ktm_url: String,
    pub cv_url: String,
    pub letter_url: String,
    pub linkedin_url: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    id: String,
    name: String,
    email: String,
    token: String,
}

#[post("/register", data = "<form>")]
pub fn register(db_user: DbConn, db_status: DbConn, form: Json<RegisterRequest>) -> Result<APIResponse, APIResponse> {
    let result = User::new(
        db_user,
        UserForm {
            email: &*form.email,
            name: &*form.name,
            nim: &*form.nim,
            password: &*form.password,
            ktm_url: &*form.ktm_url,
            cv_url: &*form.cv_url,
            letter_url: &*form.letter_url,
            linkedin_url: &*form.linkedin_url,
        },
    );

    let new_user = result?;
    let id = format!("{}", new_user.id.to_hyphenated());
    let token = new_user.generate_token();
    
    Status::new(db_status, new_user.id, "Waiting for review")?;
 
    Ok(APIResponse::ok().data(json!(&AuthResponse {
        id,
        name: new_user.name,
        email: new_user.email,
        token,
    })))
}

#[post("/login", data = "<form>")]
pub fn login(db: DbConn, form: Json<LoginRequest>) -> Result<APIResponse, APIResponse> {
    let result = User::get(db, &form.email)?;
    match result {
        None => Err(APIResponse::error().not_found()),
        Some(user) => {
            if user.verify_password(&form.password) {
                let id = format!("{}", user.id.to_hyphenated());
                let token = user.generate_token();

                Ok(APIResponse::ok().data(json!(&AuthResponse {
                    id,
                    name: user.name,
                    email: user.email,
                    token,
                })))
            } else {
                Err(APIResponse::error().unauthorized())
            }
        }
    }
}
