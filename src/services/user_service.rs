use actix::Addr;
use actix_web::{ get, post, web::{Data, Json, Path}, Responder, HttpResponse};
use crate::{AppState, DbActor, models::{chart_models::{FetchUserChart, CreateChart}, user_models::{FetchUser, FetchUserGroups}, navigation_models::FetchUserNavigations}};


#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/user/{id}/charts")]
pub async fn fetch_user_charts(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserChart).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No charts for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user charts"),
    }
}
#[get("/user/{id}/navigations")]
pub async fn fetch_user_navigations(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserNavigations {
        user_id: id
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No charts for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user charts"),
    }
}

#[get("/user/{id}/groups")]
pub async fn fetch_user_groups(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserGroups {
        user_id: id
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No charts for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user charts"),
    }
}

#[post("/user/{id}/charts")]
pub async fn create_user_chart(state: Data<AppState>, body: Json<CreateChart>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateChart {
        title: body.title.to_string()
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create Chart"),
    }
}
