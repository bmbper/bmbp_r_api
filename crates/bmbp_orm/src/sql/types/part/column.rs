use crate::OrmValue;
use crate::sql::types::query::OrmQuery;
use crate::sql::types::{OrmFunc, OrmSchemaTable};


pub enum OrmColumn {
    Simple(OrmSimpleColumn),
    Table(OrmTableColumn),
    Query(OrmQueryColumn),
    ValueColumn(OrmValueColumn),
    FuncColumn(OrmFuncColumn),
    Raw(OrmRawColumn)
}

pub struct OrmSimpleColumn {
    pub column: String,
}
pub struct OrmTableColumn {
    pub table: OrmSchemaTable,
    pub column: String,
}
pub struct OrmQueryColumn {
    pub column: OrmQuery,
}
pub struct OrmValueColumn {
    pub column: OrmValue,
}
pub struct OrmRawColumn {
    pub column: String,
}
pub struct OrmFuncColumn {
    pub func: OrmFunc,
}

pub struct OrmDmlColumn {
    pub column: OrmColumn,
    pub value: OrmValue,
}
pub struct OrmSelectColumn {
    pub column: OrmColumn,
    pub alias: String,
}
pub struct QueryFilterColumn {
    pub column: OrmColumn,
}

pub struct OrmOrderColumn{
    pub column: OrmColumn,
    pub order_type : OrderType
}
pub enum OrderType {
    Asc,
    Desc
}