use super::base::{self, crud_fns, DbBmc};
use super::ModelManager;

use crate::ctx::Ctx;
use crate::model::{Error, Result};

use lib_utils::time::Rfc3339;
use modql::field::Fields;
use sea_query::{Iden, Query};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::{Date, OffsetDateTime};

// region:	  --- Plan Date Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct PlanDate {
    // -- Relations
    pub plan_id: i64,
    pub user_id: i64,

    // -- Properties
    pub date: Date,

    // -- Timestamps
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct PlanDateForCreate {
    pub plan_id: i64,
    pub user_id: i64,
    pub date: Date,
}

#[derive(Deserialize)]
pub struct PlanDateForCreateMulti {
    pub plan_id: i64,
    pub user_id: i64,
    pub dates: Vec<Date>,
}
// endregion: --- Plan Date Types

// region:	  --- Plan Date Bmc

pub struct PlanDateBmc;

impl DbBmc for PlanDateBmc {
    const TABLE: &'static str = "plan_date";
}

impl PlanDateBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, date_c: PlanDateForCreate) -> Result<i64> {
        crud_fns::create::<Self, _>(ctx, mm, date_c).await
    }

    pub async fn create_multiple(
        ctx: &Ctx,
        mm: &ModelManager,
        date_c_m: PlanDateForCreateMulti,
    ) -> Result<Vec<i64>> {
        let plan_c_m = date_c_m
            .dates
            .into_iter()
            .map(|date| PlanDateForCreate {
                plan_id: date_c_m.plan_id,
                user_id: date_c_m.user_id,
                date,
            })
            .collect();

        crud_fns::create_multiple::<Self, _>(ctx, mm, plan_c_m).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<PlanDate> {
        crud_fns::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        crud_fns::delete::<Self>(ctx, mm, id).await
    }
}

// endregion: --- Plan Date Bmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::{
        _dev_utils,
        model::{
            plan::{PlanBmc, PlanForCreate},
            user::{User, UserBmc, UserForCreate},
        },
    };

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_create_multiple_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_plan_id = PlanBmc::create(
            &ctx,
            &mm,
            PlanForCreate {
                name: "create_multiple_plan_date".to_string(),
                urlid: "create_multiple_plan_date".to_string(),
            },
        )
        .await?;
        let fx_user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForCreate {
                plan_id: fx_plan_id,
                name: "create_multiple_plan_date".to_string(),
            },
        )
        .await?;

        let date_c_m = PlanDateForCreateMulti {
            plan_id: fx_plan_id,
            user_id: fx_user_id,
            dates: vec![
                Date::from_calendar_date(2024, time::Month::September, 5)?,
                Date::from_calendar_date(2024, time::Month::October, 20)?,
                Date::from_calendar_date(2024, time::Month::February, 22)?,
                Date::from_calendar_date(2024, time::Month::March, 21)?,
            ],
        };

        // Exec
        let ids = PlanDateBmc::create_multiple(&ctx, &mm, date_c_m).await?;

        // -- Check
        for id in ids.clone() {
            PlanDateBmc::get(&ctx, &mm, id).await?;
        }

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, fx_plan_id).await?;

        Ok(())
    }
}
// endregion: --- Tests
