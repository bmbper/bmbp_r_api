use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RespVo<T> {
    pub code: String,
    pub msg: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageData<T> {
    pub total: u64,
    pub page_no: u64,
    pub page_size: u64,
    pub data: Option<Vec<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComboVo<T> {
    pub code: String,
    pub label: String,
    pub children: Option<Vec<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountVo<T> {
    pub code: String,
    pub label: String,
    pub count: u64,
    pub children: Option<Vec<T>>,
}
