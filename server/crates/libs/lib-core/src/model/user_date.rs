use super::base::{crud_fns, DbBmc};
use super::fields::{ModelDate, Timestamp};
use super::ModelManager;

use crate::model::Result;
use modql::field::{Fields, HasSeaFields};
use modql::FromSqliteRow;
use sea_query::{all, Expr, Iden, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::Deserialize;

// region:	  --- User Date Types
#[derive(Debug, Clone, Fields, FromSqliteRow)]
pub struct UserDate {
    // -- Relations
    pub id: i64,
    pub user_id: i64,

    // -- Properties
    pub date: ModelDate,

    // -- Timestamps
    pub ctime: Timestamp,
}

#[derive(Iden)]
pub enum UserDateIden {
    UserId,
    Date,
}

#[derive(Clone, Deserialize, Fields)]
pub struct UserDateForCreate {
    pub user_id: i64,
    pub date: ModelDate,
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

    pub async fn get(mm: &ModelManager, id: i64) -> Result<UserDate> {
        crud_fns::get::<Self, _>(mm, id).await
    }

    pub async fn delete(mm: &ModelManager, id: i64) -> Result<()> {
        crud_fns::delete::<Self>(mm, id).await
    }

    pub async fn get_date(mm: &ModelManager, date_c: UserDateForCreate) -> Result<Option<i64>> {
        // -- Create query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(UserDate::sea_column_refs())
            .cond_where(all![
                Expr::col(UserDateIden::UserId).eq(date_c.user_id),
                Expr::col(UserDateIden::Date).eq(date_c.date)
            ]);
        let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

        // -- Exec Query
        let db = mm.db();
        let db = db.lock().await;
        let mut stmt = db.prepare(&sql)?;
        let date = stmt
            .query_and_then(&*values.as_params(), UserDate::from_sqlite_row)?
            .next()
            .transpose()?;

        match date {
            Some(user_date) => Ok(Some(user_date.id)),
            None => Ok(None),
        }
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

        // -- Cleanup
        PlanBmc::delete(&mm, fx_plan_id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_date_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;

        let fx_date = Date::from_calendar_date(2024, time::Month::October, 20)?;

        let fx_plan_id = PlanBmc::create(
            &mm,
            PlanForCreate {
                name: "get_user_date_ok".to_string(),
                url_id: "get_user_date_ok".to_string(),
                description: None,
            },
        )
        .await?;

        let fx_user_id = UserBmc::create(
            &mm,
            UserForCreate {
                plan_id: fx_plan_id,
                name: "get_user_date_ok".to_string(),
            },
        )
        .await?;

        let date_c = UserDateForCreate {
            user_id: fx_user_id,
            date: ModelDate::new(fx_date),
        };

        let fx_id = UserDateBmc::create(&mm, date_c.clone()).await?;

        // -- Exec
        let check_id = UserDateBmc::get_date(&mm, date_c).await?.unwrap();

        // -- Check
        assert_eq!(check_id, fx_id);

        Ok(())
    }
}
// endregion: --- Tests
