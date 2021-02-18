use std::ptr::null;

use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;

use crate::apimodels::CreateUser;
use crate::data;
use crate::data::models;
use crate::data::password::insert_new_password;
use crate::data::schema;

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    conn: &SqliteConnection,
    user: CreateUser,
) -> Result<models::Users, String> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::data::schema::users::dsl::*;

    let mut new_user = models::CreateUsers {
        name: user.name,
        gdrive: user.gdrive,
        password: 0,
    };

    let user_password = models::CreatePassword {
        passwordval: "".to_string(),
        q1: "".to_string(),
        q2: "".to_string(),
        q3: "".to_string(),
        a1: "".to_string(),
        a2: "".to_string(),
        a3: "".to_string(),
        wipe_attempt: 0,
    };
    let password_res = insert_new_password(conn, user_password).unwrap();

    let password_id = password_res.id;
    new_user.password = password_id;
    let insert = diesel::insert_into(users).values(&new_user).execute(conn);
    let insert_user = users.order(id.desc()).first(conn) as Result<models::Users, diesel::result::Error>;
    if insert_user.is_ok() {
        Ok(insert_user.unwrap())
    } else {
        Err("Couldn't insert user object".to_string())
    }
}