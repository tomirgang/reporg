use actix_session::storage::RedisSessionStore;
use backend::models::establish_connection;
use backend::run;
use std::net::TcpListener;
use std::io::Write;
use env_logger::Builder;
use log::LevelFilter;
use chrono::Local;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    log::debug!("Hello from reporg!");

    let connection_pool = establish_connection();

    let address = format!("127.0.0.1:8000");
    let listener = TcpListener::bind(address)?;

    let redis = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    run(listener, redis, connection_pool)?.await
}
