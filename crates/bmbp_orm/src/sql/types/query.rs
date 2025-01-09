use crate::sql::types::part::{OrmJoinTable, OrmOrderColumn, OrmSelectColumn, OrmTable, OrmWhere};

pub struct OrmQuery {
    select: Vec<OrmSelectColumn>,
    table: Vec<OrmTable>,
    join_table: Vec<OrmJoinTable>,
    where_: Option<OrmWhere>,
    order_by: Vec<OrmOrderColumn>,
    group_by: Vec<OrmSelectColumn>,
    having: Option<OrmWhere>,
    limit: Option<u64>,
    offset: Option<u64>,
    union: Vec<OrmQuery>,
    union_all: Vec<OrmQuery>,
}
