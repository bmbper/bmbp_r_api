use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageVo<T>
where
    T: Debug + Clone + Serialize + Default,
{
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub params: Option<T>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchVo<T>
where
    T: Debug + Clone + Serialize + Default,
{
    pub batch_vo: Vec<T>,
}
