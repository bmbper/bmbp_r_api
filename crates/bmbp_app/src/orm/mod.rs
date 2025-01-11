use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Error, Pool, Postgres};
use tokio::sync::OnceCell;
pub static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();
pub(crate) async fn build_pool(url:String,init_pool:u32) -> Result<(), Error> {
    match PgPoolOptions::new()
        .max_connections(init_pool)
        .connect(url.as_str())
        .await
    {
        Ok(pool) => {
            DB_POOL.set(pool.clone()).unwrap();
            Ok(())
        }
        Err(err) => {
            println!("{:?}", err);
            Err(err)
        }
    }
}
