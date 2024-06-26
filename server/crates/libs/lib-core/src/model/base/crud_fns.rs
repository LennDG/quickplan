// region:	  --- Modules

use crate::model::{Error, Result};
use modql::field::HasSeaFields;
use modql::{FromSqliteRow, SIden};
use sea_query::{
    query, Expr, Iden, IntoIden, Query, SeaRc, SimpleExpr, SqliteQueryBuilder, TableRef,
};
use sea_query_rusqlite::RusqliteBinder;

use super::utils::prep_fields_for_create;
use super::ModelManager;
use super::{CommonIden, DbBmc};

// endregion: --- Modules

pub async fn create<MC, E>(mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let db = mm.db();

    // -- Extract fields (name / sea-query value expression)
    let mut fields = data.not_none_sea_fields();
    prep_fields_for_create::<MC>(&mut fields);
    let (columns, sea_values) = fields.for_sea_insert();

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning_col(CommonIden::Id);

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

    // -- Exec query
    // Wait for the Mutex to free up, then execute.
    let db = db.lock().await;
    let mut stmt = db.prepare(&sql)?;
    let id = stmt.query_row(&*values.as_params(), |row| row.get(0))?;
    Ok(id)
}

pub async fn create_return<MC, E, T>(mm: &ModelManager, data: E) -> Result<T>
where
    MC: DbBmc,
    E: HasSeaFields,
    T: FromSqliteRow + Unpin + Send,
{
    let db = mm.db();

    // -- Extract fields (name / sea-query value expression)
    let mut fields = data.not_none_sea_fields();
    prep_fields_for_create::<MC>(&mut fields);
    let (columns, sea_values) = fields.for_sea_insert();

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning_all();

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

    // -- Exec query
    let db = db.lock().await;
    let mut stmt = db.prepare(&sql)?;
    let created = stmt
        .query_and_then(&*values.as_params(), T::from_sqlite_row)?
        .next()
        .ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)??;

    Ok(created)
}

pub async fn create_multiple<MC, E>(mm: &ModelManager, data: Vec<E>) -> Result<Vec<i64>>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let db = mm.db();

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .returning_col(CommonIden::Id);

    for d in data {
        // -- Extract fields (name / sea-query value expression)
        let mut fields = d.not_none_sea_fields();
        prep_fields_for_create::<MC>(&mut fields);
        let (columns, sea_values) = fields.for_sea_insert();
        query.columns(columns);
        let _ = query.values(sea_values);
    }

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

    //-- Exec query
    let db = db.lock().await;
    let mut stmt = db.prepare(&sql)?;
    let ids: Result<Vec<i64>> = stmt
        .query_and_then(&*values.as_params(), |row| {
            row.get::<_, i64>(0).map_err(Error::Rusqlite)
        })?
        .collect();

    ids
}

// pub async fn create_multiple_return<MC, E, T>(
//     _ctx:
//     mm: &ModelManager,
//     data: Vec<E>,
// ) -> Result<Vec<T>>
// where
//     MC: DbBmc,
//     E: HasFields,
//     T: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
// {
//     let db = mm.db();

//     // -- Build query
//     let mut query = Query::insert();
//     query.into_table(MC::table_ref()).returning_all();

//     let columns: Vec<SeaRc<dyn Iden>> = Vec::new();
//     for d in data {
//         // -- Extract fields (name / sea-query value expression)
//         let mut fields = d.not_none_fields();
//         prep_fields_for_create::<MC>(&mut fields);
//         let (columns, sea_values) = fields.for_sea_insert();

//         query.values(sea_values)?;
//     }

//     query.columns(columns);

//     //-- Exec query
//     let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
//     let mut tx = db.begin().await?;
//     let entities: Vec<T> = sqlx::query_as_with(&sql, values)
//         .fetch_all(&mut *tx)
//         .await?;
//     tx.commit().await?;

//     Ok(entities)
// }

pub async fn get<MC, E>(mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: FromSqliteRow + Unpin + Send,
    E: HasSeaFields,
{
    let db = mm.db();

    // -- Build Query
    let mut query = Query::select();
    query
        .from(MC::table_ref())
        .columns(E::sea_column_refs())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

    // -- Exec query
    let db = db.lock().await;
    let mut stmt = db.prepare(&sql)?;
    let entity = stmt
        .query_and_then(&*values.as_params(), E::from_sqlite_row)?
        .next()
        .ok_or_else(|| Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })??;

    Ok(entity)
}

// pub async fn list<MC, E>(_ctx:  mm: &ModelManager) -> Result<Vec<E>>
// where
//     MC: DbBmc,
//     E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
//     E: HasFields,
// {
//     let db = mm.db();

//     todo!()
// }

// pub async fn update<MC, E>(_ctx:  mm: &ModelManager, id: i64, data: E) -> Result<()>
// where
//     MC: DbBmc,
//     E: HasFields,
// {
//     let db = mm.db();

//     todo!()
// }

pub async fn delete<MC>(mm: &ModelManager, id: i64) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    // -- Build query
    let mut query = Query::delete();
    query
        .from_table(MC::table_ref())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

    // -- Exec Query
    let db = db.lock().await;
    let mut stmt = db.prepare(&sql)?;
    let count = stmt.execute(&*values.as_params())?;

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
