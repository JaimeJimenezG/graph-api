use actix::Handler;
use diesel::{QueryResult, RunQueryDsl};
use crate::schema::charts::dsl::*;

use super::chart_models::{CreateChart, Chart, NewChart, FetchUserChart};


impl Handler<CreateChart> for crate::utils::database_utils::DbActor {
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
  
  impl Handler<FetchUserChart> for crate::utils::database_utils::DbActor {
    type Result = QueryResult<Vec<Chart>>;
  
    fn handle(&mut self, _msg: FetchUserChart, _ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Fetch User Chart: Unable to establish connection");
  
      charts.get_results::<Chart>(&mut conn)
    }
  }