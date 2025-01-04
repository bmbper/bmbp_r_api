use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server};
use tracing_subscriber::util::SubscriberInitExt;

mod apc;
mod bpc;
mod core;
mod embed;
mod orm;
mod router;
mod tpc;
mod ctx;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let mut router = Router::new();
    router = embed::build_template_router(router);
    router = embed::build_static_router(router);
    router = core::build_core_route(router);
    let acceptor = TcpListener::new("127.0.0.1:36001").bind().await;
    Server::new(acceptor).serve(router).await;
}
