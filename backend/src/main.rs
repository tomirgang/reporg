use actix_session::storage::RedisSessionStore;
use backend::models::establish_connection;
use backend::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let connection_pool = establish_connection();

    let address = format!("127.0.0.1:8000");
    let listener = TcpListener::bind(address)?;

    let redis = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    run(listener, redis, connection_pool)?.await
}
