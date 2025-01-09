use crate::OrmValue;
use crate::sql::types::OrmQuery;
use crate::sql::types::part::{OrmDmlColumn, OrmJoinTable, OrmOrderColumn, OrmSchemaTable, OrmTable, OrmWhere};

pub struct OrmInsert {
    table: OrmSchemaTable,
    column: Vec<String>,
    values: Vec<OrmValue>,
    dml_columns: Vec<OrmDmlColumn>,
    query: Option<OrmQuery>,
}
