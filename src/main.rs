mod config;
mod controllers;
mod dto;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::{App, HttpServer, dev::Server, middleware::Logger, web};
use config::{cors, database, dotenv, multipart};
use routes::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::init();

    let db: web::Data<database::AppState> = database::init().await;

    let address =
        std::env::var("ADDRESS").expect("Environment variable `ADDRESS` must be defined.");

    let port: u16 = std::env::var("PORT")
        .expect("Environment variable `PORT` must be defined.")
        .parse()
        .unwrap();

    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(multipart::init())
            .wrap(cors::options())
            .wrap(Logger::default())
            .configure(router)
    })
    .bind((address, port))?
    .run();

    println!("Server running on port {port}! 🚀");

    server.await
}
