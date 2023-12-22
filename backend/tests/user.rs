mod common;

use common::login;
use dotenvy::dotenv;
use log::debug;
use std::env;

#[tokio::test]
async fn tester_login_admin_ok() {
    let app = common::spawn_app().await;
    let _client = login(&app, "Admin");
}

#[tokio::test]
async fn tester_login_organizer_ok() {
    let app = common::spawn_app().await;
    let _client = login(&app, "Organizer");
}

#[tokio::test]
async fn tester_login_supporter_ok() {
    let app = common::spawn_app().await;
    let _client = login(&app, "Supporter");
}

#[tokio::test]
async fn tester_login_guest_ok() {
    let app = common::spawn_app().await;
    let _client = login(&app, "Guest");
}

#[tokio::test]
async fn tester_login_invalid_role_forbidden() {
    let app = common::spawn_app().await;

    dotenv().ok();

    let api_key =
        env::var("TESTER_API_KEY").expect("Missing the TESTER_API_KEY environment variable.");

    let client = reqwest::Client::new();

    let response = client
        .get(&format!(
            "{}/api/user/tester_login?role={}&key={}",
            &app.address, "SuperAdmin", api_key
        ))
        .send()
        .await
        .expect("Failed to execute login request.");

    assert!(response.status() == 403);
}

#[tokio::test]
async fn tester_login_invalid_key_forbidden() {
    let app = common::spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!(
            "{}/api/user/tester_login?role={}&key={}",
            &app.address, "Admin", "SomeWrongKey"
        ))
        .send()
        .await
        .expect("Failed to execute login request.");

    assert!(response.status() == 403);
}

#[tokio::test]
async fn tester_login_session_stored() {
    let app = common::spawn_app().await;
    let client = login(&app, "Organizer").await;

    let response = client
        .get(&format!("{}/api/user/", &app.address))
        .send()
        .await
        .expect("Failed to execute user request.");

    assert!(response.status().is_success());

    let content = response.text().await.expect("Getting body failed!");

    debug!("{}", content);

    assert!(content.contains("Organizer"));
}
