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
    pub nim: String,
    pub name: String,
    pub password_hash: String,
    pub ktm_url: String,
    pub cv_url: String,
    pub letter_url: String,
    pub linkedin_url: String,
    pub division: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub nim: &'a str,
    pub name: &'a str,
    pub password_hash: &'a str,
    pub ktm_url: &'a str,
    pub cv_url: &'a str,
    pub letter_url: &'a str,
    pub linkedin_url: &'a str,
    pub division: i32,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub email: &'a str,
    pub nim: &'a str,
    pub name: &'a str,
    pub ktm_url: &'a str,
    pub cv_url: &'a str,
    pub letter_url: &'a str,
    pub linkedin_url: &'a str,
    pub division: i32,
}

pub struct UserForm<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub nim: &'a str,
    pub password: &'a str,
    pub ktm_url: &'a str,
    pub cv_url: &'a str,
    pub letter_url: &'a str,
    pub linkedin_url: &'a str,
    pub division: i32,
}

impl User {
    pub fn new<'a>(db: DbConn, user: UserForm) -> Result<User, DBError> {
        let password_hash = &hash(user.password, DEFAULT_COST).expect("failed to encrypt password");
        diesel::insert_into(users::table)
            .values(&NewUser {
                id: uuid::Uuid::new_v4(),
                name: user.name,
                email: user.email,
                nim: user.nim,
                ktm_url: user.ktm_url,
                cv_url: user.cv_url,
                letter_url: user.letter_url,
                linkedin_url: user.linkedin_url,
                division: user.division,
                password_hash,
            })
            .get_result::<User>(&*db)
    }

    pub fn update_by_id(db: &DbConn, _id: Uuid, update: UpdateUser) -> Result<User, DBError> {
        use crate::database::schema::users::dsl::*;
        diesel::update(users.find(_id))
            .set(&update)
            .get_result::<User>(&**db)
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
