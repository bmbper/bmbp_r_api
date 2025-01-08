use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Error, Pool, Postgres};
use tokio::sync::OnceCell;
pub static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();
pub(crate) async fn build_pool() -> Result<(), Error> {
    match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://bmbp:zgk0130!@127.0.0.1:5432/bmbp")
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
