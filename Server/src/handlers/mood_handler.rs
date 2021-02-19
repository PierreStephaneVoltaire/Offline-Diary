use std::ptr::null;
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use crate::data;
use crate::data::models;
use crate::data::models::{ create_moods};
use crate::data::schema;

pub fn insert_new_mood(
    conn: &SqliteConnection,
    user_mood: create_moods,
) -> Result<models::mood, String> {
    use crate::data::schema::mood::dsl::*;

    let insert_res:QueryResult<usize> = diesel::insert_into(mood).values(&user_mood).execute(conn);
    if !insert_res.is_ok() {
        return  Err(insert_res.err().unwrap().to_string());
    }

    let mood_res = mood.order(id.desc()).first(conn) as Result<models::mood, diesel::result::Error>;
    if mood_res.is_ok() {
        Ok(mood_res.unwrap())
    } else {
        Err(mood_res.err().unwrap().to_string())
    }
}