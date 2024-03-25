use reqwest::{self, Client};
use std::net::TcpListener;

#[tokio::test]
async fn good_request() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let body = "name=Aleksandar&email=prokopovic75%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Couldn't send request");

    assert_eq!(200, response.status().as_u16())
}
#[tokio::test]
async fn bad_request() {
    let adrress = spawn_app();

    let client = Client::new();

    let test_cases = vec![
        ("name=Aleksandar", "Missing email"),
        ("email=prokopovic75%40gmail.com", "Missing name"),
        ("", "Missing both email and name"),
    ];

    for (body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", adrress))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Couldn't send request");

        assert_eq!(400, response.status().as_u16(), "{}", error_msg)
    }
}
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
