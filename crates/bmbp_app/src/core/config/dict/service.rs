use crate::core::abc::{
    now_date_time, simple_id, BmbpErr, BmbpErrorKind, BmbpResp, BmbpTreeUtil, PageVo, RespVo,
    DATA_FLAG, DATA_LEVEL, DATA_STATUS, TREE_PATH_SPLIT, TREE_ROOT_CODE,
};
use crate::core::config::dict::bean::BmbpConfigDict;
use crate::core::config::dict::handler::{insert, page};
use crate::orm::DB_POOL;
use bmbp_orm::OrmSimpleSQLTrait;
use bmbp_orm::{OrmTableTrait, PageData};
use sqlx::encode::IsNull::No;
use sqlx::query::{Query, QueryAs};
use tracing_log::log::{debug, info};

pub struct BmbpDictService;

impl BmbpDictService {
    pub(crate) async fn get_tree(dict_vo: &BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        let dict_list = BmbpDictService::get_list(&dict_vo).await?;
        Ok(BmbpTreeUtil::build::<BmbpConfigDict>(dict_list))
    }
    pub(crate) async fn get_list(dict_vo: &BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        let mut query_list_sql = BmbpConfigDict::select();
        query_list_sql = format!(
            "{} ORDER BY DICT_PARENT_CODE ASC,DICT_ORDER ASC",
            query_list_sql
        );
        debug!("查询SQL:{}", query_list_sql);

        let dict_vec = sqlx::query_as(query_list_sql.as_str())
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        return Ok(dict_vec);
    }
    pub(crate) async fn get_page(
        page_vo: &PageVo<BmbpConfigDict>,
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
        let dict_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        Ok(dict_data)
    }

    pub(crate) async fn get_info_by_code(dict_code: String) -> BmbpResp<Option<BmbpConfigDict>> {
        if dict_code.is_empty() {
            return Err(BmbpErr::valid("字典编码不能为空".to_string()));
        }
        let mut select_one_sql = BmbpConfigDict::select();
        select_one_sql = format!("{} WHERE DICT_CODE = $1", select_one_sql);
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(dict_code);
        let dict_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        Ok(dict_data)
    }
    pub(crate) async fn save(dict: &mut BmbpConfigDict) -> BmbpResp<Option<BmbpConfigDict>> {
        let exist_dict = Self::get_info(dict).await?;
        if exist_dict.is_none() {
            Self::insert(dict).await?;
        } else {
            Self::update(dict).await?;
        }
        Self::get_info(dict).await
    }

