use crate::core::abc::BmbpTree;
use serde::{Deserialize, Serialize};

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(default)]
pub struct BmbpSettingDict {
    pub dict_code: String,
    pub dict_parent_code: String,
    pub dict_code_path: String,
    pub dict_name: String,
    pub dict_name_path: String,
    pub dict_children: Option<Vec<BmbpSettingDict>>,
    pub dict_alias: String,
    pub dict_value: String,
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

impl BmbpTree<BmbpSettingDict> for BmbpSettingDict {
    fn code(&self) -> String {
        self.dict_code.to_string()
    }

    fn parent_code(&self) -> String {
        self.dict_parent_code.to_string()
    }

    fn children(&self) -> Option<&Vec<BmbpSettingDict>>
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
}
