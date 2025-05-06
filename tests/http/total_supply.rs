use serde::Deserialize;

use crate::common::spawn_app;

#[derive(Deserialize, Debug)]
struct Body {
    result: String,
}

#[tokio::test]
async fn total_supply_works() {
    // Arrange.
    let app = spawn_app().await;

    // Act.
    let response = app
        .api_client
        .get(&format!("{}/v1/total", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert.
    assert!(response.status().is_success());

    let body = response
        .json::<Body>()
        .await
        .expect("Failed to extract body");

    assert_eq!(body.result, "4242.00");
}
