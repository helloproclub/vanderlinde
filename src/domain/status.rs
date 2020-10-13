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
    pub user_id: Uuid,
    pub status: i32,
    pub message: Option<String>,
    pub discord_invite: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users_status"]
pub struct CreateStatusForm<'a> {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub status: i32,
    pub message: Option<&'a str>,
    pub discord_invite: Option<&'a str>,
}

#[derive(AsChangeset)]
#[table_name = "users_status"]
pub struct UpdateStatusForm<'a> {
    pub status: i32,
    pub message: Option<&'a str>,
    pub discord_invite: Option<&'a str>,
}

impl Status {
    pub fn new<'a>(db: DbConn, user_id: uuid::Uuid, status: i32) -> Result<Status, DBError> {
        let _message = "";
        let _discord_invite = "";
        diesel::insert_into(users_status::table)
            .values(&CreateStatusForm {
                id: uuid::Uuid::new_v4(),
                user_id,
                status,
                message: None,
                discord_invite: None,
            })
            .get_result::<Status>(&*db)
    }

    pub fn find_by_userid(db: &DbConn, _user_id: Uuid) -> Result<Option<Status>, DBError> {
        use crate::database::schema::users_status::dsl::*;
        users_status
            .filter(user_id.eq(_user_id))
            .first::<Status>(&**db)
            .optional()
    }

    pub fn find_all_by_status(
        db: &DbConn,
        _status: Option<i32>,
        _limit: i32,
        _offset: i32,
    ) -> Result<Option<Vec<Status>>, DBError> {
        use crate::database::schema::users_status::dsl::*;
        match _status {
            None => users_status.load::<Status>(&**db).optional(),
            Some(s) => users_status
                .filter(status.eq(s))
                .limit(_limit as i64)
                .offset(_offset as i64)
                .load::<Status>(&**db)
                .optional(),
        }
    }

    pub fn count_by_status(db: &DbConn, _status: Option<i32>) -> Result<i64, DBError> {
        use crate::database::schema::users_status::dsl::*;
        use diesel::dsl::count;

        match _status {
            None => users_status.select(count(id)).first(&**db),
            Some(s) => users_status
                .select(count(id))
                .filter(status.eq(s))
                .first(&**db),
        }
    }

    pub fn update_by_user_id(
        db: &DbConn,
        _user_id: Uuid,
        _status: i32,
        _message: Option<&str>,
        _discord_invite: Option<&str>,
    ) -> Result<Status, DBError> {
        use crate::database::schema::users_status::dsl::*;
        let query = &UpdateStatusForm {
            status: _status,
            message: _message,
            discord_invite: _discord_invite,
        };
        diesel::update(users_status.filter(user_id.eq(_user_id)))
            .set(query)
            .get_result::<Status>(&**db)
    }
}
