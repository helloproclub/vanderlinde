use crate::domain::user::User;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct Auth {
    pub token: Option<String>,
    pub user_id: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = crate::handler::response::APIResponse;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let res = match request.headers().get_one("Authorization") {
            Some(token) => {
                let user_id = User::decode_token(token.to_owned());
                Auth {
                    token: Some(token.to_owned()),
                    user_id: user_id,
                }
            }
            None => Auth {
                token: None,
                user_id: None,
            },
        };

        Outcome::Success(res)
    }
}
