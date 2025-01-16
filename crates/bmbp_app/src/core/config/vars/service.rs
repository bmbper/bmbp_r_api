use crate::core::abc::{
    now_date_time, simple_id, BatchVo, BmbpErr, BmbpErrorKind, BmbpResp, BmbpTree, BmbpTreeUtil,
    ComboVo, PageVo, RespVo, DATA_DISABLE, DATA_ENABLE, DATA_FLAG, DATA_LEVEL, DATA_STATUS,
    TREE_PATH_SPLIT, TREE_ROOT_CODE,
};
use crate::core::config::vars::bean::BmbpConfigVars;
use crate::core::config::vars::handler::{insert, page};
use crate::orm::DB_POOL;
use bmbp_orm::OrmSimpleSQLTrait;
use bmbp_orm::{OrmTableTrait, PageData};
use sqlx::encode::IsNull::No;
use sqlx::query::{Query, QueryAs};
use std::collections::HashMap;
use tracing_log::log::{debug, info};

pub struct BmbpVarsService;

impl BmbpVarsService {
    pub(crate) async fn get_tree(vars_vo: &BmbpConfigVars) -> BmbpResp<Vec<BmbpConfigVars>> {
        let vars_list = BmbpVarsService::get_list(&vars_vo).await?;
        Ok(BmbpTreeUtil::build::<BmbpConfigVars>(vars_list))
    }
    pub(crate) async fn get_tree_ignore_node(
        vars_vo: &BmbpConfigVars,
    ) -> BmbpResp<Vec<BmbpConfigVars>> {
        let mut query_list_sql = BmbpConfigVars::select();
        let mut vars_code_path = {
            if !vars_vo.data_id.is_empty() {
                if let Some(old_vars) = Self::get_info(vars_vo).await? {
                    old_vars.vars_code_path.clone()
                } else {
                    "".to_string()
                }
            } else if !vars_vo.vars_code.is_empty() {
                if let Some(old_vars) = Self::get_info_by_code(vars_vo.vars_code.clone()).await? {
                    old_vars.vars_code_path.clone()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        };
        query_list_sql = format!(
            "{} WHERE VARS_CODE_PATH NOT LIKE CONCAT($1,'%')",
            query_list_sql
        );

        query_list_sql = format!(
            "{} ORDER BY VARS_PARENT_CODE ASC,DATA_ORDER ASC",
            query_list_sql
        );
        debug!("查询SQL:{}", query_list_sql);

        let vars_vec = sqlx::query_as(query_list_sql.as_str())
            .bind(vars_code_path)
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        Ok(BmbpTreeUtil::build::<BmbpConfigVars>(vars_vec))
    }
    pub(crate) async fn get_list(vars_vo: &BmbpConfigVars) -> BmbpResp<Vec<BmbpConfigVars>> {
        let mut query_list_sql = BmbpConfigVars::select();
        query_list_sql = format!(
            "{} ORDER BY VARS_PARENT_CODE ASC,DATA_ORDER ASC",
            query_list_sql
        );
        debug!("查询SQL:{}", query_list_sql);

        let vars_vec = sqlx::query_as(query_list_sql.as_str())
            .fetch_all(DB_POOL.get().unwrap())
            .await?;
        return Ok(vars_vec);
    }
    pub(crate) async fn get_list_by_ids(vars_id_vec: &[String]) -> BmbpResp<Vec<BmbpConfigVars>> {
        let mut query_list_sql = BmbpConfigVars::select();
        // 构建 IN 子句的占位符
        let placeholders: Vec<String> =
            (1..=vars_id_vec.len()).map(|i| format!("${}", i)).collect();
        let in_clause = placeholders.join(",");
        // 构建 SQL 语句
        query_list_sql = format!("{} WHERE DATA_ID IN ({})", query_list_sql, in_clause);
        let mut sqlx_query: QueryAs<_, BmbpConfigVars, _> = sqlx::query_as(query_list_sql.as_str());
        for vars_data_id in vars_id_vec {
            sqlx_query = sqlx_query.bind(vars_data_id);
        }

        let vars_vec = sqlx_query.fetch_all(DB_POOL.get().unwrap()).await?;
        return Ok(vars_vec);
    }
    async fn get_list_by_parent_code(vars_code: &String) -> BmbpResp<Vec<BmbpConfigVars>> {
        let mut select_one_sql = BmbpConfigVars::select();
        select_one_sql = format!("{} WHERE VARS_PARENT_CODE = $1", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigVars, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(vars_code);
        let vars_data = select_one_sqlx.fetch_all(&*DB_POOL.get().unwrap()).await?;
        Ok(vars_data)
    }
    async fn get_list_by_parent_code_path(
        vars_code_path: &String,
    ) -> BmbpResp<Vec<BmbpConfigVars>> {
        let mut select_one_sql = BmbpConfigVars::select();
        select_one_sql = format!(
            "{} WHERE VARS_CODE_PATH LIKE CONCAT($1,'%') AND VARS_CODE_PATH != $2 ",
            select_one_sql
        );
        let mut select_one_sqlx: QueryAs<_, BmbpConfigVars, _> =
            sqlx::query_as(select_one_sql.as_str())
                .bind(vars_code_path)
                .bind(vars_code_path);
        let vars_data = select_one_sqlx.fetch_all(&*DB_POOL.get().unwrap()).await?;
        Ok(vars_data)
    }
    pub(crate) async fn get_page(
        page_vo: &PageVo<BmbpConfigVars>,
    ) -> BmbpResp<PageData<BmbpConfigVars>> {
        let mut page_no = page_vo.page_num.unwrap_or(1);
        let mut page_size = page_vo.page_size.unwrap_or(10);
        page_no = if page_no == 0 { 1 } else { page_no };
        page_size = if page_size == 0 { 10 } else { page_size };

        let mut page_data: PageData<BmbpConfigVars> = PageData::default();
        page_data.page_num = page_vo.page_num.unwrap_or(1);
        page_data.page_size = page_vo.page_size.unwrap_or(10);

        let mut query_sql = BmbpConfigVars::select();
        let mut condition_sql = vec![];
        if let Some(vars_vo) = page_vo.params.as_ref() {
            if !vars_vo.vars_code.is_empty() {
                condition_sql.push(format!(
                    "VARS_CODE_PATH LIKE CONCAT('%/{}/%')",
                    vars_vo.vars_code
                ));
            }
            if !vars_vo.vars_code_path.is_empty() {
                condition_sql.push(format!(
                    "VARS_CODE_PATH LIKE CONCAT('{}%')",
                    vars_vo.vars_code_path
                ));
            }
            if !vars_vo.vars_name.is_empty() {
                condition_sql.push(format!("VARS_NAME LIKE CONCAT('%{}%')", vars_vo.vars_name));
            }
            if !vars_vo.vars_parent_code.is_empty() {
                condition_sql.push(format!(
                    "VARS_CODE_PATH LIKE CONCAT('%{}%')",
                    vars_vo.vars_parent_code
                ));
            }
            if !vars_vo.vars_alias.is_empty() {
                condition_sql.push(format!(
                    "VARS_ALIAS LIKE CONCAT('%{}%')",
                    vars_vo.vars_alias
                ));
            }
        }
        if !condition_sql.is_empty() {
            let condition_sql = condition_sql.join(" AND ");
            query_sql = format!("{} WHERE {}", query_sql, condition_sql);
        }
        // 字段排序
        query_sql = format!(
            "{} ORDER BY VARS_TREE_GRADE ASC,DATA_ORDER ASC,VARS_PARENT_CODE ASC",
            query_sql
        );
        // 计算总数
        let count_sql = format!("SELECT COUNT(*) as COUNT FROM ({}) AS t", query_sql);
        debug!("分页查询-COUNT SQL:{}", BmbpConfigVars::select().as_str());
        let mut count_sqlx = sqlx::query_as::<_, (i64,)>(count_sql.as_str());
        let count: (i64,) = count_sqlx.fetch_one(&*DB_POOL.get().unwrap()).await?;
        page_data.total = count.0.clone() as u64;

        // 计算偏移量
        let offset = (page_no - 1) * page_size;
        let page_sql = format!("{} LIMIT {} OFFSET {}", query_sql, page_size, offset);
        // 执行分页查询
        let mut page_sqlx = sqlx::query_as(page_sql.as_str());
        let data: Vec<BmbpConfigVars> = page_sqlx.fetch_all(&*DB_POOL.get().unwrap()).await?;
        page_data.data = Some(data);
        Ok(page_data)
    }
    pub(crate) async fn get_info(vars_vo: &BmbpConfigVars) -> BmbpResp<Option<BmbpConfigVars>> {
        Self::get_info_by_id(&vars_vo.data_id).await
    }

    pub(crate) async fn get_info_by_id(vars_id: &String) -> BmbpResp<Option<BmbpConfigVars>> {
        let select_one_sql = BmbpConfigVars::select_by_id();
        let mut select_one_sqlx: QueryAs<_, BmbpConfigVars, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(vars_id);
        let vars_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        Ok(vars_data)
    }
    pub(crate) async fn get_info_by_alias(vars_alias: &String) -> BmbpResp<Option<BmbpConfigVars>> {
        let select_one_sql = BmbpConfigVars::select();
        let select_one_sql = format!("{} WHERE VARS_ALIAS = $1", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigVars, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(vars_alias);
        let vars_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        Ok(vars_data)
    }
    pub(crate) async fn get_info_by_code(vars_code: String) -> BmbpResp<Option<BmbpConfigVars>> {
        if vars_code.is_empty() {
            return Err(BmbpErr::valid("参数编码不能为空".to_string()));
        }
        let mut select_one_sql = BmbpConfigVars::select();
        select_one_sql = format!("{} WHERE VARS_CODE = $1", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigVars, _> =
            sqlx::query_as(select_one_sql.as_str()).bind(vars_code);
        let vars_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        Ok(vars_data)
    }
    pub(crate) async fn save(vars: &mut BmbpConfigVars) -> BmbpResp<Option<BmbpConfigVars>> {
        let exist_vars = Self::get_info(vars).await?;
        if exist_vars.is_none() {
            Self::insert(vars).await?;
        } else {
            Self::update(vars).await?;
        }
        Self::get_info(vars).await
    }

    pub(crate) async fn insert(vars: &mut BmbpConfigVars) -> BmbpResp<usize> {
        if vars.data_id.is_empty() {
            vars.data_id = simple_id();
        }
        if vars.vars_code.is_empty() {
            vars.vars_code = vars.data_id.clone();
        }

        if vars.vars_name.is_empty() {
            return Err(BmbpErr::valid("参数名称不能为空".to_string()));
        }
        if vars.vars_alias.is_empty() {
            return Err(BmbpErr::valid("参数别名不能为空".to_string()));
        }
        if vars.vars_value.is_empty() {
            return Err(BmbpErr::valid("参数值不能为空".to_string()));
        }

        if vars.vars_parent_code.is_empty() || vars.vars_parent_code.as_str() == TREE_ROOT_CODE {
            vars.vars_parent_code = TREE_ROOT_CODE.to_string();
            vars.vars_code_path = format!(
                "{}{}{}{}",
                TREE_ROOT_CODE, TREE_PATH_SPLIT, vars.vars_code, TREE_PATH_SPLIT
            );
            vars.vars_name_path = format!(
                "{}{}{}{}",
                TREE_ROOT_CODE, TREE_PATH_SPLIT, vars.vars_name, TREE_PATH_SPLIT
            );
        } else {
            let parent_vars = Self::get_info_by_code(vars.vars_parent_code.clone()).await?;
            if parent_vars.is_none() {
                return Err(BmbpErr::valid("父级参数不存在".to_string()));
            }
            let parent_vars = parent_vars.unwrap();
            vars.vars_code_path = format!(
                "{}{}{}",
                parent_vars.vars_code_path, vars.vars_code, TREE_PATH_SPLIT
            );
            vars.vars_name_path = format!(
                "{}{}{}",
                parent_vars.vars_name_path, vars.vars_name, TREE_PATH_SPLIT
            );
        }
        // check same name
        Self::check_same_name(&vars.vars_name, &vars.vars_parent_code, &vars.data_id).await?;
        // check same alias
        Self::check_same_alias(&vars.vars_alias, &vars.data_id).await?;

        // set default value
        vars.data_flag = DATA_FLAG.to_string();
        vars.data_level = DATA_LEVEL.to_string();
        vars.data_status = DATA_STATUS.to_string();
        vars.data_create_time = now_date_time();
        vars.data_update_time = now_date_time();
        vars.data_owner_org = "".to_string();
        vars.data_sign = "".to_string();
        vars.vars_tree_grade = (vars.vars_code_path.split(TREE_PATH_SPLIT).count() as i64) - 2;

        let mut insert_sql = BmbpConfigVars::insert_all();
        info!("插入SQL:{}", insert_sql);
        let mut insert_sqlx = sqlx::query(insert_sql.as_str())
            .bind(&vars.vars_code)
            .bind(&vars.vars_parent_code)
            .bind(&vars.vars_code_path)
            .bind(&vars.vars_name)
            .bind(&vars.vars_name_path)
            .bind(&vars.vars_alias)
            .bind(&vars.vars_value)
            .bind(&vars.vars_type)
            .bind(&vars.vars_tree_grade)
            .bind(&vars.data_id)
            .bind(&vars.data_flag)
            .bind(&vars.data_level)
            .bind(&vars.data_status)
            .bind(&vars.data_order)
            .bind(&vars.data_create_time)
            .bind(&vars.data_update_time)
            .bind(&vars.data_create_user)
            .bind(&vars.data_update_user)
            .bind(&vars.data_owner_org)
            .bind(&vars.data_sign);
        insert_sqlx.execute(&*DB_POOL.get().unwrap()).await?;

        Ok(0usize)
    }
    pub(crate) async fn update(vars: &mut BmbpConfigVars) -> BmbpResp<usize> {
        if vars.data_id.is_empty() {
            return Err(BmbpErr::valid("参数ID不能为空".to_string()));
        }
        let old_vars = Self::get_info(vars).await?;
        if old_vars.is_none() {
            return Err(BmbpErr::valid("待更新的参数不存在".to_string()));
        }
        let mut old_vars = old_vars.unwrap();
        let old_vars_code_path = old_vars.vars_code_path.clone();
        let old_vars_name_path = old_vars.vars_name_path.clone();

        if vars.vars_name.is_empty() {
            vars.vars_name = old_vars.vars_name.clone();
        }
        if vars.vars_alias.is_empty() {
            vars.vars_alias = old_vars.vars_alias.clone();
        }
        if vars.vars_value.is_empty() {
            vars.vars_value = old_vars.vars_value.clone();
        }
        if vars.vars_type.is_empty() {
            vars.vars_type = old_vars.vars_type.clone();
        }
        if vars.vars_code.is_empty() {
            vars.vars_code = old_vars.vars_code.clone();
        }
        if vars.vars_parent_code.is_empty() || vars.vars_parent_code.as_str() == TREE_ROOT_CODE {
            vars.vars_code = old_vars.vars_code.clone();
            vars.vars_parent_code = TREE_ROOT_CODE.to_string();
            vars.vars_code_path = format!(
                "{}{}{}{}",
                TREE_ROOT_CODE, TREE_PATH_SPLIT, vars.vars_code, TREE_PATH_SPLIT
            );
            vars.vars_name_path = format!(
                "{}{}{}{}",
                TREE_ROOT_CODE, TREE_PATH_SPLIT, vars.vars_name, TREE_PATH_SPLIT
            );
        } else {
            let parent_vars = Self::get_info_by_code(vars.vars_parent_code.clone()).await?;
            if parent_vars.is_none() {
                return Err(BmbpErr::valid("父级参数不存在".to_string()));
            }
            let parent_vars = parent_vars.unwrap();
            vars.vars_code_path = format!(
                "{}{}{}",
                parent_vars.vars_code_path, vars.vars_code, TREE_PATH_SPLIT
            );
            vars.vars_name_path = format!(
                "{}{}{}",
                parent_vars.vars_name_path, vars.vars_name, TREE_PATH_SPLIT
            );
        }

        // check same name
        Self::check_same_name(&vars.vars_name, &vars.vars_parent_code, &vars.data_id).await?;
        // check same alias
        Self::check_same_alias(&vars.vars_alias, &vars.data_id).await?;

        // set default value
        vars.data_flag = old_vars.data_flag.clone();
        vars.data_level = old_vars.data_level.to_string();
        vars.data_status = old_vars.data_status.to_string();
        vars.data_create_time = old_vars.data_create_time.clone();
        vars.data_update_time = now_date_time();
        vars.data_create_user = "".to_string();
        vars.data_update_user = "".to_string();
        vars.data_owner_org = "".to_string();
        vars.data_sign = "".to_string();
        vars.vars_tree_grade = (vars.vars_code_path.split(TREE_PATH_SPLIT).count() as i64) - 2;

        let new_code_path = vars.vars_code_path.clone();
        let new_name_path = vars.vars_name_path.clone();

        // 开始事务
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        // 更新当前记当
        let mut update_sql = BmbpConfigVars::update_all();
        let update_query = sqlx::query(update_sql.as_str())
            .bind(&vars.vars_code)
            .bind(&vars.vars_parent_code)
            .bind(&vars.vars_code_path)
            .bind(&vars.vars_name)
            .bind(&vars.vars_name_path)
            .bind(&vars.vars_alias)
            .bind(&vars.vars_value)
            .bind(&vars.vars_type)
            .bind(&vars.vars_tree_grade)
            .bind(&vars.data_id)
            .bind(&vars.data_flag)
            .bind(&vars.data_level)
            .bind(&vars.data_status)
            .bind(&vars.data_order)
            .bind(&vars.data_create_time)
            .bind(&vars.data_update_time)
            .bind(&vars.data_create_user)
            .bind(&vars.data_update_user)
            .bind(&vars.data_owner_org)
            .bind(&vars.data_sign)
            .bind(&vars.data_id);
        update_query.execute(&mut *tx).await?;

        // 更新子级
        let mut update_child_sql =format!(" UPDATE {} SET VARS_CODE_PATH = REPLACE(VARS_CODE_PATH, $1, $2), VARS_NAME_PATH = REPLACE(VARS_NAME_PATH, $3, $4) WHERE VARS_CODE_PATH LIKE CONCAT($5,'%')",  BmbpConfigVars::table_name());
        let update_child_query = sqlx::query(update_child_sql.as_str())
            .bind(&old_vars_code_path)
            .bind(&new_code_path)
            .bind(&old_vars_name_path)
            .bind(&new_name_path)
            .bind(&old_vars_code_path);
        update_child_query.execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(0usize)
    }

    async fn check_same_name(
        vars_name: &String,
        vars_parent_code: &String,
        data_id: &String,
    ) -> BmbpResp<()> {
        let mut select_one_sql = BmbpConfigVars::select();
        select_one_sql = format!(
            "{} WHERE VARS_NAME = $1 AND VARS_PARENT_CODE = $2 AND DATA_ID != $3",
            select_one_sql
        );
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigVars, _> =
            sqlx::query_as(select_one_sql.as_str())
                .bind(vars_name)
                .bind(vars_parent_code)
                .bind(data_id);
        let vars_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        if vars_data.is_some() {
            return Err(BmbpErr::valid("参数名称重复".to_string()));
        }
        Ok(())
    }

    async fn check_same_alias(vars_alias: &String, data_id: &String) -> BmbpResp<()> {
        let mut select_one_sql = BmbpConfigVars::select();
        select_one_sql = format!("{} WHERE VARS_ALIAS = $1 AND DATA_ID != $2", select_one_sql);
        debug!("查询SQL:{}", select_one_sql);
        let mut select_one_sqlx: QueryAs<_, BmbpConfigVars, _> =
            sqlx::query_as(select_one_sql.as_str())
                .bind(vars_alias)
                .bind(data_id);
        let vars_data = select_one_sqlx
            .fetch_optional(&*DB_POOL.get().unwrap())
            .await?;
        if vars_data.is_some() {
            return Err(BmbpErr::valid("参数别名重复".to_string()));
        }
        Ok(())
    }
    pub(crate) async fn enable(vars_vo: &mut BmbpConfigVars) -> BmbpResp<usize> {
        if vars_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待启用的参数".to_string()));
        }
        let vars_info = Self::get_info_by_id(&vars_vo.data_id).await?;
        if vars_info.is_none() {
            return Err(BmbpErr::valid("指定的参数不存在".to_string()));
        }
        let vars_code_path = vars_info.as_ref().unwrap().vars_code_path.clone();
        let vars_code_vec = vars_code_path.split(TREE_PATH_SPLIT).collect::<Vec<_>>();
        // 构建 IN 子句的占位符
        let placeholders: Vec<String> = (1..=vars_code_vec.len())
            .map(|i| format!("${}", i))
            .collect();
        let in_clause = placeholders.join(",");
        // 构建 SQL 语句
        let update_sql = format!(
            "UPDATE {} SET DATA_STATUS = ${} WHERE vars_code IN ({})",
            BmbpConfigVars::table_name(),
            vars_code_vec.len() + 1, // new_status 是第一个参数
            in_clause
        );
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        let mut update_query = sqlx::query(&update_sql).bind(DATA_ENABLE);
        for code in vars_code_vec.iter() {
            update_query = update_query.bind(code);
        }
        let result = update_query.execute(&mut *tx).await;
        if result.is_err() {
            tx.rollback().await?;
            return Err(BmbpErr::valid("启用参数失败".to_string()));
        }
        tx.commit().await?;
        Ok(result?.rows_affected() as usize)
    }
    pub(crate) async fn disable(vars_vo: &mut BmbpConfigVars) -> BmbpResp<usize> {
        if vars_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待启用的参数".to_string()));
        }
        let vars_info = Self::get_info_by_id(&vars_vo.data_id).await?;
        if vars_info.is_none() {
            return Err(BmbpErr::valid("指定的参数不存在".to_string()));
        }
        let vars_code_path = vars_info.as_ref().unwrap().vars_code_path.clone();
        tracing::info!("禁用参数批量:{}", vars_code_path);
        // 构建 SQL 语句
        let update_sql = format!(
            "UPDATE {} SET DATA_STATUS = $1 WHERE VARS_CODE_PATH LIKE CONCAT($2,'%')",
            BmbpConfigVars::table_name()
        );
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        let mut update_query = sqlx::query(&update_sql)
            .bind(DATA_DISABLE)
            .bind(vars_code_path);
        let result = update_query.execute(&mut *tx).await;
        if result.is_err() {
            tx.rollback().await?;
            return Err(BmbpErr::valid("停用参数失败".to_string()));
        }
        tx.commit().await?;
        Ok(result?.rows_affected() as usize)
    }

    pub(crate) async fn delete(vars_vo: &mut BmbpConfigVars) -> BmbpResp<usize> {
        if vars_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待删除的参数".to_string()));
        }
        let mut vars_info = Self::get_info_by_id(&vars_vo.data_id).await?;
        if vars_info.is_none() {
            return Err(BmbpErr::valid("指定的参数不存在".to_string()));
        }
        let vars_vo = vars_info.unwrap();
        let vars_vec = Self::get_list_by_parent_code(&vars_vo.vars_code).await?;
        if !vars_vec.is_empty() {
            return Err(BmbpErr::valid("请先删除子参数".to_string()));
        };
        let delete_sql = BmbpConfigVars::delete_by_id();
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        let result = sqlx::query(&delete_sql)
            .bind(&vars_vo.data_id)
            .execute(&mut *tx)
            .await;
        if result.is_err() {
            tx.rollback().await?;
            return Err(BmbpErr::valid("删除参数失败".to_string()));
        }
        tx.commit().await?;
        Ok(result?.rows_affected() as usize)
    }

    pub(crate) async fn batch_enable(batch_vo: &BatchVo<String>) -> BmbpResp<usize> {
        let data_id_vec = batch_vo.batch_vo.as_slice();
        if data_id_vec.is_empty() {
            return Err(BmbpErr::valid("请指定待启用的参数".to_string()));
        }
        let vars_vec = Self::get_list_by_ids(data_id_vec).await?;
        if vars_vec.is_empty() {
            return Err(BmbpErr::valid("指定的参数不存在".to_string()));
        }
        let mut rows_affected = 0;
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        for vars in vars_vec.iter() {
            let vars_code_path = vars.vars_code_path.clone();
            let vars_code_vec = vars_code_path.split(TREE_PATH_SPLIT).collect::<Vec<_>>();
            // 构建 IN 子句的占位符
            let placeholders: Vec<String> = (1..=vars_code_vec.len())
                .map(|i| format!("${}", i))
                .collect();
            let in_clause = placeholders.join(",");
            // 构建 SQL 语句
            let update_sql = format!(
                "UPDATE {} SET DATA_STATUS = ${} WHERE vars_code IN ({})",
                BmbpConfigVars::table_name(),
                vars_code_vec.len() + 1, // new_status 是第一个参数
                in_clause
            );

            let mut update_query = sqlx::query(&update_sql);
            for code in vars_code_vec.iter() {
                update_query = update_query.bind(code);
            }
            update_query = update_query.bind(DATA_ENABLE);
            let result = update_query.execute(&mut *tx).await;
            if result.is_err() {
                tx.rollback().await?;
                return Err(BmbpErr::valid("启用参数失败".to_string()));
            }
            rows_affected += result?.rows_affected() as usize;
        }
        tx.commit().await?;
        Ok(rows_affected)
    }
    pub(crate) async fn batch_disable(batch_vo: &BatchVo<String>) -> BmbpResp<usize> {
        let data_id_vec = batch_vo.batch_vo.as_slice();
        if data_id_vec.is_empty() {
            return Err(BmbpErr::valid("请指定待停用的参数".to_string()));
        }
        let vars_vec = Self::get_list_by_ids(data_id_vec).await?;
        if vars_vec.is_empty() {
            return Err(BmbpErr::valid("指定的参数不存在".to_string()));
        }
        let mut rows_affected = 0;
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        for vars in vars_vec.iter() {
            let vars_code_path = vars.vars_code_path.clone();
            // 构建 SQL 语句
            let update_sql = format!(
                "UPDATE {} SET DATA_STATUS = $1 WHERE VARS_CODE_PATH LIKE CONCAT($2,'%')",
                BmbpConfigVars::table_name()
            );
            let mut update_query = sqlx::query(&update_sql)
                .bind(DATA_DISABLE)
                .bind(vars_code_path);
            let result = update_query.execute(&mut *tx).await;
            if result.is_err() {
                tx.rollback().await?;
                return Err(BmbpErr::valid("停用参数失败".to_string()));
            }
            rows_affected += result?.rows_affected() as usize;
        }
        tx.commit().await?;
        Ok(rows_affected)
    }
    pub(crate) async fn batch_delete(batch_vo: &BatchVo<String>) -> BmbpResp<usize> {
        let data_id_vec = batch_vo.batch_vo.as_slice();
        if data_id_vec.is_empty() {
            return Err(BmbpErr::valid("请指定待删除的参数".to_string()));
        }
        let vars_vec = Self::get_list_by_ids(data_id_vec).await?;
        if vars_vec.is_empty() {
            return Err(BmbpErr::valid("指定的参数不存在".to_string()));
        }
        let mut rows_affected = 0;
        let mut tx = DB_POOL.get().unwrap().begin().await?;
        for vars in vars_vec.iter() {
            let vars_code_path = vars.vars_code_path.clone();
            let vars_vec = Self::get_list_by_parent_code(&vars_code_path).await?;
            if !vars_vec.is_empty() {
                tx.rollback().await?;
                return Err(BmbpErr::valid("请先删除子参数".to_string()));
            };
            let delete_sql = BmbpConfigVars::delete_by_id();
            let result = sqlx::query(&delete_sql)
                .bind(&vars.data_id)
                .execute(&mut *tx)
                .await?;
            rows_affected += result.rows_affected() as usize;
        }
        tx.commit().await?;
        Ok(rows_affected)
    }
    pub(crate) async fn update_parent(vars_vo: &mut BmbpConfigVars) -> BmbpResp<usize> {
        if vars_vo.data_id.is_empty() {
            return Err(BmbpErr::valid("请指定待变更的参数".to_string()));
        }
        if vars_vo.vars_parent_code.is_empty() {
            return Err(BmbpErr::valid("请指定参数的父级".to_string()));
        }
        let vars_info = Self::get_info_by_id(&vars_vo.data_id).await?;
        if vars_info.is_none() {
            return Err(BmbpErr::valid("指定的参数不存在".to_string()));
        }
        if vars_vo.vars_parent_code == vars_info.unwrap().vars_code {
            return Err(BmbpErr::valid("参数的父级不能与自身相同".to_string()));
        }
        Self::update(vars_vo).await
    }
}
