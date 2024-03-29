// region:	  --- Modules

pub mod crud_fns;
pub mod utils;

use std::process::id;

use modql::field::HasFields;
use modql::SIden;
use sea_query::{Expr, Iden, IntoIden, Query, SqliteQueryBuilder, TableRef};
use sea_query_binder::SqlxBinder;
use sqlx::FromRow;

use crate::ctx::Ctx;
use crate::model::{Error, Result};

use super::ModelManager;

// endregion: --- Modules

// region:    --- SeaQuery Idens

#[derive(Iden)]
pub enum CommonIden {
    Id,
}

#[derive(Iden)]
pub enum TimestampIden {
    Ctime,
}

// endregion: --- SeaQuery Idens

pub trait DbBmc {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }

    fn has_creation_timestamp() -> bool {
        true
    }
}
