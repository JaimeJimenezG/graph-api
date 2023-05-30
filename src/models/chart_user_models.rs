use diesel::{Queryable, Insertable, QueryResult, self,  prelude::*};
use serde::{Deserialize, Serialize};
use actix::{Handler, Message};

#[derive(Queryable, Serialize, Deserialize)]
pub struct ChartUser {
    pub id: i32,
    pub chart_id: i32,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}