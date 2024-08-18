extern crate salvo_playground;

use salvo::catcher::Catcher;
use salvo::core::test::{ResponseExt, TestClient};
use salvo::prelude::{Service, StatusCode};
use salvo_playground::{controllers, routes};

#[tokio::test]
async fn test_404() {
    let routes = routes::routes();
    let service = Service::new(routes.router()).catcher(Catcher::default().hoop(controllers::handle4xx));

    let mut response = TestClient::get("http://localhost:5800/404")
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(response.status_code.unwrap(), StatusCode::NOT_FOUND);
    assert_eq!(result, r#"{"message":"404: Not Found"}"#);
}

#[tokio::test]
async fn test_405() {
    let routes = routes::routes();
    let service = Service::new(routes.router()).catcher(Catcher::default().hoop(controllers::handle4xx));

    let mut response = TestClient::post("http://localhost:5800")
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(
        response.status_code.unwrap(),
        StatusCode::METHOD_NOT_ALLOWED
    );
    assert_eq!(result, r#"{"message":"405: Method Not Allowed"}"#);
}
