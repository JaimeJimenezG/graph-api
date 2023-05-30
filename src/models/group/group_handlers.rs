use actix::Handler;
use diesel::{QueryResult, RunQueryDsl};
use crate::schema::groups::dsl::*;
use super::group_models::{FetchGroup, Group};


impl Handler<FetchGroup> for crate::utils::database_utils::DbActor {
    type Result = QueryResult<Vec<Group>>;
  
    fn handle(&mut self, _msg: FetchGroup, _ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Fetch Group: Unable to establish connection");
  
      groups.get_results::<Group>(&mut conn)
    }
  }
