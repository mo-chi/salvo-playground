use salvo::prelude::{Listener, Server, TcpListener};

mod controllers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    let routes = routes::routes();

    Server::new(acceptor).serve(routes).await;
}
