use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server};
use tracing_subscriber::fmt::init;
use tracing_subscriber::util::SubscriberInitExt;

mod apc;
mod bpc;
mod core;
mod embed;
mod orm;
mod router;
mod tpc;

#[derive(Debug, sqlx::FromRow)]
struct BmbpDict {
    data_id: String,
    code: String,
    code_path: String,
}
#[tokio::main]
async fn main() {
    if let Ok(pool) = orm::build_pool().await {
        if let Ok(user) =
            sqlx::query_as::<_, BmbpDict>("SELECT data_id, code, code_path FROM gmgp_setting_dict")
                .bind(42)
                .fetch_one(&pool)
                .await
        {
            println!("user: {:?}", user);
        }
    }

    tracing_subscriber::fmt().init();
    let mut router = Router::new();
    router = embed::build_template_router(router);
    router = embed::build_static_router(router);
    let acceptor = TcpListener::new("127.0.0.1:36001").bind().await;
    Server::new(acceptor).serve(router).await;
}
