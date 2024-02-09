use lib_utils::time::Rfc3339;
use modql::field::Fields;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;

use super::{
    base::{crud_fns, DbBmc},
    ModelManager,
};
use crate::ctx::Ctx;
use crate::model::{Error, Result};

// region:	  --- User Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct User {
    // -- Relations
    pub id: i64,

    // -- Properties
    pub name: String,

    // -- Timestamps
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
}

#[derive(Fields)]
pub struct UserForCreate {
    pub plan_id: i64,
    pub name: String,
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
