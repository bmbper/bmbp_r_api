use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageVo<T>
where
    T: Debug + Clone + Serialize,
{
    pub page_no: Option<i64>,
    pub page_size: Option<i64>,
    pub params: Option<T>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchVo<T>
where
    T: Debug + Clone + Serialize,
{
    pub batch_vo: Vec<T>,
}
