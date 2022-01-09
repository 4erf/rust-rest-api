#![allow(unused_must_use)]

extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_cors;
extern crate actix_rt;
extern crate bcrypt;
extern crate derive_more;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
extern crate serde;
extern crate uuid;

mod api;
mod config;
mod constants;
mod error;
mod middleware;
mod models;
mod schema;
mod services;
mod utils;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};
use futures::{FutureExt, io};
use std::default::Default;
use std::env;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = config::db::migrate_and_config_db(&db_url);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard()
            )
            .data(pool.clone())
            .data(actix_web::web::PayloadConfig::new(1 << 25))
            .data(actix_web::web::JsonConfig::default().limit(1 << 25))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(crate::middleware::auth_middleware::Authentication) // Comment this line of code if you want to integrate with yew-address-book-frontend
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}