use diesel::{Queryable, Insertable, QueryResult};
use serde::Serialize;
use actix::Message;

use crate::schema::groups;


#[derive(Queryable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Group>>")]
pub struct FetchGroup;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=groups)]
pub struct NewGroup {
    pub id : i32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}