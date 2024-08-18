extern crate salvo_playground;

use salvo::core::test::{ResponseExt, TestClient};
use salvo::prelude::{Service, StatusCode};
use salvo_playground::{models, routes};

#[tokio::test]
async fn test_get_users() {
    let routes = routes::routes();
    let service = Service::new(routes.router());

    let mut response = TestClient::get("http://localhost:5800/users")
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    assert_eq!(result, r#"[{"id":1,"name":"alice"},{"id":2,"name":"bob"}]"#);
}

#[tokio::test]
async fn test_get_user() {
    let routes = routes::routes();
    let service = Service::new(routes.router());

    let mut response = TestClient::get("http://localhost:5800/users/1")
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    assert_eq!(result, r#"{"id":1,"name":"alice"}"#);
}

#[tokio::test]
async fn test_post_user() {
    let routes = routes::routes();
    let service = Service::new(routes.router());

    let request = models::user::Request {
        name: "alice".to_string(),
    };

    let mut response = TestClient::post("http://localhost:5800/users")
        .json(&request)
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(response.status_code.unwrap(), StatusCode::CREATED);
    assert_eq!(result, "");
}

#[tokio::test]
async fn test_put_user() {
    let routes = routes::routes();
    let service = Service::new(routes.router());

    let request = models::user::Request {
        name: "alice".to_string(),
    };

    let mut response = TestClient::put("http://localhost:5800/users/1")
        .json(&request)
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    assert_eq!(result, "");
}

#[tokio::test]
async fn test_delete_user() {
    let routes = routes::routes();
    let service = Service::new(routes.router());

    let mut response = TestClient::delete("http://localhost:5800/users/1")
        .send(&service)
        .await;

    let result = response.take_string().await.unwrap();

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    assert_eq!(result, "");
}
