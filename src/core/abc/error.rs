use crate::core::abc::RespVo;
use salvo::{async_trait, Depot, Request, Response, Writer};

pub struct BmbpErr {
    pub kind: BmbpErrorKind,
    pub msg: String,
    pub code: String,
}

pub enum BmbpErrorKind {
    HTTP,
    VALID,
    DB,
    SQL,
    OTHER,
}

pub type BmbpResp<T> = Result<T, BmbpErr>;

#[async_trait]
impl Writer for BmbpErr {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let resp: RespVo<String> = RespVo {
            code: "-1".to_string(),
            msg: self.msg,
            data: None,
        };
        res.render(serde_json::to_string(&resp).unwrap());
    }
}
