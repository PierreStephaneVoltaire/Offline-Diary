#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::{Deserialize, Serialize};
use serde::ser::Serializer;
use serde_json;
use serde_json::{Map, Value};
use std::collections::HashMap;
use crate::data::schema::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUser {
    pub(crate) name: String,
    pub(crate) gdrive: String,
    pub     password: String,
    pub     q1: String,
    pub     q2: String,
    pub     q3: String,
    pub    a1: String,
    pub    a2: String,
    pub    a3: String,
    pub    wipe_attempt : i32,
}

impl CreateUser {
    pub fn new(
               name: String,
               gdrive: String,     password: String,
                   q1: String,
                    q2: String,
                   q3: String,
                   a1: String,
                  a2: String,
                 a3: String,
                   wipe_attempt : i32,) -> CreateUser {
        CreateUser {
            name,
            gdrive,
            password,
            q1,
            q2,
            q3,
            a1,
            a2,
            a3,
            wipe_attempt
        }
    }
}
