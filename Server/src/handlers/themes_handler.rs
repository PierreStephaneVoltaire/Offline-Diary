use std::ptr::null;
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use crate::data;
use crate::data::models;
use crate::data::models::{ create_themes};
use crate::data::schema;

pub fn insert_new_theme(
    conn: &SqliteConnection,
    user_theme: create_themes,
) -> Result<models::theme, String> {
    use crate::data::schema::themes::dsl::*;

    let insert_res:QueryResult<usize> = diesel::insert_into(themes).values(&user_theme).execute(conn);
    if !insert_res.is_ok() {
        return  Err(insert_res.err().unwrap().to_string());
    }

    let theme_res = themes.order(id.desc()).first(conn) as Result<models::theme, diesel::result::Error>;
    if theme_res.is_ok() {
        Ok(theme_res.unwrap())
    } else {
        Err(theme_res.err().unwrap().to_string())
    }
}