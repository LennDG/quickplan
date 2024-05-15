use super::base::{crud_fns, DbBmc};
use super::fields::{ModelDate, Timestamp};
use super::ModelManager;

use crate::model::Result;
use modql::field::Fields;
use modql::FromSqliteRow;
use serde::Deserialize;

// region:	  --- User Date Types
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
// endregion: --- User Date Types

// region:	  --- User Date Bmc

pub struct UserDateBmc;

impl DbBmc for UserDateBmc {
    const TABLE: &'static str = "user_date";
}

impl UserDateBmc {
    pub async fn create(mm: &ModelManager, date_c: UserDateForCreate) -> Result<i64> {
        crud_fns::create::<Self, _>(mm, date_c).await
    }

    pub async fn create_multiple(
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

        crud_fns::create_multiple::<Self, _>(mm, plan_c_m).await
    }

    pub async fn get(mm: &ModelManager, id: i64) -> Result<UserDate> {
        crud_fns::get::<Self, _>(mm, id).await
    }

    pub async fn delete(mm: &ModelManager, id: i64) -> Result<()> {
        crud_fns::delete::<Self>(mm, id).await
    }
}

// endregion: --- User Date Bmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::{
        _dev_utils,
        model::{
            plan::{PlanBmc, PlanForCreate},
            user::{self, User, UserBmc, UserForCreate},
        },
    };

    use super::*;
    use anyhow::Result;
    use time::Date;

    #[tokio::test]
    async fn test_create_user_date_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;

        let fx_date = Date::from_calendar_date(2024, time::Month::September, 5)?;

        let fx_plan_id = PlanBmc::create(
            &mm,
            PlanForCreate {
                name: "create_user_date".to_string(),
                url_id: "create_user_date".to_string(),
                description: None,
            },
        )
        .await?;

        let fx_user_id = UserBmc::create(
            &mm,
            UserForCreate {
                plan_id: fx_plan_id,
                name: "create_user_date".to_string(),
            },
        )
        .await?;

        let date_c = UserDateForCreate {
            user_id: fx_user_id,
            date: ModelDate::new(fx_date),
        };

        // -- Exec
        let id = UserDateBmc::create(&mm, date_c).await?;

        // -- Check
        let user_date = UserDateBmc::get(&mm, id).await?;
        assert_eq!(user_date.date.date(), fx_date);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_date_multiple_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;

        let fx_plan_id = PlanBmc::create(
            &mm,
            PlanForCreate {
                name: "create_multiple_user_date".to_string(),
                url_id: "create_multiple_user_date".to_string(),
                description: None,
            },
        )
        .await?;
        let fx_user_id = UserBmc::create(
            &mm,
            UserForCreate {
                plan_id: fx_plan_id,
                name: "create_multiple_user_date".to_string(),
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
        let ids = UserDateBmc::create_multiple(&mm, date_c_m).await?;

        // -- Check
        for id in ids.clone() {
            UserDateBmc::get(&mm, id).await?;
        }

        // -- Cleanup
        PlanBmc::delete(&mm, fx_plan_id).await?;

        Ok(())
    }
}
// endregion: --- Tests
