// region:	  --- Modules

use modql::field::HasFields;
use modql::SIden;
use sea_query::{
    query, Expr, Iden, IntoIden, Query, SeaRc, SimpleExpr, SqliteQueryBuilder, TableRef,
};
use sea_query_binder::SqlxBinder;
use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;

use crate::ctx::Ctx;
use crate::model::{Error, Result};

use super::utils::prep_fields_for_create;
use super::ModelManager;
use super::{CommonIden, DbBmc};

// endregion: --- Modules

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DbBmc,
    E: HasFields,
{
    let db = mm.db();

    // -- Extract fields (name / sea-query value expression)
    let mut fields = data.not_none_fields();
    prep_fields_for_create::<MC>(&mut fields);
    let (columns, sea_values) = fields.for_sea_insert();

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning_col(CommonIden::Id);

    // -- Exec query
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let (id,) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(id)
}

pub async fn create_return<MC, E, T>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<T>
where
    MC: DbBmc,
    E: HasFields,
    T: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    let db = mm.db();

    // -- Extract fields (name / sea-query value expression)
    let mut fields = data.not_none_fields();
    prep_fields_for_create::<MC>(&mut fields);
    let (columns, sea_values) = fields.for_sea_insert();

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning_all();

    // -- Exec query
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let (created) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(created)
}

pub async fn create_multiple<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: Vec<E>) -> Result<Vec<i64>>
where
    MC: DbBmc,
    E: HasFields,
{
    let db = mm.db();

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .returning_col(CommonIden::Id);

    let columns: Vec<SeaRc<dyn Iden>> = Vec::new();

    for d in data {
        // -- Extract fields (name / sea-query value expression)
        let mut fields = d.not_none_fields();
        prep_fields_for_create::<MC>(&mut fields);
        let (columns, sea_values) = fields.for_sea_insert();
        query.columns(columns);
        query.values(sea_values);
    }

    //-- Exec query
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let ids: Vec<(i64,)> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;
    let ids: Vec<i64> = ids.into_iter().map(|(id,)| id).collect();

    Ok(ids)
}

pub async fn create_multiple_return<MC, E, T>(
    _ctx: &Ctx,
    mm: &ModelManager,
    data: Vec<E>,
) -> Result<Vec<T>>
where
    MC: DbBmc,
    E: HasFields,
    T: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    let db = mm.db();

    // -- Build query
    let mut query = Query::insert();
    query.into_table(MC::table_ref()).returning_all();

    let columns: Vec<SeaRc<dyn Iden>> = Vec::new();
    for d in data {
        // -- Extract fields (name / sea-query value expression)
        let mut fields = d.not_none_fields();
        prep_fields_for_create::<MC>(&mut fields);
        let (columns, sea_values) = fields.for_sea_insert();

        query.values(sea_values)?;
    }

    query.columns(columns);

    //-- Exec query
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let entities: Vec<T> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    Ok(entities)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
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
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
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
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
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

    // -- Build query
    let mut query = Query::delete();
    query
        .from_table(MC::table_ref())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    // -- Exec Query
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let count = sqlx::query_with(&sql, values)
        .execute(db)
        .await?
        .rows_affected();

    // -- Check result
    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}
