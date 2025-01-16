use crate::core::abc::BmbpTree;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use bmbp_orm::{OrmSimpleSQLTrait, OrmTableTrait};

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(default)]
pub struct BmbpConfigVars {
    pub vars_code: String,
    pub vars_parent_code: String,
    pub vars_code_path: String,
    pub vars_name: String,
    pub vars_name_path: String,
    pub vars_children: Option<Vec<BmbpConfigVars>>,
    pub vars_alias: String,
    pub vars_value: String,
    pub vars_type: String,
    pub vars_tree_grade: i64,
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
#[derive(Default,Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VarsTypeEnum {
    #[default]
    SYSTEM,
    CUSTOM,
}

impl BmbpTree<BmbpConfigVars> for BmbpConfigVars {
    fn code(&self) -> String {
        self.vars_code.to_string()
    }

    fn parent_code(&self) -> String {
        self.vars_parent_code.to_string()
    }

    fn children(&self) -> Option<&Vec<BmbpConfigVars>>
    where
        Self: Sized,
    {
        self.vars_children.as_ref()
    }

    fn set_children(&mut self, children: Vec<Self>)
    where
        Self: Sized,
    {
        self.vars_children = Some(children);
    }
    fn node_order(&self) -> isize {
        self.data_order.clone() as isize
    }
}
impl OrmTableTrait for BmbpConfigVars {
    fn table_name() -> String {
        "bmbp_config_vars".to_string()
    }

    fn table_columns() -> Vec<String> {
        vec![
            "vars_code".to_string(),
            "vars_parent_code".to_string(),
            "vars_code_path".to_string(),
            "vars_name".to_string(),
            "vars_name_path".to_string(),
            "vars_alias".to_string(),
            "vars_value".to_string(),
            "vars_type".to_string(),
            "vars_tree_grade".to_string(),
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
impl OrmSimpleSQLTrait<BmbpConfigVars> for BmbpConfigVars {
    fn insert(&self) -> String {
        "".to_string()
    }

    fn update(&self) -> String {
        "".to_string()
    }
}
impl<'a> FromRow<'a, PgRow> for BmbpConfigVars {
    fn from_row(row: &'a PgRow) -> Result<Self, Error> {
        Ok(BmbpConfigVars {
            vars_code: row.try_get("vars_code")?,
            vars_parent_code: row.try_get("vars_parent_code")?,
            vars_code_path: row.try_get("vars_code_path")?,
            vars_name: row.try_get("vars_name")?,
            vars_name_path: row.try_get("vars_name_path")?,
            vars_children: None, // 忽略这一列
            vars_alias: row.try_get("vars_alias")?,
            vars_value: row.try_get("vars_value")?,
            vars_type: row.try_get("vars_type")?,
            vars_tree_grade: row.try_get("vars_tree_grade")?,
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