extern crate diesel;

use crate::config;
use crate::database::schema::users;
use crate::database::DbConn;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::result::Error as DBError;
use diesel::RunQueryDsl;
use frank_jwt::{decode, encode, Algorithm, ValidationOptions};
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub password_hash: &'a str,
}

impl User {
    pub fn new<'a>(
        db: DbConn,
        email: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Result<User, DBError> {
        let password_hash = &hash(password, DEFAULT_COST).expect("failed to encrypt password");
        diesel::insert_into(users::table)
            .values(&NewUser {
                id: uuid::Uuid::new_v4(),
                username,
                email,
                password_hash,
            })
            .get_result::<User>(&*db)
    }

    pub fn get<'a>(db: DbConn, _email: &'a str) -> Result<Option<User>, DBError> {
        use crate::database::schema::users::dsl::*;
        users
            .filter(email.eq(_email))
            .first::<User>(&*db)
            .optional()
    }

    pub fn find_by_id(db: &DbConn, _id: Uuid) -> Result<Option<User>, DBError> {
        use crate::database::schema::users::dsl::*;
        users.find(_id).first::<User>(&**db).optional()
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap()
    }

    pub fn generate_token(&self) -> String {
        let payload = json!({
            "id": format!("{}", self.id),
        });

        let header = json!({});
        let secret = config::secret();
        let jwt = encode(header, &secret, &payload, Algorithm::HS256);
        jwt.expect("token error")
    }

    pub fn decode_token(token: String) -> Option<String> {
        let (_, payload) = decode(
            &token,
            &config::secret(),
            Algorithm::HS256,
            &ValidationOptions::dangerous(),
        )
        .unwrap();

        match payload["id"].as_str() {
            Some(id) => Some(id.to_owned()),
            None => None,
        }
    }
}
