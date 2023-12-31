use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Given
    let app_address = spawn_app();

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // When
    let response = client
        .get(format!("{app_address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Must be bound...");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener);
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
