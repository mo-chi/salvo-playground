use super::models::{self, MessageResponse};
use salvo::prelude::{handler, Json, Response, StatusCode};

#[handler]
pub async fn index(res: &mut Response) {
    let response = MessageResponse {
        message: "Hello World".to_string(),
    };

    res.render(Json(response));
}

#[handler]
pub async fn handle4xx(res: &mut Response) {
    if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
        let response = MessageResponse {
            message: "404: Not Found".to_string(),
        };

        res.status_code(StatusCode::NOT_FOUND);
        res.render(Json(response));
    } else if StatusCode::METHOD_NOT_ALLOWED == res.status_code.unwrap_or(StatusCode::METHOD_NOT_ALLOWED) {
        let response = MessageResponse {
            message: "405: Method Not Allowed".to_string(),
        };

        res.status_code(StatusCode::METHOD_NOT_ALLOWED);
        res.render(Json(response));
    }
}

pub mod users {
    use super::models::user;
    use salvo::prelude::{handler, Json, Request, Response, StatusCode};

    #[handler]
    pub async fn list(res: &mut Response) {
        let response = vec![
            user::Response {
                id: 1,
                name: "alice".to_string(),
            },
            user::Response {
                id: 2,
                name: "bob".to_string(),
            },
        ];

        res.render(Json(response));
    }

    #[handler]
    pub async fn get(res: &mut Response) {
        let response = user::Response {
            id: 1,
            name: "alice".to_string(),
        };

        res.render(Json(response));
    }

    #[handler]
    pub async fn create(req: &mut Request, res: &mut Response) {
        let user = req.parse_json::<user::Request>().await.unwrap();
        // let user = req.extract::<user::Request>().await.unwrap();
        tracing::debug!("user={:#?}", user);

        res.status_code(StatusCode::CREATED);
    }

    #[handler]
    pub async fn update(req: &mut Request, res: &mut Response) {
        let id = req.param::<u64>("id").unwrap();
        let user = req.parse_json::<user::Request>().await.unwrap();
        tracing::debug!("id={:?}, user={:#?}", id, user);

        res.status_code(StatusCode::OK);
    }

    #[handler]
    pub async fn delete(req: &mut Request, res: &mut Response) {
        let id = req.param::<u64>("id").unwrap();
        tracing::debug!("id={:?}", id);

        res.status_code(StatusCode::OK);
    }
}
