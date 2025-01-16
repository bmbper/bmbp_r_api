use crate::core::abc::{
    now_date_time, simple_id, BatchVo, BmbpErr, BmbpErrorKind, BmbpResp, BmbpTree, BmbpTreeUtil,
    ComboVo, PageVo, RespVo, DATA_DISABLE, DATA_ENABLE, DATA_FLAG, DATA_LEVEL, DATA_STATUS,
    TREE_PATH_SPLIT, TREE_ROOT_CODE,
};
use crate::core::config::dict::bean::{BmbpConfigDict, DictQueryVo};
use crate::core::config::dict::handler::{insert, page};
use crate::orm::DB_POOL;
use bmbp_orm::OrmSimpleSQLTrait;
use bmbp_orm::{OrmTableTrait, PageData};
use sqlx::encode::IsNull::No;
use sqlx::query::{Query, QueryAs};
use std::collections::HashMap;
use tracing_log::log::{debug, info};

pub struct BmbpDictService;

impl BmbpDictService {
    pub(crate) async fn get_tree(dict_vo: &BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        let dict_list = BmbpDictService::get_list(&dict_vo).await?;
        Ok(BmbpTreeUtil::build::<BmbpConfigDict>(dict_list))
    }
    pub(crate) async fn get_tree_ignore_node(
        dict_vo: &BmbpConfigDict,
    ) -> BmbpResp<Vec<BmbpConfigDict>> {
        let mut query_list_sql = BmbpConfigDict::select();
        let mut dict_code_path = {
            if !dict_vo.data_id.is_empty() {
                if let Some(old_dict) = Self::get_info(dict_vo).await? {
                    old_dict.dict_code_path.clone()
                } else {
                    "".to_string()
                }
            } else if !dict_vo.dict_code.is_empty() {
                if let Some(old_dict) = Self::get_info_by_code(dict_vo.dict_code.clone()).await? {
                    old_dict.dict_code_path.clone()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        };
        query_list_sql = format!(
            "{} WHERE DICT_CODE_PATH NOT LIKE CONCAT($1,'%')",
            query_list_sql
        );

        query_list_sql = format!(
            "{} ORDER BY DICT_PARENT_CODE ASC,DATA_ORDER ASC",
            query_list_sql
        );
        debug!("查询SQL:{}", query_list_sql);

        let dict_vec = sqlx::query_as(query_list_sql.as_str())
            .bind(dict_code_path)
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        Ok(BmbpTreeUtil::build::<BmbpConfigDict>(dict_vec))
    }
    pub(crate) async fn get_list(dict_vo: &BmbpConfigDict) -> BmbpResp<Vec<BmbpConfigDict>> {
        let mut query_list_sql = BmbpConfigDict::select();
        query_list_sql = format!(
            "{} ORDER BY DICT_PARENT_CODE ASC,DATA_ORDER ASC",
            query_list_sql
        );
        debug!("查询SQL:{}", query_list_sql);

        let dict_vec = sqlx::query_as(query_list_sql.as_str())
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        return Ok(dict_vec);
    }
    pub(crate) async fn get_list_by_ids(dict_id_vec: &[String]) -> BmbpResp<Vec<BmbpConfigDict>> {
        let mut query_list_sql = BmbpConfigDict::select();
        // 构建 IN 子句的占位符
        let placeholders: Vec<String> =
            (1..=dict_id_vec.len()).map(|i| format!("${}", i)).collect();
        let in_clause = placeholders.join(",");
        // 构建 SQL 语句
        query_list_sql = format!("{} WHERE DATA_ID IN ({})", query_list_sql, in_clause);
        let mut  sqlx_query: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(query_list_sql.as_str());
        for dict_data_id in dict_id_vec {
            sqlx_query = sqlx_query.bind(dict_data_id);
        }

        let dict_vec = sqlx_query
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        return Ok(dict_vec);
    }
    async fn get_list_by_parent_code(dict_code: &String) -> BmbpResp<Vec<BmbpConfigDict>> {
        let mut select_one_sql = BmbpConfigDict::select();
        select_one_sql = format!("{} WHERE DICT_PARENT_CODE = $1", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(dict_code);
        let dict_data = select_one_sqlx.fetch_all(&*DB_POOL.get().unwrap()).await?;
        Ok(dict_data)
    }
    async fn get_list_by_parent_code_path(
        dict_code_path: &String,
    ) -> BmbpResp<Vec<BmbpConfigDict>> {
        let mut select_one_sql = BmbpConfigDict::select();
        select_one_sql = format!(
            "{} WHERE DICT_CODE_PATH LIKE CONCAT($1,'%') AND DICT_CODE_PATH != $2 ",
            select_one_sql
        );
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str())
                .bind(dict_code_path)
                .bind(dict_code_path);
        let dict_data = select_one_sqlx.fetch_all(&*DB_POOL.get().unwrap()).await?;
        Ok(dict_data)
    }
    pub(crate) async fn get_page(
        page_vo: &PageVo<BmbpConfigDict>,
    ) -> BmbpResp<PageData<BmbpConfigDict>> {
        let mut page_no = page_vo.page_num.unwrap_or(1);
        let mut page_size = page_vo.page_size.unwrap_or(10);
        page_no = if page_no == 0 { 1 } else { page_no };
        page_size = if page_size == 0 { 10 } else { page_size };

        let mut page_data: PageData<BmbpConfigDict> = PageData::default();
        page_data.page_num = page_vo.page_num.unwrap_or(1);
        page_data.page_size = page_vo.page_size.unwrap_or(10);

        let mut query_sql = BmbpConfigDict::select();
        let mut condition_sql = vec![];
        if let Some(dict_vo) = page_vo.params.as_ref() {
            if !dict_vo.dict_code.is_empty() {
                condition_sql.push(format!("DICT_CODE_PATH LIKE CONCAT('%/{}/%')",dict_vo.dict_code));
            }
            if !dict_vo.dict_code_path.is_empty() {
                condition_sql.push(format!("DICT_CODE_PATH LIKE CONCAT('{}%')",dict_vo.dict_code_path));
            }
            if !dict_vo.dict_name.is_empty() {
                condition_sql.push(format!("DICT_NAME LIKE CONCAT('%{}%')",dict_vo.dict_name));
            }
            if !dict_vo.dict_parent_code.is_empty() {
                condition_sql.push(format!("DICT_CODE_PATH LIKE CONCAT('%{}%')",dict_vo.dict_parent_code));
            }
            if !dict_vo.dict_alias.is_empty() {
                condition_sql.push(format!("DICT_ALIAS LIKE CONCAT('%{}%')",dict_vo.dict_alias));
            }
        }
        if !condition_sql.is_empty() {
            let condition_sql = condition_sql.join(" AND ");
            query_sql = format!("{} WHERE {}", query_sql,condition_sql);
        }
        // 字段排序
        query_sql = format!("{} ORDER BY DICT_TREE_GRADE ASC,DATA_ORDER ASC,DICT_PARENT_CODE ASC", query_sql);
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
        Self::get_info_by_id(&dict_vo.data_id).await
    }

    pub(crate) async fn get_info_by_id(dict_id: &String) -> BmbpResp<Option<BmbpConfigDict>> {
        let select_one_sql = BmbpConfigDict::select_by_id();
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(dict_id);
        let dict_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        Ok(dict_data)
    }
    pub(crate) async fn get_info_by_alias(dict_alias: &String) -> BmbpResp<Option<BmbpConfigDict>> {
        let select_one_sql = BmbpConfigDict::select();
        let select_one_sql = format!("{} WHERE DICT_ALIAS = $1", select_one_sql);
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigDict, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(dict_alias);
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

        if dict.dict_parent_code.is_empty() || dict.dict_parent_code.as_str() == TREE_ROOT_CODE {
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
        let old_dict = Self::get_info(dict).await?;
        if old_dict.is_none() {
            return Err(BmbpErr::valid("待更新的字典不存在".to_string()));
        }
        let mut old_dict = old_dict.unwrap();
        let old_dict_code_path = old_dict.dict_code_path.clone();
        let old_dict_name_path = old_dict.dict_name_path.clone();

        if dict.dict_name.is_empty() {
            dict.dict_name = old_dict.dict_name.clone();
        }
        if dict.dict_alias.is_empty() {
            dict.dict_alias = old_dict.dict_alias.clone();
        }
        if dict.dict_value.is_empty() {
            dict.dict_value = old_dict.dict_value.clone();
        }
        if dict.dict_code.is_empty() {
            dict.dict_code = old_dict.dict_code.clone();
        }
        if dict.dict_parent_code.is_empty() || dict.dict_parent_code.as_str() == TREE_ROOT_CODE {
            dict.dict_code = old_dict.dict_code.clone();
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
        dict.data_flag = old_dict.data_flag.clone();
        dict.data_level = old_dict.data_level.to_string();
        dict.data_status = old_dict.data_status.to_string();
        dict.data_create_time = old_dict.data_create_time.clone();
        dict.data_update_time = now_date_time();
        dict.data_create_user = "".to_string();
        dict.data_update_user = "".to_string();
        dict.data_owner_org = "".to_string();
        dict.data_sign = "".to_string();
        dict.dict_tree_grade = (dict.dict_code_path.split(TREE_PATH_SPLIT).count() as i64) - 2;

        let new_code_path = dict.dict_code_path.clone();
        let new_name_path = dict.dict_name_path.clone();

        // 开始事务
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        // 更新当前记当
        let mut update_sql = BmbpConfigDict::update_all();
        let update_query = sqlx::query(update_sql.as_str())
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
            .bind(&dict.data_sign)
            .bind(&dict.data_id);
        update_query.execute(&mut *tx).await?;

        // 更新子级
        let mut update_child_sql =format!(" UPDATE {} SET DICT_CODE_PATH = REPLACE(DICT_CODE_PATH, $1, $2), DICT_NAME_PATH = REPLACE(DICT_NAME_PATH, $3, $4) WHERE DICT_CODE_PATH LIKE CONCAT($5,'%')",  BmbpConfigDict::table_name());
        let update_child_query = sqlx::query(update_child_sql.as_str())
            .bind(&old_dict_code_path)
            .bind(&new_code_path)
            .bind(&old_dict_name_path)
            .bind(&new_name_path)
            .bind(&old_dict_code_path);
        update_child_query.execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(0usize)
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
    pub(crate) async fn enable(dict_vo: &mut BmbpConfigDict) -> BmbpResp<usize> {
        if dict_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待启用的字典".to_string()));
        }
        let dict_info = Self::get_info_by_id(&dict_vo.data_id).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let dict_code_path = dict_info.as_ref().unwrap().dict_code_path.clone();
        let dict_code_vec = dict_code_path.split(TREE_PATH_SPLIT).collect::<Vec<_>>();
        // 构建 IN 子句的占位符
        let placeholders: Vec<String> = (1..=dict_code_vec.len())
            .map(|i| format!("${}", i))
            .collect();
        let in_clause = placeholders.join(",");
        // 构建 SQL 语句
        let update_sql = format!(
            "UPDATE {} SET DATA_STATUS = ${} WHERE dict_code IN ({})",
            BmbpConfigDict::table_name(),
            dict_code_vec.len() + 1, // new_status 是第一个参数
            in_clause
        );
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        let mut update_query = sqlx::query(&update_sql).bind(DATA_ENABLE);
        for code in dict_code_vec.iter() {
            update_query = update_query.bind(code);
        }
        let result = update_query.execute(&mut *tx).await;
        if result.is_err() {
            tx.rollback().await?;
            return Err(BmbpErr::valid("启用字典失败".to_string()));
        }
        tx.commit().await?;
        Ok(result?.rows_affected() as usize)
    }
    pub(crate) async fn disable(dict_vo: &mut BmbpConfigDict) -> BmbpResp<usize> {
        if dict_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待启用的字典".to_string()));
        }
        let dict_info = Self::get_info_by_id(&dict_vo.data_id).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let dict_code_path = dict_info.as_ref().unwrap().dict_code_path.clone();
        tracing::info!("禁用字典批量:{}", dict_code_path);
        // 构建 SQL 语句
        let update_sql = format!(
            "UPDATE {} SET DATA_STATUS = $1 WHERE DICT_CODE_PATH LIKE CONCAT($2,'%')",
            BmbpConfigDict::table_name()
        );
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        let mut update_query = sqlx::query(&update_sql)
            .bind(DATA_DISABLE)
            .bind(dict_code_path);
        let result = update_query.execute(&mut *tx).await;
        if result.is_err() {
            tx.rollback().await?;
            return Err(BmbpErr::valid("停用字典失败".to_string()));
        }
        tx.commit().await?;
        Ok(result?.rows_affected() as usize)
    }

    pub(crate) async fn delete(dict_vo: &mut BmbpConfigDict) -> BmbpResp<usize> {
        if dict_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待删除的字典".to_string()));
        }
        let mut dict_info = Self::get_info_by_id(&dict_vo.data_id).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let dict_vo = dict_info.unwrap();
        let dict_vec = Self::get_list_by_parent_code(&dict_vo.dict_code).await?;
        if !dict_vec.is_empty() {
            return Err(BmbpErr::valid("请先删除子字典".to_string()));
        };
        let delete_sql = BmbpConfigDict::delete_by_id();
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        let result = sqlx::query(&delete_sql)
            .bind(&dict_vo.data_id)
            .execute(&mut *tx)
            .await;
        if result.is_err() {
            tx.rollback().await?;
            return Err(BmbpErr::valid("删除字典失败".to_string()));
        }
        tx.commit().await?;
        Ok(result?.rows_affected() as usize)
    }

    pub(crate) async fn batch_enable(batch_vo: &BatchVo<String>) -> BmbpResp<usize> {
        let data_id_vec = batch_vo.batch_vo.as_slice();
        if data_id_vec.is_empty() {
            return Err(BmbpErr::valid("请指定待启用的字典".to_string()));
        }
        let dict_vec = Self::get_list_by_ids(data_id_vec).await?;
        if dict_vec.is_empty() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let mut rows_affected = 0;
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        for dict in dict_vec.iter() {
            let dict_code_path = dict.dict_code_path.clone();
            let dict_code_vec = dict_code_path.split(TREE_PATH_SPLIT).collect::<Vec<_>>();
            // 构建 IN 子句的占位符
            let placeholders: Vec<String> = (1..=dict_code_vec.len())
                .map(|i| format!("${}", i))
                .collect();
            let in_clause = placeholders.join(",");
            // 构建 SQL 语句
            let update_sql = format!(
                "UPDATE {} SET DATA_STATUS = ${} WHERE dict_code IN ({})",
                BmbpConfigDict::table_name(),
                dict_code_vec.len() + 1, // new_status 是第一个参数
                in_clause
            );

            let mut update_query = sqlx::query(&update_sql);
            for code in dict_code_vec.iter() {
                update_query = update_query.bind(code);
            }
            update_query = update_query.bind(DATA_ENABLE);
            let result = update_query.execute(&mut *tx).await;
            if result.is_err() {
                tx.rollback().await?;
                return Err(BmbpErr::valid("启用字典失败".to_string()));
            }
            rows_affected += result?.rows_affected() as usize;
        }
        tx.commit().await?;
        Ok(rows_affected)
    }
    pub(crate) async fn batch_disable(batch_vo: &BatchVo<String>) -> BmbpResp<usize> {
        let data_id_vec = batch_vo.batch_vo.as_slice();
        if data_id_vec.is_empty() {
            return Err(BmbpErr::valid("请指定待停用的字典".to_string()));
        }
        let dict_vec = Self::get_list_by_ids(data_id_vec).await?;
        if dict_vec.is_empty() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let mut rows_affected = 0;
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        for dict in dict_vec.iter() {
            let dict_code_path = dict.dict_code_path.clone();
            // 构建 SQL 语句
            let update_sql = format!(
                "UPDATE {} SET DATA_STATUS = $1 WHERE DICT_CODE_PATH LIKE CONCAT($2,'%')",
                BmbpConfigDict::table_name()
            );
            let mut update_query = sqlx::query(&update_sql)
                .bind(DATA_DISABLE)
                .bind(dict_code_path);
            let result = update_query.execute(&mut *tx).await;
            if result.is_err() {
                tx.rollback().await?;
                return Err(BmbpErr::valid("停用字典失败".to_string()));
            }
            rows_affected += result?.rows_affected() as usize;
        }
        tx.commit().await?;
        Ok(rows_affected)
    }
    pub(crate) async fn batch_delete(batch_vo: &BatchVo<String>) -> BmbpResp<usize> {
        let data_id_vec = batch_vo.batch_vo.as_slice();
        if data_id_vec.is_empty() {
            return Err(BmbpErr::valid("请指定待删除的字典".to_string()));
        }
        let dict_vec = Self::get_list_by_ids(data_id_vec).await?;
        if dict_vec.is_empty() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let mut rows_affected = 0;
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        for dict in dict_vec.iter() {
            let dict_code_path = dict.dict_code_path.clone();
            let dict_vec = Self::get_list_by_parent_code(&dict_code_path).await?;
            if !dict_vec.is_empty() {
                tx.rollback().await?;
                return Err(BmbpErr::valid("请先删除子字典".to_string()));
            };
            let delete_sql = BmbpConfigDict::delete_by_id();
            let result = sqlx::query(&delete_sql)
                .bind(&dict.data_id)
                .execute(&mut *tx)
                .await?;
            rows_affected += result.rows_affected() as usize;
        }
        tx.commit().await?;
        Ok(rows_affected)
    }
    pub(crate) async fn update_parent(dict_vo: &mut BmbpConfigDict) -> BmbpResp<usize> {
        if dict_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待变更的字典".to_string()));
        }
        if dict_vo.dict_parent_code.is_empty() {
            return Err(BmbpErr::valid("请指定字典的父级".to_string()));
        }
        let dict_info = Self::get_info_by_id(&dict_vo.data_id).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        if dict_vo.dict_parent_code == dict_info.unwrap().dict_code {
            return Err(BmbpErr::valid("字典的父级不能与自身相同".to_string()));
        }
        Self::update(dict_vo).await
    }

    pub(crate) async fn combo(dict_query_vo: &DictQueryVo) -> BmbpResp<Vec<ComboVo>> {
        let dict_alias = dict_query_vo.dict_code.clone();
        if dict_alias.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let dict_info = Self::get_info_by_alias(&dict_alias).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let dict_code = dict_info.unwrap().dict_code;
        let dict_vec = Self::get_list_by_parent_code(&dict_code).await?;
        let mut combo_vec = vec![];
        for dict in dict_vec.iter() {
            let combo_tmp = ComboVo {
                code: dict.dict_value.clone(),
                label: dict.dict_name.clone(),
                children: None,
            };
            combo_vec.push(combo_tmp);
        }
        Ok(combo_vec)
    }
    pub(crate) async fn combos(
        dict_query_vo: &DictQueryVo,
    ) -> BmbpResp<HashMap<String, Vec<ComboVo>>> {
        let dict_alias = dict_query_vo.dict_codes.clone();
        if dict_alias.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let mut combo_map = HashMap::new();
        for dict_alias in dict_alias.iter() {
            let combo_query_vo = DictQueryVo {
                dict_code: dict_alias.to_string(),
                ..Default::default()
            };
            let combo_vec = Self::combo(&combo_query_vo).await?;
            combo_map.insert(dict_alias.to_string(), combo_vec);
        }
        Ok(combo_map)
    }

    pub(crate) async fn combo_tree(dict_query_vo: &DictQueryVo) -> BmbpResp<Vec<ComboVo>> {
        let dict_alias = dict_query_vo.dict_code.clone();
        if dict_alias.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let dict_info = Self::get_info_by_alias(&dict_alias).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let dict_code_path = dict_info.unwrap().dict_code_path;
        let dict_vec = Self::get_list_by_parent_code_path(&dict_code_path).await?;

        let dict_vec_tree = BmbpTreeUtil::build::<BmbpConfigDict>(dict_vec);

        let mut combo_vec = vec![];
        for dict in dict_vec_tree.iter() {
            let combo_tmp = Self::build_tree_combo(dict);
            combo_vec.push(combo_tmp);
        }
        Ok(combo_vec)
    }
    pub(crate) async fn combos_tree(
        dict_query_vo: &DictQueryVo,
    ) -> BmbpResp<HashMap<String, Vec<ComboVo>>> {
        let dict_alias = dict_query_vo.dict_codes.clone();
        if dict_alias.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let mut combo_map = HashMap::new();
        for dict_alias in dict_alias.iter() {
            let combo_query_vo = DictQueryVo {
                dict_code: dict_alias.to_string(),
                ..Default::default()
            };
            let combo_vec = Self::combo_tree(&combo_query_vo).await?;
            combo_map.insert(dict_alias.to_string(), combo_vec);
        }
        Ok(combo_map)
    }
    fn build_tree_combo(dict: &BmbpConfigDict) -> ComboVo {
        let mut combo_tmp = ComboVo {
            code: dict.dict_value.clone(),
            label: dict.dict_name.clone(),
            children: None,
        };
        if let Some(children) = &dict.dict_children {
            let mut child_combo_vec = vec![];
            for child in children.iter() {
                let combo_tmp = Self::build_tree_combo(child);
                child_combo_vec.push(combo_tmp);
            }
            combo_tmp.children = Some(child_combo_vec);
        }
        combo_tmp
    }
    pub(crate) async fn display_convert(
        dict_query_vo: &DictQueryVo,
    ) -> BmbpResp<HashMap<String, String>> {
        if dict_query_vo.dict_code.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let dict_info = Self::get_info_by_alias(&dict_query_vo.dict_code).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let dict_code = dict_info.as_ref().unwrap().dict_code.to_string();
        let dict_vec = Self::get_list_by_parent_code(&dict_code).await?;
        let mut map = HashMap::new();
        for dict in dict_vec.iter() {
            map.insert(dict.dict_value.to_string(), dict.dict_name.to_string());
        }
        Ok(map)
    }
    pub(crate) async fn displays_convert(
        dict_query_vo: &DictQueryVo,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        if dict_query_vo.dict_codes.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let mut map = HashMap::new();

        for dict_alias in dict_query_vo.dict_codes.iter() {
            let combo_map = Self::display_convert(dict_query_vo).await?;
            map.insert(dict_alias.to_string(), combo_map);
        }
        Ok(map)
    }
    pub(crate) async fn display_convert_tree(
        dict_query_vo: &DictQueryVo,
    ) -> BmbpResp<HashMap<String, String>> {
        if dict_query_vo.dict_code.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let dict_info = Self::get_info_by_alias(&dict_query_vo.dict_code).await?;
        if dict_info.is_none() {
            return Err(BmbpErr::valid("指定的字典不存在".to_string()));
        }
        let dict_code_path = dict_info.as_ref().unwrap().dict_code_path.to_string();
        let dict_vec = Self::get_list_by_parent_code_path(&dict_code_path).await?;
        let dict_vec_tree = BmbpTreeUtil::build::<BmbpConfigDict>(dict_vec);
        let mut map = Self::build_tree_display_convert(dict_vec_tree.as_slice());
        Ok(map)
    }
    pub(crate) async fn displays_convert_tree(
        dict_query_vo: &DictQueryVo,
    ) -> BmbpResp<HashMap<String, HashMap<String, String>>> {
        if dict_query_vo.dict_codes.is_empty() {
            return Err(BmbpErr::valid("请指定字典别名".to_string()));
        }
        let mut map = HashMap::new();

        for dict_alias in dict_query_vo.dict_codes.iter() {
            let combo_map = Self::display_convert_tree(dict_query_vo).await?;
            map.insert(dict_alias.to_string(), combo_map);
        }
        Ok(map)
    }
    fn build_tree_display_convert(dict_slice: &[BmbpConfigDict]) -> HashMap<String, String> {
        let mut display_map = HashMap::new();
        for dict in dict_slice {
            let key = dict.dict_value.to_string();
            display_map.insert(key.clone(), dict.dict_name.to_string());
            if let Some(children) = dict.dict_children.as_ref() {
                let child_map = Self::build_tree_display_convert(children.as_slice());
                for (k, v) in child_map {
                    display_map.insert(format!("{}.{}", key, k), v);
                }
            }
        }
        display_map
    }
}
