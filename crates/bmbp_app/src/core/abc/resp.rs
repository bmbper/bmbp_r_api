use crate::core::abc::error::BmbpErr;
use salvo::prelude::Text::Json;
use salvo::{async_trait, Depot, Request, Response, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RespVo<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> RespVo<T> {
    pub fn ok(data: T) -> RespVo<T> {
        RespVo {
            code: "0".to_string(),
            msg: "success".to_string(),
            data:Some(data),
        }
    }
    pub fn ok_msg(data: T, msg: String) -> RespVo<T> {
        RespVo {
            code: "0".to_string(),
            msg,
            data:Some(data),
        }
    }

    pub fn fail(msg: String) -> RespVo<T> {
        RespVo {
            code: "-1".to_string(),
            msg,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComboVo {
    pub code: String,
    pub label: String,
    pub children: Option<Vec<Self>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountVo<T> {
    pub code: String,
    pub label: String,
    pub count: u64,
    pub children: Option<Vec<T>>,
}

#[async_trait]
impl<T> Writer for RespVo<T>
where
    T: Serialize + Send + Sync,
{
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        res.render(serde_json::to_string(&self).unwrap())
    }
}
