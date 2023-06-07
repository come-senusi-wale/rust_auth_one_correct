#[macro_use]
extern crate validator_derive;

mod config;
mod models;
mod handlers;
mod db;

use actix_web::{HttpServer, App, web,};
//use deadpool_postgres::ClientWrapper;
//use jsonwebtoken::errors::Error;
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;



#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || ( 

        App::new()
            .app_data(web::Data::new(AppState{
                db: pool.clone(),
                secret: String::from("secret")
            }))
            .route("/", web::get().to(status))
            .route("/user{_:/?}", web::get().to(get_user))
            .route("/register{_:/?}", web::post().to(register_user))
            .route("/login{_:/?}", web::post().to(login_user))
            .route("/protected{_:/?}", web::post().to(protected))
    ))
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
   
}
