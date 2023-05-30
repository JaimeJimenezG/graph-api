use diesel::{Queryable, Insertable, QueryResult, self,  prelude::*};
use serde::{Deserialize, Serialize};
use actix::{Handler, Message};

use crate::schema::charts::dsl::*;
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

impl Handler<CreateChart> for crate::services::database_service::DbActor {
  type Result = QueryResult<Chart>;

  fn handle(&mut self, msg: CreateChart, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Create User Charts: Unable to establish connection");

    let new_charts = NewChart {
      title: msg.title
    };

    diesel::insert_into(charts)
      .values(new_charts)
      .get_result::<Chart>(&mut conn)
  }
}

impl Handler<FetchUserChart> for crate::services::database_service::DbActor {
  type Result = QueryResult<Vec<Chart>>;

  fn handle(&mut self, _msg: FetchUserChart, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User Chart: Unable to establish connection");

    charts.get_results::<Chart>(&mut conn)
  }
}