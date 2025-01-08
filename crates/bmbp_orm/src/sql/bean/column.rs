use crate::sql::bean::dql::OrmQuery;
use crate::sql::bean::func::OrmFunc;
use crate::sql::bean::table::OrmSchemaTable;
use crate::sql::bean::value::BValue;

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
    pub column: BValue,
}
pub struct OrmRawColumn {
    pub column: String,
}
pub struct OrmFuncColumn {
    pub func: OrmFunc,
}

pub struct OrmDmlColumn {
    pub column: OrmColumn,
    pub value: BValue,
}
pub struct OrmSelectColumn {
    pub column: OrmColumn,
    pub alias: String,
}
pub struct QueryFilterColumn {
    pub column: OrmColumn,
}
