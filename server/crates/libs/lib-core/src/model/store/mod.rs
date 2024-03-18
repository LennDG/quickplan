use crate::config::core_config;
use lib_utils::envs::get_env;
use rusqlite::Connection;
use std::sync::Arc;
use std::{str::FromStr, time::Duration};
use tokio::sync::Mutex;

// region:	  --- Modules
mod error;
pub use self::error::{Error, Result};
// endregion: --- Modules

// static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sql/migrations");
// static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sql/migrations");

pub type Db = Arc<Mutex<Connection>>;

pub async fn new_db_pool() -> Result<Db> {
    let db_url = if cfg!(test) {
        let test_db = &core_config().DB_TEST_FILE;
        tokio::fs::remove_file(test_db).await?;
        test_db
    } else {
        let db = &core_config().DB_FILE;
        db
    };

    let conn = Connection::open(db_url)?;
    //let conn = Connection::open_in_memory()?;
    conn.pragma_update(None, "journal_mode", "WAL").unwrap();
    conn.pragma_update(None, "synchronous", "normal").unwrap();

    let db: Db = Arc::new(Mutex::new(conn));
    create_schema(&db).await?;
    //migrate(&db).await?;
    Ok(db)
}

pub async fn create_schema(db: &Db) -> Result<()> {
    db.lock().await.execute(
        include_str!("../../../sql/migrations/0001_create_schema.sql"),
        (),
    )?;
    Ok(())
}
