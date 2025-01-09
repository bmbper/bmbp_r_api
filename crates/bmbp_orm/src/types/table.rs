pub trait OrmTableTrait {
    fn table_name() -> String;
    fn table_columns() -> Vec<String>;
    fn table_primary_key() -> String;
}

