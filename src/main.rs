pub mod data;
pub mod models;
pub mod handlers;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::handlers::{get_persons, get_person, create_person, delete_person};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/persons/", web::post().to(create_person))
            .route("/persons/", web::get().to(get_persons))
            .route("/persons/{id}", web::get().to(get_person))
            .route("/persons/{id}", web::delete().to(delete_person))
    })
    .bind("127.0.0.1:8080").expect("Failed to bind to localhost port 8080.")
    .run()
    .await
}
