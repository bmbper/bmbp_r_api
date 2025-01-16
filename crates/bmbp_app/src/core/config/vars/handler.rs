use crate::core::abc::{BatchVo, BmbpResp, ComboVo, PageVo, RespVo};
use bmbp_orm::PageData;
use salvo::prelude::*;
use std::collections::HashMap;
use tracing_log::log::info;
use crate::core::config::vars::bean::BmbpConfigVars;
use crate::core::config::vars::service::BmbpVarsService;

#[handler]
pub async fn tree(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpConfigVars>>> {
    let dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_tree = BmbpVarsService::get_tree(&dict_vo).await?;
    Ok(RespVo::ok(dict_tree))
}

#[handler]
pub async fn tree_ignore_node(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpConfigVars>>> {
    let dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_tree = BmbpVarsService::get_tree_ignore_node(&dict_vo).await?;
    Ok(RespVo::ok(dict_tree))
}

#[handler]
pub async fn page(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<PageData<BmbpConfigVars>>> {
    let dict_vo = req
        .parse_body::<PageVo<BmbpConfigVars>>()
        .await
        .unwrap_or(PageVo::default());
    info!("page dict_vo:{:?}", dict_vo);
    let dict_page = BmbpVarsService::get_page(&dict_vo).await?;
    Ok(RespVo::ok(dict_page))
}

#[handler]
pub async fn list(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<Vec<BmbpConfigVars>>> {
    let dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_list = BmbpVarsService::get_list(&dict_vo).await?;
    Ok(RespVo::ok(dict_list))
}

#[handler]
pub async fn info(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpConfigVars>>> {
    let dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_info = BmbpVarsService::get_info(&dict_vo).await?;
    Ok(RespVo::ok(dict_info))
}

#[handler]
pub async fn save(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<Option<BmbpConfigVars>>> {
    let mut dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_info = BmbpVarsService::save(&mut dict_vo).await?;
    Ok(RespVo::ok(dict_info))
}

#[handler]
pub async fn insert(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<Option<String>>> {
    let mut dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let _ = BmbpVarsService::insert(&mut dict_vo).await?;
    Ok(RespVo::ok(Some(dict_vo.data_id.to_string())))
}
#[handler]
pub async fn update(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<Option<usize>>> {
    let mut dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_info = BmbpVarsService::update(&mut dict_vo).await?;
    Ok(RespVo::ok(Some(dict_info)))
}

#[handler]
pub async fn enable(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let mut dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_info = BmbpVarsService::enable(&mut dict_vo).await?;
    Ok(RespVo::ok(dict_info))
}
#[handler]
pub async fn disable(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let mut dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_info = BmbpVarsService::disable(&mut dict_vo).await?;
    Ok(RespVo::ok(dict_info))
}

#[handler]
pub async fn delete(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let mut dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_info = BmbpVarsService::delete(&mut dict_vo).await?;
    Ok(RespVo::ok(dict_info))
}

#[handler]
pub async fn batch_enable(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let batch_vo = req
        .parse_body::<BatchVo<String>>()
        .await
        .unwrap_or(BatchVo::default());
    let row_count = BmbpVarsService::batch_enable(&batch_vo).await?;
    Ok(RespVo::ok(row_count))
}
#[handler]
pub async fn batch_disable(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let batch_vo = req
        .parse_body::<BatchVo<String>>()
        .await
        .unwrap_or(BatchVo::default());
    let row_count = BmbpVarsService::batch_disable(&batch_vo).await?;
    Ok(RespVo::ok(row_count))
}
#[handler]
pub async fn batch_delete(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let batch_vo = req
        .parse_body::<BatchVo<String>>()
        .await
        .unwrap_or(BatchVo::default());
    let row_count = BmbpVarsService::batch_delete(&batch_vo).await?;
    Ok(RespVo::ok(row_count))
}

#[handler]
pub async fn update_parent(
    req: &mut Request,
    _depot: &mut Depot,
    _resp: &mut Response,
) -> BmbpResp<RespVo<usize>> {
    let mut dict_vo = req
        .parse_body::<BmbpConfigVars>()
        .await
        .unwrap_or(BmbpConfigVars::default());
    let dict_info = BmbpVarsService::update_parent(&mut dict_vo).await?;
    Ok(RespVo::ok(dict_info))
}