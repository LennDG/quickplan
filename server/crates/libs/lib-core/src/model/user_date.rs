use super::base::{self, crud_fns, DbBmc};
use super::fields::{ModelDate, Timestamp};
use super::ModelManager;

use crate::ctx::Ctx;
use crate::model::{Error, Result};
use modql::field::Fields;
use modql::FromSqliteRow;
use sea_query::{Iden, Query};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// region:	  --- Plan Date Types
#[derive(Debug, Clone, Fields, FromSqliteRow)]
pub struct UserDate {
    // -- Relations
    pub user_id: i64,

    // -- Properties
    pub date: ModelDate,

    // -- Timestamps
    pub ctime: Timestamp,
}

#[derive(Deserialize, Fields)]
pub struct UserDateForCreate {
    pub user_id: i64,
    pub date: ModelDate,
}

#[derive(Deserialize)]
pub struct UserDateForCreateMulti {
    pub user_id: i64,
    pub dates: Vec<ModelDate>,
}
// endregion: --- Plan Date Types

// region:	  --- Plan Date Bmc

pub struct UserDateBmc;

impl DbBmc for UserDateBmc {
    const TABLE: &'static str = "user_date";
}

impl UserDateBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, date_c: UserDateForCreate) -> Result<i64> {
        crud_fns::create::<Self, _>(ctx, mm, date_c).await
    }

    pub async fn create_multiple(
        ctx: &Ctx,
        mm: &ModelManager,
        date_c_m: UserDateForCreateMulti,
    ) -> Result<Vec<i64>> {
        let plan_c_m = date_c_m
            .dates
            .into_iter()
            .map(|date| UserDateForCreate {
                user_id: date_c_m.user_id,
                date,
            })
            .collect();

        crud_fns::create_multiple::<Self, _>(ctx, mm, plan_c_m).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<UserDate> {
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
    use time::Date;

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
                url_id: "create_multiple_plan_date".to_string(),
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

        let date_c_m = UserDateForCreateMulti {
            user_id: fx_user_id,
            dates: vec![
                ModelDate::new(Date::from_calendar_date(2024, time::Month::September, 5)?),
                ModelDate::new(Date::from_calendar_date(2024, time::Month::October, 20)?),
                ModelDate::new(Date::from_calendar_date(2024, time::Month::February, 22)?),
                ModelDate::new(Date::from_calendar_date(2024, time::Month::March, 21)?),
            ],
        };

        // -- Exec
        let ids = UserDateBmc::create_multiple(&ctx, &mm, date_c_m).await?;

        // -- Check
        for id in ids.clone() {
            UserDateBmc::get(&ctx, &mm, id).await?;
        }

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, fx_plan_id).await?;

        Ok(())
    }
}
// endregion: --- Tests
