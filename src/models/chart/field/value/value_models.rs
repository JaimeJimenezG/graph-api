use diesel::{Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize)]
pub struct Value {
  pub id: i32,
  pub field_id: i32,
  pub value: i32,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime
}