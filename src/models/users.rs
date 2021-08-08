use crate::schema::users;
use crate::db;
use crate::error::ServerError;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
#[table_name = "users"]
pub struct Users {
    pub user_id: i32,
    pub email: String,
    pub password: String
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String
}

impl Users {
    pub fn find(id: i32) -> Result<Self, ServerError> {
        let conn = db::connection()?;
        let user = users::table.load::<Users>(&conn)?;
        Ok(user)
    }
    pub fn create(user: NewUser) -> Result<Self, ServerError> {
        let conn = db::connection()?;
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }
}

