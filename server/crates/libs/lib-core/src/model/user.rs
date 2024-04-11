use lib_utils::time::Rfc3339;
use modql::{field::Fields, FromSqliteRow};
use serde::Serialize;
use serde_with::serde_as;

use super::{
    base::{crud_fns, DbBmc},
    fields::Timestamp,
    user_date::UserDate,
    ModelManager,
};
use crate::ctx::Ctx;
use crate::model::{Error, Result};

// region:	  --- User Types
#[derive(Debug, Fields, Clone, FromSqliteRow)]
pub struct User {
    // -- Relations
    pub id: i64,

    // -- Properties
    pub name: String,

    // -- Timestamps
    pub ctime: Timestamp,
}

#[derive(Fields)]
pub struct UserForCreate {
    pub plan_id: i64,
    pub name: String,
}

pub struct UserDates {
    pub plan_id: i64,
    pub name: String,
    pub dates: Vec<UserDate>,
}
// endregion: --- User Types

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "plan_user";
}

impl UserBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<i64> {
        crud_fns::create::<Self, _>(ctx, mm, user_c).await
    }

    pub async fn create_return(
        ctx: &Ctx,
        mm: &ModelManager,
        user_c: UserForCreate,
    ) -> Result<User> {
        crud_fns::create_return::<Self, _, _>(ctx, mm, user_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<User> {
        crud_fns::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        crud_fns::delete::<Self>(ctx, mm, id).await
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::time::Duration;

    use crate::{
        _dev_utils,
        model::plan::{PlanBmc, PlanForCreate},
    };

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_user_bmc_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_plan_name = "plan_user_create_ok";
        let fx_plan_urlid = "plan_url_user_create_ok";
        let plan_c = PlanForCreate {
            name: fx_plan_name.to_string(),
            url_id: fx_plan_urlid.to_string(),
            description: None
        };
        let fx_user_name = "user_create_ok";

        // -- Exec
        let plan_id = PlanBmc::create(&ctx, &mm, plan_c).await?;

        let user_c = UserForCreate {
            plan_id,
            name: fx_user_name.to_string(),
        };
        let user_id = UserBmc::create(&ctx, &mm, user_c).await?;

        // -- Check
        let user = UserBmc::get(&ctx, &mm, user_id).await?;
        assert_eq!(fx_user_name, user.name);

        // -- Cleanup
        PlanBmc::delete(&ctx, &mm, plan_id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_user_bmc_create_name_too_long_fail() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_plan_name = "plan_user_create_name_too_long_fail";
        let fx_plan_urlid = "user_create_name_too_long_fail";
        let plan_c = PlanForCreate {
            name: fx_plan_name.to_string(),
            url_id: fx_plan_urlid.to_string(),
            description: None
        };
        let fx_user_name = "This is a string input for the test. It serves as a demonstration of a text that exceeds the required length of 128 characters. 
        The purpose is to test how the system handles longer inputs and whether it correctly identifies them as being too long.";

        // -- Exec
        let plan_id = PlanBmc::create(&ctx, &mm, plan_c).await?;

        let user_c = UserForCreate {
            plan_id,
            name: fx_user_name.to_string(),
        };
        let result_user_name_too_long = UserBmc::create(&ctx, &mm, user_c).await;

        // -- Check
        assert!(result_user_name_too_long.is_err());

        Ok(())
    }
}
// endregion: --- Tests
