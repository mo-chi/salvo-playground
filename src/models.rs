use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub mod user {
    use salvo::prelude::Extractible;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Extractible)]
    #[salvo(extract(default_source(from = "body")))]
    pub struct Request {
        pub name: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Response {
        pub id: u64,
        pub name: String,
    }
}
