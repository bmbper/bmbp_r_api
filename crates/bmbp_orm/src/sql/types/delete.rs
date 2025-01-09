use crate::sql::types::part::{OrmJoinTable, OrmOrderColumn, OrmTable, OrmWhere};

pub struct OrmDelete {
    table: Vec<OrmTable>,
    join_table: Vec<OrmJoinTable>,
    where_: Option<OrmWhere>,
    order_by: Vec<OrmOrderColumn>,
    limit: Option<u64>,
    offset: Option<u64>,
}