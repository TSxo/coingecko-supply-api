use crate::common::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange.
    let app = spawn_app().await;

    // Act.
    let response = app
        .api_client
        .get(&format!("{}/healthz", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert.
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
