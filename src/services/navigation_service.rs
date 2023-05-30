use actix::Addr;
use actix_web::{ get, web::Data, Responder, HttpResponse};

use crate::{models::navigation::navigation_models::FetchPublicNavigations, utils::database_utils::{DbActor, AppState}};



#[get("/navigations/public")]
pub async fn fetch_public_navigations(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchPublicNavigations).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No Public navigations found")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve public navigations"),
    }
}