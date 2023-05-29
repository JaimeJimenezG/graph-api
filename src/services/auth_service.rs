use actix::Addr;
use actix_session::Session;
use actix_web::{ post, web::{ Data, Json }, Responder, HttpResponse, get};
use crate::{AppState, DbActor, models::user_models::{ CreateUser, LoginUser, check_auth }};

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
pub async fn login_user(state: Data<AppState>, body: Json<LoginUser>, session: Session) -> impl Responder {
    let user = body.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();
    
    match db.send(LoginUser {
        nickname: user.nickname.to_string(),
        password: user.password.to_string()
    }).await
     {
        Ok(Ok(user)) => {
            
            if user.id >= 1 {
                if !user.active {
                  return HttpResponse::NotFound().json(format!("Innactive User"));
                }

                session.insert("user_id", &user.id).unwrap();
                session.renew();

                return HttpResponse::Ok().json(user);
              } else {
                return HttpResponse::Ok().json("Wrong password or user");
              }
        },
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("Wrong password or user")),
        _ => HttpResponse::InternalServerError().json("Unnable to login"),
    }
}

#[get("/user/logout")]
pub async fn logout_user(_state: Data<AppState>, session: Session) -> impl Responder {
    if check_auth(&session).is_err() {
        return HttpResponse::NotFound().body("No user logged in.");
    }
    session.purge();

    HttpResponse::SeeOther()
        .append_header(("Location", "/login"))
        .body(format!("User logged out successfully."))
}