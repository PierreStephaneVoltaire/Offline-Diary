//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer, HttpRequest};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use crate::apimodels::CreateUser;
use crate::data::actions::insert_new_user;

mod data;
mod apimodels;


type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[post("/user")]
async fn create_user(pool: web::Data<DbPool>,
                                      post: web::Json<CreateUser>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let post = web::block(move || insert_new_user(&conn, post.0))
        .await
        .map_err(|e| {
            HttpResponse::InternalServerError().finish()
        })?;
    println!("model: {:?}", post);

    Ok(HttpResponse::Ok().json(post))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // use env file on dev
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(create_user)
    })
        .bind(&bind)?
        .run()
        .await
}
