use tracing_log::log::info;
use crate::core::abc::{BmbpResp, BmbpTreeUtil};
use crate::core::config::dict::bean::BmbpConfigDict;
use crate::orm::{DB_POOL};
use bmbp_orm::BmbpTable;
use bmbp_orm::BmbpTableSQL;

pub struct BmbpDictService;

impl BmbpDictService {
    pub(crate) async fn get_tree(dict_vo: BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        let dict_list = BmbpDictService::get_list(dict_vo).await?;
        Ok(BmbpTreeUtil::build::<BmbpConfigDict>(dict_list))
    }
    pub(crate) async fn get_list(dict_vo: BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        info!("查询SQL:{}", BmbpConfigDict::select().as_str());
        let dict_vec = sqlx::query_as(BmbpConfigDict::select().as_str())
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        return Ok(dict_vec);
    }
}
