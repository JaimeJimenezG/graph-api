use diesel::{Queryable, Insertable, QueryResult, self,  prelude::*};
use serde::{Serialize, Deserialize};
use actix::{Handler, Message};

use crate::schema::users::dsl::*;
use crate::schema::groups::dsl::*;
use crate::schema::{users, group_users};
use super::group_models::Group;

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
#[rtype(result = "QueryResult<bool>")]
pub struct LoginUser {
  pub nickname: String,
  pub password: String
}

impl Handler<FetchUserGroups> for crate::services::database_service::DbActor {
  type Result = QueryResult<Vec<Group>>;

  fn handle(&mut self, _msg: FetchUserGroups, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    groups.
      inner_join(group_users::table)
      .filter(group_users::user_id.eq(_msg.user_id))
      .select(groups::all_columns())
      .get_results::<Group>(&mut conn)
  }
}


impl Handler<FetchUser> for crate::services::database_service::DbActor {
  type Result = QueryResult<Vec<User>>;

  fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    users.get_results::<User>(&mut conn)
  }
}

impl Handler<CreateUser> for crate::services::database_service::DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Create User Charts: Unable to establish connection");

    let new_user = NewUser {
      name: msg.name,
      first_name: msg.first_name,
      second_name: msg.second_name,
      nickname: msg.nickname,
      password: msg.password,
    };

    diesel::insert_into(users)
      .values(new_user)
      .get_result::<User>(&mut conn)
  }
}

impl Handler<LoginUser> for crate::services::database_service::DbActor {
  type Result = QueryResult<bool>;

  fn handle(&mut self, _msg: LoginUser, _ctx: &mut Self::Context) -> Self::Result {

    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    let fetched_user = &mut users
      .filter(nickname.eq(_msg.nickname))
      .filter(password.eq(_msg.password))
      .limit(1)
      .get_result::<User>(&mut conn)?;
    
      if fetched_user.id >= 1 {
          if !fetched_user.active {
            print!("User is not active.");
              return Ok(false);
          }
          print!("User logged in successfully.");
          // Study how to implement this
          // Session Module
          // session.insert("user_id", &fetched_user.id)?;
          // session.renew();
            

          Ok(true)
        } else {
          print!("User not found.");
          Ok(false)
        }
  }
}