    pub(crate) async fn insert(dict: &mut BmbpConfigDict) -> BmbpResp<usize> {
        if dict.data_id.is_empty() {
            dict.data_id = simple_id();
        }
        if dict.dict_code.is_empty() {
            dict.dict_code = dict.data_id.clone();
        }

        if dict.dict_name.is_empty() {
            return Err(BmbpErr::valid("字典名称不能为空".to_string()));
        }
        if dict.dict_alias.is_empty() {
            return Err(BmbpErr::valid("字典别名不能为空".to_string()));
        }
        if dict.dict_value.is_empty() {
            return Err(BmbpErr::valid("字典值不能为空".to_string()));
        }

        if dict.dict_parent_code.is_empty() {
            dict.dict_parent_code = TREE_ROOT_CODE.to_string();
            dict.dict_code_path = format!(
                "{}{}{}{}",
                TREE_ROOT_CODE, TREE_PATH_SPLIT, dict.dict_code, TREE_PATH_SPLIT
            );
            dict.dict_name_path = format!(
                "{}{}{}{}",
                TREE_ROOT_CODE, TREE_PATH_SPLIT, dict.dict_name, TREE_PATH_SPLIT
            );
        } else {
            let parent_dict = Self::get_info_by_code(dict.dict_parent_code.clone()).await?;
            if parent_dict.is_none() {
                return Err(BmbpErr::valid("父级字典不存在".to_string()));
            }
            let parent_dict = parent_dict.unwrap();
            dict.dict_code_path = format!(
                "{}{}{}",
                parent_dict.dict_code_path, dict.dict_code, TREE_PATH_SPLIT
            );
            dict.dict_name_path = format!(
                "{}{}{}",
                parent_dict.dict_name_path, dict.dict_name, TREE_PATH_SPLIT
            );
        }
        // check same name
        Self::check_same_name(&dict.dict_name, &dict.dict_parent_code, &dict.data_id).await?;
        // check same value
        Self::check_same_value(&dict.dict_value, &dict.dict_parent_code, &dict.data_id).await?;
        // check same alias
        Self::check_same_alias(&dict.dict_alias, &dict.data_id).await?;

        // set default value
        dict.data_flag = DATA_FLAG.to_string();
        dict.data_level = DATA_LEVEL.to_string();
        dict.data_status = DATA_STATUS.to_string();
        dict.data_create_time = now_date_time();
        dict.data_update_time = now_date_time();
        dict.data_owner_org = "".to_string();
        dict.data_sign = "".to_string();
        dict.dict_tree_grade = (dict.dict_code_path.split(TREE_PATH_SPLIT).count() as i64) - 2;

        let mut insert_sql = BmbpConfigDict::insert_all();
        info!("插入SQL:{}", insert_sql);
        let mut insert_sqlx = sqlx::query(insert_sql.as_str())
            .bind(&dict.dict_code)
            .bind(&dict.dict_parent_code)
            .bind(&dict.dict_code_path)
            .bind(&dict.dict_name)
            .bind(&dict.dict_name_path)
            .bind(&dict.dict_alias)
            .bind(&dict.dict_value)
            .bind(&dict.dict_tree_grade)
            .bind(&dict.data_id)
            .bind(&dict.data_flag)
            .bind(&dict.data_level)
            .bind(&dict.data_status)
            .bind(&dict.data_order)
            .bind(&dict.data_create_time)
            .bind(&dict.data_update_time)
            .bind(&dict.data_create_user)
            .bind(&dict.data_update_user)
            .bind(&dict.data_owner_org)
            .bind(&dict.data_sign);
        insert_sqlx.execute(&*DB_POOL.get().unwrap()).await?;

        Ok(0usize)
    }
    pub(crate) async fn update(dict: &mut BmbpConfigDict) -> BmbpResp<usize> {
        if dict.data_id.is_empty() {
            return Err(BmbpErr::valid("字典ID不能为空".to_string()));
        }
        Ok(0)
    }

    async fn check_same_name(
        dict_name: &String,
        dict_parent_code: &String,
        data_id: &String,
    ) -> BmbpResp<()> {
        let mut select_one_sql = BmbpConfigDict::select();
        select_one_sql = format!(
            "{} WHERE DICT_NAME = $1 AND DICT_PARENT_CODE = $2 AND DATA_ID != $3",
            select_one_sql
        );
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str())
                .bind(dict_name)
                .bind(dict_parent_code)
                .bind(data_id);
        let dict_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        if dict_data.is_some() {
            return Err(BmbpErr::valid("字典名称重复".to_string()));
        }
        Ok(())
    }

    async fn check_same_value(
        dict_value: &String,
        dict_parent_code: &String,
        data_id: &String,
    ) -> BmbpResp<()> {
        let mut select_one_sql = BmbpConfigDict::select();
        select_one_sql = format!(
            "{} WHERE DICT_VALUE = $1 AND DICT_PARENT_CODE = $2 AND DATA_ID != $3",
            select_one_sql
        );
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str())
                .bind(dict_value)
                .bind(dict_parent_code)
                .bind(data_id);
        let dict_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        if dict_data.is_some() {
            return Err(BmbpErr::valid("字典值重复".to_string()));
        }
        Ok(())
    }

    async fn check_same_alias(dict_alias: &String, data_id: &String) -> BmbpResp<()> {
        let mut select_one_sql = BmbpConfigDict::select();
        select_one_sql = format!("{} WHERE DICT_ALIAS = $1 AND DATA_ID != $2", select_one_sql);
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str())
                .bind(dict_alias)
                .bind(data_id);
        let dict_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        if dict_data.is_some() {
            return Err(BmbpErr::valid("字典别名重复".to_string()));
        }
        Ok(())
    }
}
