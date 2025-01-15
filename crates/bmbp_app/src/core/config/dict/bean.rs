use crate::core::abc::BmbpTree;
use bmbp_orm::{OrmSimpleSQLTrait, OrmTableTrait};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpConfigDict {
    pub dict_code: String,
    pub dict_parent_code: String,
    pub dict_code_path: String,
    pub dict_name: String,
    pub dict_name_path: String,
    pub dict_children: Option<Vec<BmbpConfigDict>>,
    pub dict_alias: String,
    pub dict_value: String,
    pub dict_tree_grade: i64,
    pub data_id: String,
    pub data_flag: String,
    pub data_level: String,
    pub data_status: String,
    pub data_order: i64,
    pub data_create_time: String,
    pub data_update_time: String,
    pub data_create_user: String,
    pub data_update_user: String,
    pub data_owner_org: String,
    pub data_sign: String,
}
impl BmbpTree<BmbpConfigDict> for BmbpConfigDict {
    fn code(&self) -> String {
        self.dict_code.to_string()
    }

    fn parent_code(&self) -> String {
        self.dict_parent_code.to_string()
    }

    fn children(&self) -> Option<&Vec<BmbpConfigDict>>
    where
        Self: Sized,
    {
        self.dict_children.as_ref()
    }

    fn set_children(&mut self, children: Vec<Self>)
    where
        Self: Sized,
    {
        self.dict_children = Some(children);
    }

    fn node_order(&self) -> isize {
        self.data_order as isize
    }
}
impl OrmTableTrait for BmbpConfigDict {
    fn table_name() -> String {
        "bmbp_config_dict".to_string()
    }

    fn table_columns() -> Vec<String> {
        vec![
            "dict_code".to_string(),
            "dict_parent_code".to_string(),
            "dict_code_path".to_string(),
            "dict_name".to_string(),
            "dict_name_path".to_string(),
            "dict_alias".to_string(),
            "dict_value".to_string(),
            "dict_tree_grade".to_string(),
            "data_id".to_string(),
            "data_flag".to_string(),
            "data_level".to_string(),
            "data_status".to_string(),
            "data_order".to_string(),
            "data_create_time".to_string(),
            "data_update_time".to_string(),
            "data_create_user".to_string(),
            "data_update_user".to_string(),
            "data_owner_org".to_string(),
            "data_sign".to_string(),
        ]
    }

    fn table_primary_key() -> String {
        "data_id".to_string()
    }
}
impl OrmSimpleSQLTrait<BmbpConfigDict> for BmbpConfigDict {
    fn insert(&self) -> String {
        "".to_string()
    }

    fn update(&self) -> String {
        "".to_string()
    }
}
impl<'a> FromRow<'a, PgRow> for BmbpConfigDict {
    fn from_row(row: &'a PgRow) -> Result<Self, Error> {
        Ok(BmbpConfigDict {
            dict_code: row.try_get("dict_code")?,
            dict_parent_code: row.try_get("dict_parent_code")?,
            dict_code_path: row.try_get("dict_code_path")?,
            dict_name: row.try_get("dict_name")?,
            dict_name_path: row.try_get("dict_name_path")?,
            dict_children: None, // 忽略这一列
            dict_alias: row.try_get("dict_alias")?,
            dict_value: row.try_get("dict_value")?,
            dict_tree_grade: row.try_get("dict_tree_grade")?,
            data_id: row.try_get("data_id")?,
            data_flag: row.try_get("data_flag")?,
            data_level: row.try_get("data_level")?,
            data_status: row.try_get("data_status")?,
            data_order: row.try_get("data_order")?,
            data_create_time: row.try_get("data_create_time")?,
            data_update_time: row.try_get("data_update_time")?,
            data_create_user: row.try_get("data_create_user")?,
            data_update_user: row.try_get("data_update_user")?,
            data_owner_org: row.try_get("data_owner_org")?,
            data_sign: row.try_get("data_sign")?,
        })
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct DictQueryVo {
    pub(crate) dict_code: String,
    pub(crate) dict_codes: Vec<String>,
}
