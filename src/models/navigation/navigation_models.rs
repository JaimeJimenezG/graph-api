use diesel::{Queryable, QueryResult};
use serde::Serialize;
use actix::Message;


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
