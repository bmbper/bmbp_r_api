use crate::core::abc::{BmbpResp,  PageVo, RespVo};
use crate::core::config::dict::bean::BmbpConfigDict;
use salvo::prelude::*;
use bmbp_orm::PageData;
use crate::core::config::dict::service::BmbpDictService;

#[handler]
pub async fn tree(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpConfigDict>>> {
    let dict_vo = req
        .parse_body::<BmbpConfigDict>()
        .await
        .unwrap_or(BmbpConfigDict::default());
    let dict_tree = BmbpDictService::get_tree(dict_vo).await?;
    Ok(RespVo::ok(dict_tree))
}

#[handler]
pub async fn tree_ignore_node(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpConfigDict>>> {
    Ok(RespVo::ok(vec![]))
}

#[handler]
pub async fn page(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<PageData<BmbpConfigDict>>> {
    let dict_vo = req
        .parse_body::<PageVo<BmbpConfigDict>>()
        .await
        .unwrap_or(PageVo::default());
    let dict_page = BmbpDictService::get_page(dict_vo).await?;
    Ok(RespVo::ok(dict_page))
}

#[handler]
pub async fn list(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpConfigDict>>> {
    Ok(RespVo::ok(vec![]))
}

#[handler]
pub async fn info(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpConfigDict>>> {
    let dict_vo = req
        .parse_body::<BmbpConfigDict>()
        .await
        .unwrap_or(BmbpConfigDict::default());
    let dict_info = BmbpDictService::get_info(&dict_vo).await?;
    Ok(RespVo::ok(dict_info))
}

#[handler]
pub async fn save(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpConfigDict>>> {
    Ok(RespVo::ok(None))
}

#[handler]
pub async fn insert(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpConfigDict>>> {
    Ok(RespVo::ok(None))
}
#[handler]
pub async fn update(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpConfigDict>>> {
    Ok(RespVo::ok(None))
}

#[handler]
pub async fn enable(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn disable(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn delete(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn batch_enable(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn batch_disable(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn batch_delete(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn update_parent(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn combo(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn combos(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn combo_tree(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn combos_tree(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn display(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}

#[handler]
pub async fn displays(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn display_tree(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
#[handler]
pub async fn displays_tree(
    req: &mut Request,
    depot: &mut Depot,
    rep: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    Ok(RespVo::ok(0usize))
}
