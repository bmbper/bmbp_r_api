use crate::sql::types::OrmDelete;
use crate::sql::render::util::{OrmJoinTableRender, OrmTableRender, OrmWhereRender};

pub struct OrmDeleteRender{
    delete: OrmDelete
}

impl OrmTableRender for OrmDeleteRender {

}
impl OrmJoinTableRender for OrmDeleteRender {

}
impl OrmWhereRender for OrmDeleteRender {

}