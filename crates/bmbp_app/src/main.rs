use crate::config::AppConfig;
use crate::orm::build_pool;
use salvo::__private::tracing;
use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server};
use tracing_log::log::info;

mod apc;
mod bpc;
mod config;
mod core;
mod orm;
mod tpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let app_config = config::load_app_config("./bmbp.toml").unwrap_or(AppConfig::default());
    let host = app_config.server.host.clone();
    let port = app_config.server.port.clone();
    let address = format!("{}:{}", host, port);
    info!("启动服务, 即将监听{}", address);

    info!("初始化数据库连接...");
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        app_config.datasource.username,
        app_config.datasource.password,
        app_config.datasource.host,
        app_config.datasource.port,
        app_config.datasource.database
    );
    build_pool(database_url, app_config.datasource.init_size).await?;

    let mut router = Router::new();
    router = core::build_core_route(router);
    let acceptor = TcpListener::new(address).bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}
