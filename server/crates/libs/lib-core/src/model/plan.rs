use modql::{
    field::{Fields, HasSeaFields},
    FromSqliteRow,
};
use rusqlite::Row;
use sea_query::{Expr, Iden, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::Serialize;
use serde_with::serde_as;

use crate::{ctx::Ctx, model::user::UserBmc};

use super::{
    base::{self, crud_fns, DbBmc},
    fields::Timestamp,
    user::UserDates,
    ModelManager,
};
use crate::model::{Error, Result};

// region:	  --- Plan Types
#[derive(Debug, Clone, Fields, FromSqliteRow)]
pub struct Plan {
    // -- Relations
    pub id: i64,

    // -- Properties
    pub name: String,
    pub url_id: String,
    pub description: Option<String>,

    // -- Timestamps
    pub ctime: Timestamp,
}

#[derive(Fields, Clone)]
pub struct PlanForCreate {
    pub name: String,
    pub url_id: String,
}

#[derive(Iden)]
pub enum PlanIden {
    UrlId,
}
// endregion: --- Plan Types

// region:	  --- PlanBmc
pub struct PlanBmc;

impl DbBmc for PlanBmc {
    const TABLE: &'static str = "plan";
}

impl PlanBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, plan_c: PlanForCreate) -> Result<i64> {
        crud_fns::create::<Self, _>(ctx, mm, plan_c).await
    }

    pub async fn create_return(
        ctx: &Ctx,
        mm: &ModelManager,
        plan_c: PlanForCreate,
    ) -> Result<Plan> {
        crud_fns::create_return::<Self, _, _>(ctx, mm, plan_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Plan> {
        crud_fns::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        crud_fns::delete::<Self>(ctx, mm, id).await
    }

    pub async fn get_plan_by_url(
        _ctx: &Ctx,
        url_id: &str,
        mm: &ModelManager,
    ) -> Result<Option<Plan>> {
        let db = mm.db();

        // -- Build Query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(Plan::sea_column_refs())
            .and_where(Expr::col(PlanIden::UrlId).eq(url_id));
        let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

        // -- Exec query
        let db = db.lock().await;
        let mut stmt = db.prepare(&sql)?;
        let plan = stmt
            .query_and_then(&*values.as_params(), Plan::from_sqlite_row)?
            .next()
            .transpose();

        Ok(plan?)
    }
}

// endregion: --- PlanBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::time::Duration;

    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_plan_bmc_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_plan_name = "plan_create_ok";
        let fx_plan_urlid = "planurl_create_ok";
        let plan_c = PlanForCreate {
            name: fx_plan_name.to_string(),
            url_id: fx_plan_urlid.to_string(),
        };

        // -- Exec
        let id = PlanBmc::create(&ctx, &mm, plan_c.clone()).await?;
        // -- Check
        let plan = PlanBmc::get(&ctx, &mm, id).await?;
        assert_eq!(fx_plan_name, plan.name);

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_plan_bmc_create_return_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_plan_name = "plan_create_return_ok";
        let fx_plan_urlid = "planurl_create_return_ok";
        let plan_c = PlanForCreate {
            name: fx_plan_name.to_string(),
            url_id: fx_plan_urlid.to_string(),
        };

        // -- Exec
        let plan = PlanBmc::create_return(&ctx, &mm, plan_c).await?;

        // -- Check
        assert_eq!(fx_plan_name, plan.name);

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, plan.id).await?;

        Ok(())
    }
}
// endregion: --- Tests
