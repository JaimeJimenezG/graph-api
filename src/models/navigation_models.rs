use diesel::{Queryable, QueryResult, self,  prelude::*};
use serde::Serialize;
use actix::{Handler, Message};

use crate::schema::{navigations, user_navigations};

#[derive(Queryable, Serialize)]
pub struct Navigation {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub public: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub component_id: i32,
    pub icon: String,
    pub order: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Navigation>>")]
pub struct FetchUserNavigations {
    pub user_id: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Navigation>>")]
pub struct FetchPublicNavigations;

impl Handler<FetchUserNavigations> for crate::services::database_service::DbActor {
    type Result = QueryResult<Vec<Navigation>>;

    fn handle(&mut self, _msg: FetchUserNavigations, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch FetchUserNavigations: Unable to establish connection");
    
        navigations::dsl::navigations.
        inner_join(user_navigations::table)
        .filter(user_navigations::user_id.eq(_msg.user_id))
        .filter(user_navigations::active.eq(true))
        .select(navigations::all_columns)
        .order(navigations::order.asc())
        .get_results::<Navigation>(&mut conn)
    }
}

impl Handler<FetchPublicNavigations> for crate::services::database_service::DbActor {
    type Result = QueryResult<Vec<Navigation>>;

    fn handle(&mut self, _msg: FetchPublicNavigations, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> = self.0.get().expect("Fetch FetchPublicNavigations: Unable to establish connection");
    
        navigations::dsl::navigations
        .filter(navigations::public.eq(true))
        .get_results::<Navigation>(&mut conn)
      }
}