// region:	  --- Modules

use std::process::id;

use sqlx::postgres::PgRow;
use sqlx::FromRow;

use crate::ctx::Ctx;
use crate::model::{Error, Result};

use super::ModelManager;

// endregion: --- Modules

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DbBmc,
{
    let db = mm.db();

    todo!()

    // let fields = data.not_none_fields();
    // let (id,) = sqlb::insert()
    //     .table(MC::TABLE)
    //     .data(fields)
    //     .returning(&["id"])
    //     .fetch_one(db)
    //     .await?;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();

    todo!()
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();

    todo!()
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    todo!()
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    todo!()
}
