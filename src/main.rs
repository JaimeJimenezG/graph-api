use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use actix_web::{web::Data, App, HttpServer, http, cookie::Key};
use actix_cors::Cors;
use dotenv::dotenv;
use services::{auth_service::{login_user, logout_user, register_user}, user_service::{fetch_users, fetch_user_charts, create_user_chart, fetch_user_navigations, fetch_user_groups}, navigation_service::fetch_public_navigations, group_service::fetch_groups};
use utils::database_utils::{AppState, DbActor, get_pool};
use std::env;
mod services;
mod models;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let secret_key = Key::generate();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = actix::SyncArbiter::start(5, move || DbActor(pool.clone()));
    
    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin("http://localhost:5173")
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().ends_with(b"5173")
        })
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

        App::new()
            .wrap(SessionMiddleware::builder(
                CookieSessionStore::default(),
                secret_key.clone())
                .cookie_secure(false)
                .build())
            .wrap(cors)
            .app_data(Data::new(AppState { db: db_addr.clone() }))
            .service(login_user)
            .service(logout_user)
            .service(register_user)
            .service(fetch_users)
            .service(fetch_user_charts)
            .service(create_user_chart)
            .service(fetch_user_navigations)
            .service(fetch_public_navigations)
            .service(fetch_groups)
            .service(fetch_user_groups)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
