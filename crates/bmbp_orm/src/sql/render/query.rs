use crate::sql::types::OrmQuery;
use crate::sql::render::util::{OrmJoinTableRender, OrmTableRender, OrmWhereRender};

pub struct OrmQueryRender {
    query: OrmQuery,
}
impl OrmTableRender for OrmQueryRender{

}
impl OrmJoinTableRender for OrmQueryRender{

}
impl OrmWhereRender for OrmQueryRender{

}