use crate::sql::types::OrmQuery;
use crate::sql::builder::util::{OrmJoinTableBuilder, OrmTableBuilder, OrmWhereBuilder};

pub struct OrmQueryBuilder {
    query: OrmQuery,
}
impl OrmTableBuilder for OrmQueryBuilder{

}
impl OrmJoinTableBuilder for OrmQueryBuilder{

}
impl OrmWhereBuilder for OrmQueryBuilder{

}