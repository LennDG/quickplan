use super::base;
use super::{base::DbBmc, ModelManager};

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

#[derive(Iden)]
enum PlanDateTable {
    PlanId,
    UserId,
    Date,
    Ctime,
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
    pub date: Vec<Date>,
}
// endregion: --- Plan Date Types

// region:	  --- Plan Date Bmc

pub struct PlanDateBmc;

impl DbBmc for PlanDateBmc {
    const TABLE: &'static str = "plan_date";
}

impl PlanDateBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, date_c: PlanDateForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, date_c).await
    }

    pub async fn create_multiple(
        ctx: &Ctx,
        mm: &ModelManager,
        date_c: PlanDateForCreateMulti,
    ) -> Result<Vec<i64>> {
        todo!()
    }
}

// endregion: --- Plan Date Bmc
