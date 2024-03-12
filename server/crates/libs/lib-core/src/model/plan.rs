use lib_utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::{FromRow, QueryBuilder};
use time::OffsetDateTime;

use crate::{ctx::Ctx, model::user::UserBmc};

use super::{
    base::{self, crud_fns, DbBmc},
    user::UserDates,
    ModelManager,
};
use crate::model::{Error, Result};

// region:	  --- Plan Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Plan {
    // -- Relations
    pub id: i64,

    // -- Properties
    pub name: String,
    pub url_id: String,
    pub description: Option<String>,

    // -- Timestamps
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
}

#[derive(Fields)]
pub struct PlanForCreate {
    pub name: String,
    pub url_id: String,
}

pub struct FullPlan {
    // -- Plan data
    pub plan: Plan,

    // -- User data
    pub user_dates: Vec<UserDates>,
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
            .columns(Plan::field_column_refs())
            .and_where(Expr::col(PlanIden::UrlId).eq(url_id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let plan = sqlx::query_as_with::<_, Plan, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(plan)
    }

    pub async fn get_full_plan(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<FullPlan> {
        let db = mm.db();

        // -- Build Query

        todo!()
    }
}

// endregion: --- PlanBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;

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

        // Exec
        let id = PlanBmc::create(&ctx, &mm, plan_c).await?;

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

        // Exec
        let plan = PlanBmc::create_return(&ctx, &mm, plan_c).await?;

        // -- Check
        assert_eq!(fx_plan_name, plan.name);

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, plan.id).await?;

        Ok(())
    }
}
// endregion: --- Tests
