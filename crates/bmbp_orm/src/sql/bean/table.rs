use crate::sql::bean::dql::OrmQuery;

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