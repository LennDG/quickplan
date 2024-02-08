// region:	  --- Modules

use std::process::id;

use modql::field::HasFields;
use modql::SIden;
use sea_query::{Expr, Iden, IntoIden, PostgresQueryBuilder, Query, TableRef};
use sea_query_binder::SqlxBinder;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use crate::ctx::Ctx;
use crate::model::{Error, Result};

use super::ModelManager;

// endregion: --- Modules

#[derive(Iden)]
pub enum CommonIden {
    Id,
}

pub trait DbBmc {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DbBmc,
    E: HasFields,
{
    let db = mm.db();

    // -- Extract fields (name / sea-query value expression)
    let fields = data.not_none_fields();
    let (columns, sea_values) = fields.for_sea_insert();

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning(Query::returning().columns([CommonIden::Id]));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let (id,) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(id)
}

pub async fn create_multiple<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: Vec<E>) -> Result<()>
where
    MC: DbBmc,
    E: HasFields,
{
    todo!()
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();

    // -- Build Query
    let mut query = Query::select();
    query
        .from(MC::table_ref())
        .columns(E::field_column_refs())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entity = sqlx::query_as_with::<_, E, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })?;

    Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();

    todo!()
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
where
    MC: DbBmc,
    E: HasFields,
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
