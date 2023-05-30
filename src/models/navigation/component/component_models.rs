use diesel::{Queryable, Insertable, QueryResult, self,  prelude::*};
use serde::{Serialize};
use actix::{Handler, Message};
use crate::schema::groups::dsl::*;
use crate::schema::groups;

#[derive(Queryable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String
}