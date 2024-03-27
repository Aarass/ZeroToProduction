mod test_app;

#[tokio::test]
async fn good_request() {
    let test_app = test_app::spawn_app().await;

    let client = reqwest::Client::new();

    let body = "name=Aleksandar&email=prokopovic75%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Couldn't send request");

    assert_eq!(200, response.status().as_u16());
}
#[tokio::test]
async fn bad_request() {
    let test_app = test_app::spawn_app().await;

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Aleksandar", "Missing email"),
        ("email=prokopovic75%40gmail.com", "Missing name"),
        ("", "Missing both email and name"),
    ];

    for (body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Couldn't send request");

        assert_eq!(400, response.status().as_u16(), "{}", error_msg)
    }
}
