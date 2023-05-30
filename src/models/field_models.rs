use diesel::{Queryable, Insertable, QueryResult, self,  prelude::*};
use serde::{Deserialize, Serialize};
use actix::{Handler, Message};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Value {
  pub id: i32,
  pub name: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime
}