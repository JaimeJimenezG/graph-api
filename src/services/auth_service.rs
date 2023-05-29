use actix::Addr;
use actix_web::{ post, web::{ Data, Json }, Responder, HttpResponse};
use crate::{AppState, DbActor, models::user_models::{ CreateUser, LoginUser }};


#[post("/user/cerate")]
pub async fn register_user(state: Data<AppState>, body: Json<CreateUser>) -> impl Responder {
    let user = body.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    // Enrypt password
    // let password_hash = argon2
    //     .hash_password(data.password.as_bytes(), &salt)
    //     .unwrap()
    //     .to_string();

    match db.send(CreateUser {
        name: user.name.to_string(),
        first_name: user.first_name.to_string(),
        second_name: user.second_name.to_string(),
        nickname: user.nickname.to_string(),
        password: user.password.to_string()
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("Unnable to register user")),
        _ => HttpResponse::InternalServerError().json("Unnable to register user"),
    }
}
#[post("/user/login")]
pub async fn login_user(state: Data<AppState>, body: Json<LoginUser>) -> impl Responder {
    let user = body.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(LoginUser {
        nickname: user.nickname.to_string(),
        password: user.password.to_string()
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("Unnable to login user")),
        _ => HttpResponse::InternalServerError().json("Unnable to login user"),
    }
}

// pub async fn logout_user(session: Session) -> HttpResponse {
//     if check_auth(&session).is_err() {
//         return HttpResponse::NotFound().body("No user logged in.");
//     }

//     session.purge();
//     HttpResponse::SeeOther()
//         .append_header(("Location", "/login"))
//         .body(format!("User logged out successfully."))
// }