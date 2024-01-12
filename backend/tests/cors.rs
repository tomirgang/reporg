mod common;

use common::login;
use reqwest::header::ORIGIN;

#[tokio::test]
async fn cors_localhost_is_ok() {
    let app = common::spawn_app().await;
    let client = login(&app, "Guest").await;

    let response = client
        .get(&format!("{}/api/user/", &app.address))
        .header(ORIGIN, "http://127.0.0.1:8000")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[ignore] // TODO: find out why request is not blocked
#[tokio::test]
async fn cors_example_com_fails() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/api/user/", &app.address))
        .header(ORIGIN, "https:/example.com")
        .send()
        .await
        .expect("Failed to execute request.");

    log::error!("Response is: {}", response.status());

    assert!(response.status() == 400);
}
