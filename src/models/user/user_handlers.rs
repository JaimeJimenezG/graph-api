use actix::Handler;
use diesel::{QueryResult, QueryDsl, RunQueryDsl, ExpressionMethods, Table};

use crate::{models::group::group_models::Group, schema::{group_users, users::dsl::*, groups::dsl::*}};

use super::user_models::{FetchUserGroups, FetchUser, User, CreateUser, NewUser, LoginUser};


impl Handler<FetchUserGroups> for crate::utils::database_utils::DbActor {
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
  
  
  impl Handler<FetchUser> for crate::utils::database_utils::DbActor {
    type Result = QueryResult<Vec<User>>;
  
    fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");
  
      users.get_results::<User>(&mut conn)
    }
  }
  
  impl Handler<CreateUser> for crate::utils::database_utils::DbActor {
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
  
  impl Handler<LoginUser> for crate::utils::database_utils::DbActor {
    type Result = QueryResult<User>;
    
    fn handle(&mut self, _msg: LoginUser, _ctx: &mut Self::Context) -> Self::Result {
  
      let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");
  
      users
        .filter(nickname.eq(_msg.nickname))
        .filter(password.eq(_msg.password))
        .limit(1)
        .get_result::<User>(&mut conn)
    }
  }
  
  
