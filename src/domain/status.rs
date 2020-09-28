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
}

#[derive(Insertable)]
#[table_name = "users_status"]
pub struct CreateStatusForm<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "users_status"]
pub struct UpdateStatusForm<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
}

impl Status {
    pub fn new<'a>(
        db: DbConn,
        status: &'a str,
    ) -> Result<Status, DBError> {
        diesel::insert_into(users_status::table)
            .values(&CreateStatusForm {
                id: uuid::Uuid::new_v4(),
                status,
            })
            .get_result::<Status>(&*db)
    }

    pub fn find_by_id(db: &DbConn, _id: Uuid) -> Result<Option<Status>, DBError> {
        use crate::database::schema::users_status::dsl::*;
        users_status.find(_id).first::<Status>(&**db).optional()
    }

    pub fn update_by_id(db: &DbConn, _id: Uuid, update: UpdateStatusForm) -> Result<Status, DBError> {
        use crate::database::schema::users_status::dsl::*;
        let now = chrono::Local::now().naive_local();
        diesel::update(users_status.find(_id))
            .set((&update))
            .get_result::<Status>(&**db)
    }
}