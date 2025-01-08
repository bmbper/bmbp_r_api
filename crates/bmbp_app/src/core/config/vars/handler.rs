use salvo::{handler, Depot, Request, Response};
use crate::core::abc::{BmbpResp, PageData, RespVo};
use crate::core::config::vars::bean::BmbpConfigVars;

#[handler]
pub async fn tree(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Vec<BmbpConfigVars>>>  {
    Ok(RespVo::ok(vec![]))
}

#[handler]
pub async fn tree_ignore_node(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Vec<BmbpConfigVars>>>  {
    Ok(RespVo::ok(vec![]))
}

#[handler]
pub async fn page(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<PageData<BmbpConfigVars>>>  {
    Ok(RespVo::ok(PageData::default()))
}

#[handler]
pub async fn list(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Vec<BmbpConfigVars>>>  {
    Ok(RespVo::ok(vec![]))
}

#[handler]
pub async fn info(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpConfigVars>>>  {
    Ok(RespVo::ok(None))
}

#[handler]
pub async fn save(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpConfigVars>>>  {
    Ok(RespVo::ok(None))
}

#[handler]
pub async fn insert(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpConfigVars>>>  {
    Ok(RespVo::ok(None))
}
#[handler]
pub async fn update(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpConfigVars>>>  {
    Ok(RespVo::ok(None))
}

#[handler]
pub async fn enable(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn disable(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn delete(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn batch_enable(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn batch_disable(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn batch_delete(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn update_parent(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
    Ok(RespVo::ok(0usize))
}
