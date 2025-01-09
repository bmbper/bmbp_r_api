use crate::sql::types::{OrmUpdate};
use crate::sql::render::util::{OrmJoinTableRender, OrmTableRender, OrmWhereRender};

pub struct OrmUpdateRender{
    update: OrmUpdate
}

impl OrmTableRender for OrmUpdateRender {

}
impl OrmJoinTableRender for OrmUpdateRender {

}
impl OrmWhereRender for OrmUpdateRender {

}