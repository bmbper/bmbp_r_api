use sqlx::{Connection, Error, PgConnection, PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;


pub(crate) async fn build_pool() -> Result<Pool<Postgres>, Error> {
   PgPoolOptions::new().max_connections(5).connect("postgres://bmbp:zgk0130!@127.0.0.1:5432/bmbp").await
}