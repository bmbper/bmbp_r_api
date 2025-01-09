use crate::sql::types::{OrmUpdate};
use crate::sql::builder::query::OrmQueryBuilder;
use crate::sql::builder::util::{OrmJoinTableBuilder, OrmTableBuilder, OrmWhereBuilder};

pub struct OrmUpdateBuilder{
    update: OrmUpdate
}

impl OrmTableBuilder for OrmUpdateBuilder{

}
impl OrmJoinTableBuilder for OrmUpdateBuilder{

}
impl OrmWhereBuilder for OrmUpdateBuilder{

}