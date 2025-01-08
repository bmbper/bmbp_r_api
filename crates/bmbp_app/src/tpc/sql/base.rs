
pub trait BmbpTableIdent {
    fn table_name() -> String;
    fn table_alias() -> String;
    fn table_column() -> Vec<impl BmbpColumnIdent>;
    fn table_primary() -> Vec<impl BmbpColumnIdent>;
}
pub trait BmbpColumnIdent {
    fn column_name() -> String;
}

pub enum BmbpSql {
    Query(BmbpQuerySql),
    Insert(BmbpInsertSql),
    Update(BmbpUpdateSql),
    Delete(BmbpDeleteSql),
}

pub struct BmbpQuerySql {}
pub struct BmbpInsertSql {}
pub struct BmbpUpdateSql {}
pub struct BmbpDeleteSql {}

pub enum BmbpTable {
    SchemaTable(BmbpSchemaTable),
    QueryTable(BmbpQueryTable),
    StringTable(BmbpStringTable)
}

pub struct BmbpSchemaTable {
    pub table_schema: String,
    pub table_name: String,
    pub table_alias: String,
}

pub struct BmbpQueryTable{
    table_name: BmbpQuerySql,
    table_alias: String,
}

pub struct BmbpStringTable {
    pub table_name: String,
    pub table_alias: String,
}


pub enum BmbpColumn{
    SchemaColumn(BmbpSchemaColumn),
    QueryColumn(BmbpQueryColumn),
    StaticColumn(BmbpStaticColumn),
    FuncColumn(BmbpFuncColumn)
}
pub struct BmbpSchemaColumn {
    pub column_name: String,
    pub column_alias: String,
}
pub struct BmbpQueryColumn {
    pub column_name: BmbpQuerySql,
    pub column_alias: String,
}
pub struct BmbpStaticColumn {
    pub column_name: String,
    pub column_alias: String,
}
pub struct BmbpFuncColumn {
    pub column_name: String,
    pub column_alias: String,
}


