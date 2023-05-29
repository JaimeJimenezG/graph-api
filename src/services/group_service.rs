use actix::Addr;
use actix_web::{ get, Responder, HttpResponse, web::Data};
use crate::{AppState, DbActor, models::group_models::FetchGroup};


#[get("/groups")]
pub async fn fetch_groups(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchGroup).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}
