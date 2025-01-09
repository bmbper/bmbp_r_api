use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PageData<T>
where
    T: Default + Debug + Serialize,
{
    pub total: u64,
    pub page_no: u64,
    pub page_size: u64,
    pub data: Option<Vec<T>>,
}
