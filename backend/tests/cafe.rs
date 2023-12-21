mod common;

use backend::models::cafe::Cafe;
use chrono::NaiveDateTime;

#[tokio::test]
async fn future_cafes_works() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/cafe/", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let content = response.text().await.expect("Getting body failed!");

    let _cafes: Vec<Cafe> = serde_json::from_str(&content).expect("Parsing body failed!");
}

#[tokio::test]
async fn create_cafe_ok() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    let body = "location=Haus%20des%20Gastes&address=Maria-Dorothea-Stra%C3%9Fe%208%2C%2091161%20Hilpoltstein&date=2018-06-12T19%3A30";

    let response = client
        .post(&format!("{}/cafe/", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let content = response.text().await.expect("Getting body failed!");

    let cafe: Cafe = serde_json::from_str(&content).expect("Parsing cafe failed!");

    assert!(cafe.id >= 0);
    assert_eq!(cafe.location, String::from("Haus des Gastes"));
    assert_eq!(
        cafe.address,
        String::from("Maria-Dorothea-Straße 8, 91161 Hilpoltstein")
    );

    let date = NaiveDateTime::parse_from_str("2018-06-12T19:30", "%Y-%m-%dT%H:%M")
        .expect("Parsing date failed!");
    assert_eq!(cafe.date, date);
}
