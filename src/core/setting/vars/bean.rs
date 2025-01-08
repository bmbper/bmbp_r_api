use crate::core::abc::BmbpTree;
use serde::{Deserialize, Serialize};

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(default)]
pub struct BmbpSettingVars {
    pub vars_code: String,
    pub vars_parent_code: String,
    pub vars_code_path: String,
    pub vars_name: String,
    pub vars_name_path: String,
    pub vars_children: Option<Vec<BmbpSettingVars>>,
    pub vars_alias: String,
    pub vars_value: String,
    pub vars_type: VarsTypeEnum,
    pub data_id: String,
    pub data_flag: String,
    pub data_level: String,
    pub data_status: String,
    pub data_order: u64,
    pub data_create_time: String,
    pub data_update_time: String,
    pub data_create_user: String,
    pub data_update_user: String,
    pub data_owner_org: String,
    pub data_sign: String,
}

impl BmbpTree<BmbpSettingVars> for BmbpSettingVars {
    fn code(&self) -> String {
        self.vars_code.to_string()
    }

    fn parent_code(&self) -> String {
        self.vars_parent_code.to_string()
    }

    fn children(&self) -> Option<&Vec<BmbpSettingVars>>
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
}
#[derive(Default,Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VarsTypeEnum {
    #[default]
    SYSTEM,
    CUSTOM,
}
