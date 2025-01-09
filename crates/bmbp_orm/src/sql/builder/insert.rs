use crate::sql::types::OrmInsert;
use crate::sql::builder::util::OrmTableBuilder;

pub struct OrmInsertBuilder{
    insert: OrmInsert
}
impl OrmTableBuilder for OrmInsertBuilder{
}