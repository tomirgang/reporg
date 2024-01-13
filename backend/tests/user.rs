mod common;

use backend::{settings::Settings, models::user::User};
use common::login;

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
    let settings = Settings::new().unwrap();

    let app = common::spawn_app().await;

    let api_key = settings.tester.key;

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

    assert!(content.contains("Organizer"));
}

#[tokio::test]
async fn user_list_works() {
    let app = common::spawn_app().await;
    let client = login(&app, "Organizer").await;

    let response = client
        .get(&format!("{}/api/user/list", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let content = response.text().await.expect("Getting body failed!");

    let _users: Vec<User> = serde_json::from_str(&content).expect("Parsing body failed!");
}

#[tokio::test]
async fn user_list_is_protected() {
    let app = common::spawn_app().await;
    let client = login(&app, "Guest").await;

    let response = client
        .get(&format!("{}/api/user/list", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), 403);

    let client = login(&app, "Supporter").await;

    let response = client
        .get(&format!("{}/api/user/list", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), 403);
}
