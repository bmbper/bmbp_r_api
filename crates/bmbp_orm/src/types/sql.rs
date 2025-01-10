use crate::OrmTableTrait;

pub trait OrmQuerySqlTrait{

}
pub trait OrmInsertSqlTrait{

}

pub trait OrmUpdateSqlTrait{

}

pub trait OrmDeleteSqlTrait{

}

pub trait OrmCurdSqlTrait{

}

pub trait OrmSimpleSQLTrait<T>
where
    T: OrmTableTrait,
{
    fn insert(&self) -> String;
    fn update(&self) -> String;
    fn insert_all() -> String {
        let mut insert_sql = "".to_string();
        let mut insert_columns = vec![];
        let mut insert_values = vec![];
        for (index, col_name) in T::table_columns().iter().enumerate() {
            insert_columns.push(format!("{}", col_name));
            insert_values.push(format!("${}", index + 1));
        }
        insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            T::table_name(),
            insert_columns.join(","),
            insert_values.join(",")
        );
        return insert_sql;
    }
    fn update_all() -> String {
        let mut insert_sql = "".to_string();
        let mut set_columns = vec![];
        for (index, col_name) in T::table_columns().iter().enumerate() {
            set_columns.push(format!("{} = ${}", col_name, index + 1));
        }
        insert_sql = format!(
            "UPDATE  {} SET {} WHERE {} = ${} ",
            T::table_name(),
            set_columns.join(","),
            T::table_primary_key(),
            T::table_columns().len() + 1
        );
        return insert_sql;
    }
    fn select() -> String {
        format!(
            "SELECT {} FROM  {}  ",
            T::table_columns().join(","),
            T::table_name(),
        )
    }
    fn select_by_id() -> String {
        format!(
            "SELECT {} FROM  {} WHERE {} = $1 ",
            T::table_columns().join(","),
            T::table_name(),
            T::table_primary_key()
        )
    }
    fn enable_by_id() -> String {
        format!(
            "UPDATE {} SET DATA_STATUS = '1' WHERE {} = $1 ",
            T::table_name(),
            T::table_primary_key(),
        )
    }
    fn disable_by_id() -> String {
        format!(
            "UPDATE {} SET DATA_STATUS = '0' WHERE {} = $1 ",
            T::table_name(),
            T::table_primary_key(),
        )
    }
    fn delete_by_id() -> String {
        let mut delete_sql = "".to_string();
        delete_sql = format!(
            "DELETE FROM {} WHERE {} = $1 ",
            T::table_name(),
            T::table_primary_key()
        );
        return delete_sql;
    }
}
