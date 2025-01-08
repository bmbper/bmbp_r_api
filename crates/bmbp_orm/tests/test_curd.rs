use bmbp_orm::OrmRecord;
use sqlx::pool::Pool;
use sqlx::Postgres;

pub struct Demo {}

impl OrmRecord for Demo {
    async fn insert(&self, pool: &Pool<Postgres>) -> Result<String, String> {
        Ok("".to_string())
    }
}

#[test]
fn test_curd() {
}
