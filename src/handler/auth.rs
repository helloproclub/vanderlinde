extern crate bcrypt;

use super::response::APIResponse;
use crate::database::DbConn;
use crate::domain::user::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    id: String,
    username: String,
    email: String,
    token: String,
}

#[post("/register", data = "<form>")]
pub fn register(db: DbConn, form: Json<RegisterRequest>) -> Result<APIResponse, APIResponse> {
    let result = User::new(db, &form.email, &form.username, &form.password);
    let new_user = result?;
    let id = format!("{}", new_user.id.to_hyphenated());
    let token = new_user.generate_token();
    Ok(APIResponse::ok().data(json!(&AuthResponse {
        id,
        username: new_user.username,
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
                    username: user.username,
                    email: user.email,
                    token,
                })))
            } else {
                Err(APIResponse::error().unauthorized())
            }
        }
    }
}
