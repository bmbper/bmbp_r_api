use salvo::prelude::*;
use crate::core::abc::{BmbpResp, PageData, RespVo};
use crate::core::setting::dict::bean::BmbpSettingDict;

#[handler]
pub async fn tree(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Vec<BmbpSettingDict>>>  {
   Ok(RespVo::ok(vec![BmbpSettingDict::default()]))
}

#[handler]
pub async fn tree_ignore_node(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Vec<BmbpSettingDict>>>  {
   Ok(RespVo::ok(vec![]))
}

#[handler]
pub async fn page(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<PageData<BmbpSettingDict>>>  {
   Ok(RespVo::ok(PageData::default()))
}

#[handler]
pub async fn list(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Vec<BmbpSettingDict>>>  {
   Ok(RespVo::ok(vec![]))
}

#[handler]
pub async fn info(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpSettingDict>>>  {
   Ok(RespVo::ok(None))
}

#[handler]
pub async fn save(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpSettingDict>>>  {
   Ok(RespVo::ok(None))
}

#[handler]
pub async fn insert(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpSettingDict>>>  {
   Ok(RespVo::ok(None))
}
#[handler]
pub async fn update(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<Option<BmbpSettingDict>>>  {
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

#[handler]
pub async fn combo(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}


#[handler]
pub async fn combos(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn combo_tree(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn combos_tree(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn display(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn displays(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn display_tree(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn displays_tree(req:&mut Request,depot: &mut Depot,rep:&mut Response) -> BmbpResp<RespVo<usize>>  {
   Ok(RespVo::ok(0usize))
}