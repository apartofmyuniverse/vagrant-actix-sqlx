use dotenvy::dotenv;
use log::{info};

use actix_web::{web::Data, App, HttpServer};


mod services;
mod config;

use services::{fetch_users, fetch_user_articles, create_user_article};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use config::{load_config};

pub struct AppState {
    db : Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();
    env_logger::Builder::from_default_env().init();
    info!("Hello, world!");

    let cfg = load_config("config.yml".to_string());

    info!("url: {} port : {}",cfg.service.url,cfg.service.port);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");
    

    HttpServer::new( move || {
        App::new()
        .app_data( Data::new(AppState {db: pool.clone()}))
        .service(create_user_article)
        .service(fetch_users)
        .service(fetch_user_articles)
    })
    .bind((cfg.service.url,cfg.service.port))?
    .run()
    .await
}
