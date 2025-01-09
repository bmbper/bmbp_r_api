use crate::sql::types::part::{OrmDmlColumn, OrmJoinTable, OrmOrderColumn, OrmTable, OrmWhere};

pub struct OrmUpdate {
    table: Vec<OrmTable>,
    join_table: Vec<OrmJoinTable>,
    dml_column: Vec<OrmDmlColumn>,
    where_: Option<OrmWhere>,
    order_by: Vec<OrmOrderColumn>,
    limit: Option<u64>,
    offset: Option<u64>,
}


