use crate::core::abc::{BmbpErr, BmbpErrorKind, BmbpResp, BmbpTreeUtil, PageVo, RespVo};
use crate::core::config::dict::bean::BmbpConfigDict;
use crate::core::config::dict::handler::page;
use crate::orm::DB_POOL;
use bmbp_orm::OrmSimpleSQLTrait;
use bmbp_orm::{OrmTableTrait, PageData};
use sqlx::query::QueryAs;
use tracing_log::log::{debug, info};

pub struct BmbpDictService;

impl BmbpDictService {
    pub(crate) async fn get_tree(dict_vo: BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        let dict_list = BmbpDictService::get_list(dict_vo).await?;
        Ok(BmbpTreeUtil::build::<BmbpConfigDict>(dict_list))
    }
    pub(crate) async fn get_list(dict_vo: BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        debug!("查询SQL:{}", BmbpConfigDict::select().as_str());
        let dict_vec = sqlx::query_as(BmbpConfigDict::select().as_str())
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        return Ok(dict_vec);
    }
    pub(crate) async fn get_page(
        page_vo: PageVo<BmbpConfigDict>,
    ) -> BmbpResp<PageData<BmbpConfigDict>> {
        let mut page_no = page_vo.page_size.unwrap_or(1);
        let mut page_size = page_vo.page_no.unwrap_or(10);
        page_no = if page_no == 0 { 1 } else { page_no };
        page_size = if page_size == 0 { 10 } else { page_size };

        let mut page_data: PageData<BmbpConfigDict> = PageData::default();
        page_data.page_no = page_vo.page_no.unwrap_or(1);
        page_data.page_size = page_vo.page_size.unwrap_or(10);

        let query_sql = BmbpConfigDict::select();
        // 计算总数
        let count_sql = format!("SELECT COUNT(*) as COUNT FROM ({}) AS t", query_sql);
        debug!("分页查询-COUNT SQL:{}", BmbpConfigDict::select().as_str());
        let mut count_sqlx = sqlx::query_as::<_, (i64,)>(count_sql.as_str());
        let count: (i64,) = count_sqlx.fetch_one(&*DB_POOL.get().unwrap()).await?;
        page_data.total = count.0.clone() as u64;

        // 计算偏移量
        let offset = (page_no - 1) * page_size;
        let page_sql = format!("{} LIMIT {} OFFSET {}", query_sql, page_size, offset);

        debug!("分页查询-列表数据SQL:{}", BmbpConfigDict::select().as_str());
        // 执行分页查询
        let mut page_sqlx = sqlx::query_as(page_sql.as_str());
        let data: Vec<BmbpConfigDict> = page_sqlx.fetch_all(&*DB_POOL.get().unwrap()).await?;
        page_data.data = Some(data);
        Ok(page_data)
    }
    pub(crate) async fn get_info(dict_vo: &BmbpConfigDict) -> BmbpResp<Option<BmbpConfigDict>> {
        let dict_data_id = dict_vo.data_id.clone();
        let select_one_sql = BmbpConfigDict::select_by_id();
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(dict_data_id);
        let dict_data = select_one_sqlx.fetch_one(&*DB_POOL.get().unwrap()).await?;
        Ok(Some(dict_data))
    }
}
