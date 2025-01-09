use crate::sql::types::OrmDelete;
use crate::sql::builder::util::{OrmJoinTableBuilder, OrmTableBuilder, OrmWhereBuilder};

pub struct OrmDeleteBuilder{
    delete: OrmDelete
}

impl OrmTableBuilder for OrmDeleteBuilder{

}
impl OrmJoinTableBuilder for OrmDeleteBuilder{

}
impl OrmWhereBuilder for OrmDeleteBuilder{

}