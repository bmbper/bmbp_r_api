use sqlx::{MySql, MySqlPool, PgPool, Pool, Postgres, SqlitePool};

pub struct BmbpOrmPool{
    pub pool:BmbpOrmPoolInner,
}

pub enum  BmbpOrmPoolInner{
    PgPool(PgPool),
    MysqlPool(MySqlPool),
    SqlitePool(SqlitePool)
}