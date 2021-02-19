use std::ptr::null;
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use crate::data;
use crate::data::models;
use crate::data::models::{CreatePassword, user_password};
use crate::data::schema;

pub fn insert_new_password(
    conn: &SqliteConnection,
    user_password: CreatePassword,
) -> Result<models::user_password, String> {
    use crate::data::schema::password::dsl::*;

    let insert_res:QueryResult<usize> = diesel::insert_into(password).values(&user_password).execute(conn);
    if !insert_res.is_ok() {
       return  Err(insert_res.err().unwrap().to_string());
    }

    let passwordres = password.order(id.desc()).first(conn) as Result<models::user_password, diesel::result::Error>;
    if passwordres.is_ok() {
        Ok(passwordres.unwrap())
    } else {
        Err(passwordres.err().unwrap().to_string())
    }
}