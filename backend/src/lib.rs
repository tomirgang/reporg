pub mod models;
pub mod services;

use crate::services::cafe::{future_cafes, create_cafe};
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use models::DbPool;

pub fn run(
    listener: TcpListener,
    db_pool: DbPool
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::scope("/cafe").service(future_cafes).service(create_cafe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}