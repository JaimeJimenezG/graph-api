use diesel::{Queryable, Insertable, QueryResult};
use serde::{Serialize, Deserialize};
use actix:: Message;
use crate::models::group::group_models::Group;
use crate::schema::users;


#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub first_name: String,
    pub second_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub active: bool,
    pub password: String,
    pub nickname: String
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
  pub name: String,
  pub first_name: String,
  pub second_name: String,
  pub nickname: String,
  pub password: String
}

#[derive(Insertable, Serialize)]
#[diesel(table_name=users)]
pub struct NewUser {
  pub name: String,
  pub first_name: String,
  pub second_name: String,
  pub nickname: String,
  pub password: String
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Group>>")]
pub struct FetchUserGroups {
  pub user_id: i32
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct LoginUser {
  pub nickname: String,
  pub password: String
}
