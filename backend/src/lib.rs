pub mod models;
pub mod services;

use crate::services::cafe::{create_cafe, future_cafes};
use crate::services::identity::{index, login, logout};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::dev::Server;
use actix_web::{cookie::Key, http, web, App, HttpServer};
use models::DbPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    redis: RedisSessionStore,
    db_pool: DbPool,
) -> Result<Server, std::io::Error> {
    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(redis.clone(), secret_key.clone()))
            .service(
                web::scope("/cafe")
                    .service(future_cafes)
                    .service(create_cafe),
            )
            .service(
                web::scope("/user")
                    .service(index)
                    .service(login)
                    .service(logout),
            )
            .app_data(web::Data::new(db_pool.clone()))

    })
    .listen(listener)?
    .run();

    Ok(server)
}
