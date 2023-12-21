use actix_session::storage::RedisSessionStore;
use backend::models::{establish_connection, DbPool};
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: DbPool,
}

pub async fn spawn_app() -> TestApp {
    let connection_pool = establish_connection();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();

    let redis = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    let server =
        backend::run(listener, redis, connection_pool.clone());
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
