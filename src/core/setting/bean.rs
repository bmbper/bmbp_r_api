use crate::core::abc::BmbpTree;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpSettingDict {
    pub code: String,
    pub parent_code: String,
    pub code_path: String,
    pub name: String,
    pub name_path: String,
    pub children: Option<Vec<BmbpSettingDict>>,
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
        self.code.to_string()
    }

    fn parent_code(&self) -> String {
        self.parent_code.to_string()
    }

    fn children(&self) -> Option<&Vec<BmbpSettingDict>>
    where
        Self: Sized,
    {
        self.children.as_ref()
    }

    fn set_children(&mut self, children: Vec<Self>)
    where
        Self: Sized,
    {
        self.children = Some(children);
    }
}


#[cfg(test)]
mod test {
    use crate::core::abc::BmbpTree;
    use crate::core::setting::bean::BmbpSettingDict;

    #[test]
    pub fn test_dict() {
        let mut dict_parent = BmbpSettingDict {
            code: "1".to_string(),
            parent_code: "0".to_string(),
            code_path: "1".to_string(),
            name: "1".to_string(),
            name_path: "1".to_string(),
            children: None,
            data_id: "1".to_string(),
            data_flag: "1".to_string(),
            data_level: "1".to_string(),
            data_status: "1".to_string(),
            data_order: 1,
            data_create_time: "1".to_string(),
            data_update_time: "1".to_string(),
            data_create_user: "1".to_string(),
            data_update_user: "1".to_string(),
            data_owner_org: "1".to_string(),
            data_sign: "1".to_string(),
        };
        let dict_child = BmbpSettingDict {
            code: "1".to_string(),
            parent_code: "1".to_string(),
            code_path: "1".to_string(),
            name: "1".to_string(),
            name_path: "1".to_string(),
            children: None,
            data_id: "1".to_string(),
            data_flag: "1".to_string(),
            data_level: "1".to_string(),
            data_status: "1".to_string(),
            data_order: 1,
            data_create_time: "1".to_string(),
            data_update_time: "1".to_string(),
            data_create_user: "1".to_string(),
            data_update_user: "1".to_string(),
            data_owner_org: "1".to_string(),
            data_sign: "1".to_string(),
        };
        dict_parent.set_children(vec![dict_child]);
        println!("{:?}", dict_parent);
    }
}