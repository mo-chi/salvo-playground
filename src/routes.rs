use super::controllers;
use salvo::catcher::Catcher;
use salvo::prelude::{Router, Service};

pub fn routes() -> Service {
    let router = Router::new().push(index()).push(users());

    Service::new(router).catcher(error_handler())
}

fn index() -> Router {
    Router::with_path("/").get(controllers::index)
}

fn users() -> Router {
    Router::with_path("/users")
        .get(controllers::users::list)
        .post(controllers::users::create)
        .push(
            Router::with_path("<id>")
                .get(controllers::users::get)
                .put(controllers::users::update)
                .delete(controllers::users::delete),
        )
}

fn error_handler() -> Catcher {
    Catcher::default().hoop(controllers::handle4xx)
}
