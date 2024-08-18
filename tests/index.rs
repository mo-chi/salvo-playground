extern crate salvo_playground;

use salvo::core::test::{ResponseExt, TestClient};
use salvo::prelude::{Service, StatusCode};
use salvo_playground::routes;

#[tokio::test]
async fn test_index() {
    let routes = routes::routes();
    let service = Service::new(routes.router());

    let mut response = TestClient::get("http://localhost:5800")
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    assert_eq!(result, r#"{"message":"Hello World"}"#);
}
