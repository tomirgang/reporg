use actix_session::storage::RedisSessionStore;
use backend::models::establish_connection;
use backend::run;
use backend::settings::Settings;
use chrono::Local;
use core::str::FromStr;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::net::TcpListener;

#[actix_web::main]
async fn main() {
    let settings = Settings::new().unwrap();

    let filter = LevelFilter::from_str(&settings.log.level).expect("Invalid log level!");

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, filter)
        .init();

    log::debug!("Hello from reporg!");

    let db = establish_connection(&settings.database.url).await.unwrap();

    let address = format!("127.0.0.1:8000");
    let listener = TcpListener::bind(address).expect("Failed to bind to address.");

    let redis = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    run(listener, redis, db, settings).await.await.unwrap();
}
