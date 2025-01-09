use sqlx::{MySqlPool, PgPool, SqlitePool};

pub struct BmbpOrmPool{
    pub pool:BmbpOrmPoolInner,
}

pub enum  BmbpOrmPoolInner{
    PgPool(PgPool),
    MysqlPool(MySqlPool),
    SqlitePool(SqlitePool)
}