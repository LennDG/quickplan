use crate::config::core_config;
use lib_utils::envs::get_env;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite, SqlitePool,
};
use std::{str::FromStr, time::Duration};

// region:	  --- Modules
mod error;
pub use self::error::{Error, Result};
// endregion: --- Modules

pub type Db = SqlitePool;

pub async fn new_db_pool() -> Result<Db> {
    let db_url = if cfg!(test) {
        let test_db_url = format!("sqlite://{}/db/test.db", get_env("PWD").unwrap());
        if Sqlite::database_exists(&test_db_url).await? {
            Sqlite::drop_database(&test_db_url).await?;
        }
        test_db_url
    } else {
        core_config().DB_URL.clone()
    };

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .read_only(false);

    let db = SqlitePoolOptions::new().connect_with(options).await?;
    migrate(&db).await?;
    Ok(db)
}

async fn migrate(db: &Db) -> Result<()> {
    Ok(sqlx::migrate!("sql/migrations").run(db).await?)
}
