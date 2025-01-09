use crate::orm::build_pool;
use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server};
use salvo::__private::tracing;
use tracing_log::log::info;

mod apc;
mod bpc;
mod core;
mod orm;
mod tpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    info!("初始化数据库连接...");
    build_pool().await?;
    let mut router = Router::new();
    router = core::build_core_route(router);
    let acceptor = TcpListener::new("127.0.0.1:36001").bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}
