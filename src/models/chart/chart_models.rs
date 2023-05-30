use diesel::{Queryable, Insertable, QueryResult};
use serde::{Deserialize, Serialize};
use actix::Message;
use crate::schema::charts;

#[derive(Queryable, Serialize)]
pub struct Chart {
  pub id: i32,
  pub title: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub active: bool
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Chart>")]
pub struct CreateChart {
  pub title: String
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Chart>>")]
pub struct FetchUserChart;

#[derive(Insertable, Serialize)]
#[diesel(table_name=charts)]
pub struct NewChart {
  pub title: String,
  }
