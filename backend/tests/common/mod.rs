use actix_session::storage::RedisSessionStore;
use backend::models::{establish_connection, DbPool};
use backend::settings::Settings;
use chrono::Local;
use core::str::FromStr;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: DbPool,
}

pub async fn spawn_app() -> TestApp {
    let settings = Settings::new().unwrap_or_else(|e| {
        panic!("Settings error: {}", e);
    });

    let filter = LevelFilter::from_str(&settings.log.level).expect("Invalid log level!");

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
    let settings = Settings::new().unwrap();

    let api_key = settings.tester.key;

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Client builder error!");

    let response = client
        .get(&format!(
            "{}/api/user/tester_login?role={}&key={}",
            app.address, role, api_key
        ))
        .send()
        .await
        .expect("Failed to execute login request.");

    assert!(response.status().is_success());

    client
}
