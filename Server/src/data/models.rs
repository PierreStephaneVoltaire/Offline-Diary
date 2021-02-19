use serde::{Deserialize, Serialize};

use crate::data::schema::*;
use diesel::types::{Double, Integer};
use diesel::sql_types;

#[derive(Debug, Clone, Serialize, Queryable)]

pub struct  links  {
        id : i32,
        link: String,
        person_fk : i32,
    }


#[derive(Debug, Clone, Serialize, Queryable)]

pub struct    mentions {
        entry : i32,
        person : i32,

}

#[derive(Debug, Clone, Serialize, Queryable)]

pub struct  mood  {
        id : i32,
        name: String,
    }


#[derive(Debug, Clone, Serialize, Queryable)]

pub struct     nicknames {
        id : i32,
        nickname: String,
        person_fk : i32,
    }


#[derive(Debug, Clone, Serialize, Queryable,Insertable)]
#[table_name = "password"]
pub struct   user_password {
        pub    id : i32,
        pub     passwordval: String,
        pub     q1: String,
        pub     q2: String,
        pub     q3: String,
        pub    a1: String,
        pub    a2: String,
        pub    a3: String,
        pub    wipe_attempt : i32,

}
#[derive(Debug, Clone, Serialize, Queryable,Insertable)]
#[table_name = "password"]
pub struct    CreatePassword  {
        pub passwordval: String,
        pub q1: String,
        pub q2: String,
        pub q3: String,
        pub a1: String,
        pub a2: String,
        pub a3: String,
        pub wipe_attempt : i32,
}





#[derive(Debug, Clone, Serialize, Queryable)]

pub struct    person  {
        id : i32,
        name: String,
        description :Option< String>,
        image :Option< String>,

}

#[derive(Debug, Clone, Serialize, Queryable)]

pub struct   preferences  {
        id : i32,
        autosave :Option< bool>,
        autolock_interval :i32,
        backup :Option< bool>,
        autodelete_interval :Option< i32>,
        online :bool,
        user : i32,
        themes : i32,

}

#[derive(Debug, Clone, Serialize, Queryable)]

pub struct    simping  {
        person : i32,
        entry : i32,
        amount : i32,
        currency: String,
        is_worth : bool,

}

#[derive(Debug, Clone, Serialize, Queryable)]

pub struct    tags  {
        id : i32,
        name: String,
        person_fk  :Option< i32>,
        entry_fk  :Option< i32>,

}

#[derive(Debug, Clone, Serialize, Queryable,Insertable)]

pub struct   theme  {
        id : i32,
        name: String,
        main_color: String,
        font_family: String,
        serif :i32,
    }

#[derive(Debug, Clone, Serialize, Queryable,Insertable)]
#[table_name = "themes"]
pub struct   create_themes  {
        id : i32,
        name: String,
        main_color: String,
        font_family: String,
        serif :i32,
}


#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Users {
      pub  id : i32,
        pub    name: String,
        pub    gdrive: String,
        pub   password : i32,
    }


#[derive(Debug, Clone, Serialize, Queryable,Insertable)]
#[table_name = "users"]
pub struct    CreateUsers  {
        pub    name: String,
        pub    gdrive: String,
        pub   password : i32,

}


