use std::net::TcpListener;
use backend::models::{DbPool, establish_connection};
use backend::models::cafe::Cafe;

pub struct TestApp {
    pub address: String,
    pub db_pool: DbPool,
}

async fn spawn_app() -> TestApp {
    let connection_pool = establish_connection();
    
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = backend::run(listener, connection_pool.clone()).expect("Failed to bind to address.");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn future_cafes_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/cafe/", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let content = response
        .text()
        .await
        .expect("Getting body failed!");
    
    let _cafes: Vec<Cafe> = serde_json::from_str(&content).expect("Parsing body failed!");
}
