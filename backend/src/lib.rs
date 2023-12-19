pub mod models;
pub mod services;

use crate::services::cafe::{future_cafes, create_cafe};
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use models::DbPool;

pub fn run(
    listener: TcpListener,
    db_pool: DbPool
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let cors = Cors::default()
              .allowed_origin("https://reporg.de")
              .allowed_origin("http://127.0.0.1:8000")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::scope("/cafe").service(future_cafes).service(create_cafe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}