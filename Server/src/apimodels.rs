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
               gdrive: String) -> CreateUser {
        CreateUser {
            name: name,
            gdrive: gdrive,
        }
    }
}
