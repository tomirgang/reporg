use actix_session::storage::RedisSessionStore;
use backend::models::{establish_connection, DbPool};
use std::net::TcpListener;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use chrono::Local;
use dotenvy::dotenv;
use std::env;
use core::str::FromStr;

pub struct TestApp {
    pub address: String,
    pub db_pool: DbPool,
}

pub async fn spawn_app() -> TestApp {
    dotenv().ok();

    let log_level = env::var("REPORG_LOG_LEVEL").unwrap_or("error".to_string());
    let filter = LevelFilter::from_str(&log_level).expect("Invalid log level!");

    let _res = Builder::new()
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
        .try_init();

    let connection_pool = establish_connection();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();

    let redis = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    let server = backend::run(listener, redis, connection_pool.clone());
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn login(app: &TestApp, role: &str) -> reqwest::Client {
    dotenv().ok();

    let api_key = env::var("TESTER_API_KEY").expect("Missing the TESTER_API_KEY environment variable.");

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Client builder error!");

    let response = client.get(&format!("{}/user/tester_login?role={}&key={}", app.address, role, api_key))
        .send()
        .await
        .expect("Failed to execute login request.");

    assert!(response.status().is_success());

    client
}