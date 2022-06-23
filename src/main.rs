#![allow(dead_code)]
#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer, HttpResponse, Result, get};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use time::Duration;
mod auth_handler;
mod email_service;
mod errors;
mod invitation_handler;
mod models;
mod register_handler;
mod schema;
mod utils;


struct AppState {
    foo: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "harmonious-back=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {

        App::new()
            .app_data(web::Data::new(pool.clone()))
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(Duration::days(1))
                    .secure(false),
            ))
            .app_data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            .configure(app_config)
    })

    .bind((std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()), std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().expect("PORT must be a number")))?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .app_data(web::Data::new(AppState {
                foo: "bar".to_string(),
            }))
            .service(index)
            .service(register)


            .service(
                web::scope("/api")
                    .service(invitation_handler::post_invitation)
                    .service(register_handler::register_user)
                    .service(auth_handler::login)
                    .service(auth_handler::logout)
                    .service(auth_handler::get_me),
            )
    );
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!(r"../static/index.html")))

}

#[get("/register")]
async fn register() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!(r"../static/register.html")))

}
