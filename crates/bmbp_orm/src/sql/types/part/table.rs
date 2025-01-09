use crate::sql::types::OrmWhere;
use crate::sql::types::query::OrmQuery;

pub enum OrmTable{
    SchemaTable(OrmSchemaTable),
    SimpleTable(OrmSimpleTable),
    QueryTable(OrmQueryTable)
}

pub struct OrmSchemaTable{
    pub schema: String,
    pub name: String,
    pub alias: String,
}
pub struct OrmSimpleTable{
    pub name: String,
    pub alias: String,
}
pub struct OrmQueryTable{
    pub query: OrmQuery,
    pub alias: String,
}

pub struct OrmJoinTable{
    pub table: OrmTable,
    pub join_type: JoinType,
    pub filter: OrmWhere
}

pub enum JoinType{
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullJoin,
}