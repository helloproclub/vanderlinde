extern crate diesel;

use crate::database::schema::users_status;
use crate::database::DbConn;
use diesel::prelude::*;
use diesel::result::Error as DBError;
use diesel::RunQueryDsl;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Status {
    pub id: Uuid,
    pub status: String,
    pub message: String,
    pub discord_invite: String,
}

#[derive(Insertable)]
#[table_name = "users_status"]
pub struct CreateStatusForm<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub message: &'a str,
    pub discord_invite: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "users_status"]
pub struct UpdateStatusForm<'a> {
    pub status: &'a str,
    pub message: &'a str,
    pub discord_invite: &'a str,
}

impl Status {
    pub fn new<'a>(
        db: DbConn,
        id: uuid::Uuid,
        status: &'a str,
    ) -> Result<Status, DBError> {
        let message = "";
        let discord_invite = "";
        diesel::insert_into(users_status::table)
            .values(&CreateStatusForm {
                id,
                status,
                message,
                discord_invite,
            })
            .get_result::<Status>(&*db)
    }

    pub fn find_by_id(db: &DbConn, _id: Uuid) -> Result<Option<Status>, DBError> {
        use crate::database::schema::users_status::dsl::*;
        users_status.find(_id).first::<Status>(&**db).optional()
    }

    pub fn update_by_id(
        db: &DbConn,
        _id: String,
        status_update: String,
        message_update: String,
        discord_invite_update: String,
    ) -> Result<Status, DBError> {
        use crate::database::schema::users_status::dsl::*;
        let query = &UpdateStatusForm{
            status: &status_update,
            message: &message_update,
            discord_invite: &discord_invite_update,
        };
        diesel::update(users_status.find(uuid::Uuid::parse_str(&_id).unwrap()))
            .set(query)
            .get_result::<Status>(&**db)
    }
}