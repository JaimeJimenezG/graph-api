use diesel::{Queryable, Insertable, QueryResult, self,  prelude::*};
use serde::{Serialize};
use actix::{Handler, Message};
use crate::schema::groups::dsl::*;
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

impl Handler<FetchGroup> for crate::services::database_service::DbActor {
    type Result = QueryResult<Vec<Group>>;
  
    fn handle(&mut self, _msg: FetchGroup, _ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Fetch Group: Unable to establish connection");
  
      groups.get_results::<Group>(&mut conn)
    }
  }
