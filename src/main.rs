use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server};

mod core;
mod embed;
mod router;
mod apc;
mod bpc;
mod orm;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let mut router = Router::new();
    router = embed::build_template_router(router);
    router = embed::build_static_router(router);
    let acceptor = TcpListener::new("127.0.0.1:36001").bind().await;
    Server::new(acceptor).serve(router).await;
}
