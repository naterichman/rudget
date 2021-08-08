#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod db;
mod error;
mod models;
mod routes;
mod schema;

use crate::routes::users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    HttpServer::new(|| {
        App::new()
            .configure(users::user_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
