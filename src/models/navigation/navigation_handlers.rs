use actix::Handler;
use diesel::{QueryResult, PgConnection, QueryDsl, ExpressionMethods, prelude::*};
use crate::schema::{navigations, user_navigations};
use super::navigation_models::{FetchUserNavigations, Navigation, FetchPublicNavigations};


impl Handler<FetchUserNavigations> for crate::utils::database_utils::DbActor {
    type Result = QueryResult<Vec<Navigation>>;

    fn handle(&mut self, _msg: FetchUserNavigations, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch FetchUserNavigations: Unable to establish connection");
    
        navigations::dsl::navigations.
        inner_join(user_navigations::table)
        .filter(user_navigations::user_id.eq(_msg.user_id))
        .filter(user_navigations::active.eq(true))
        .select(navigations::all_columns)
        .get_results::<Navigation>(&mut conn)
    }
}

impl Handler<FetchPublicNavigations> for crate::utils::database_utils::DbActor {
    type Result = QueryResult<Vec<Navigation>>;

    fn handle(&mut self, _msg: FetchPublicNavigations, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = self.0.get().expect("Fetch FetchPublicNavigations: Unable to establish connection");
    
        navigations::dsl::navigations
        .filter(navigations::public.eq(true))
        .get_results::<Navigation>(&mut conn)
      }
}