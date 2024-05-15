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
    pub description: Option<String>,
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

    pub async fn get_plan_by_url(_ctx: &Ctx, mm: &ModelManager, url_id: &str) -> Result<Plan> {
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
            .ok_or_else(|| Error::PlanUrlNotFound {
                url_id: url_id.to_string(),
            })?;

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
        let fx_plan_description = "plan_description_create_ok";
        let plan_c = PlanForCreate {
            name: fx_plan_name.to_string(),
            url_id: fx_plan_urlid.to_string(),
            description: Some(fx_plan_description.to_string()),
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
            description: None,
        };

        // -- Exec
        let plan = PlanBmc::create_return(&ctx, &mm, plan_c).await?;

        // -- Check
        assert_eq!(fx_plan_name, plan.name);

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, plan.id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_plan_bmc_by_url_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_plan_name = "plan_url_ok";
        let fx_plan_urlid = "planurl_url_ok";
        let plan_c = PlanForCreate {
            name: fx_plan_name.to_string(),
            url_id: fx_plan_urlid.to_string(),
            description: None,
        };

        // -- Exec
        let id = PlanBmc::create(&ctx, &mm, plan_c.clone()).await?;
        // -- Check
        let plan = PlanBmc::get_plan_by_url(&ctx, &mm, fx_plan_urlid).await?;
        assert_eq!(fx_plan_name, plan.name);

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_plan_bmc_input_too_long_fail() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        // -- Exec
        let result_name_too_long = PlanBmc::create(&ctx, &mm, PlanForCreate {
            name: "This is a string input for the test. It serves as a demonstration of a text that exceeds the required length of 128 characters. 
            The purpose is to test how the system handles longer inputs and whether it correctly identifies them as being too long.".to_string(),
            url_id: "short".to_string(),
            description: None
        }).await;

        let result_url_id_too_long = PlanBmc::create(&ctx, &mm, PlanForCreate {
            name: "short".to_string(),
            url_id: "This is a string input for the test. It serves as a demonstration of a text that exceeds the required length of 128 characters. 
            The purpose is to test how the system handles longer inputs and whether it correctly identifies them as being too long.".to_string(),
            description: None
        }).await;

        let result_description_too_long = PlanBmc::create(&ctx, &mm, PlanForCreate {
            name: "short".to_string(),
            url_id: "short".to_string(),
            description: Some("
            Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. 
            Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. 
            Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. 
            Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. 
            Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. 
            Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. 
            Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. 
            Quisque rutrum. Aenean imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. 
            Etiam rhoncus. Maecenas tempus, tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. 
            Nam quam nunc, blandit vel, luctus pulvinar, hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus.
            Donec vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit amet orci eget eros faucibus tincidunt. Duis leo. 
            Sed fringilla mauris sit amet nibh. Donec sodales sagittis magna. 
            Sed consequat, leo eget bibendum sodales, augue velit cursus nunc, quis gravida magna mi a libero. Fusce vulputate eleifend sapien. 
            Vestibulum purus quam, scelerisque ut, mollis sed, nonummy id, metus. Nullam accumsan lorem in dui. 
            Cras ultricies mi eu turpis hendrerit fringilla. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; 
            In ac dui quis mi consectetuer lacinia. Nam pretium turpis et arcu. Duis arcu tortor, suscipit eget, imperdiet nec, imperdiet iaculis, ipsum. 
            Sed aliquam ultrices mauris. Integer ante arcu, accumsan a, consectetuer eget, posuere ut, mauris. Praesent adipiscing. 
            Phasellus ullamcorper ipsum rutrum nunc. Nunc nonummy metus. Vestib
            ".to_string())
        }).await;

        // -- Check
        assert!(result_name_too_long.is_err());
        assert!(result_url_id_too_long.is_err());
        assert!(result_description_too_long.is_err());

        Ok(())
    }
}
// endregion: --- Tests
