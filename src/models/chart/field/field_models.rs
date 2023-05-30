use diesel::{Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize)]
pub struct Value {
  pub id: i32,
  pub name: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime
}