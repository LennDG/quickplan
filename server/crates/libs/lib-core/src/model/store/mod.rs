use crate::config::core_config;
use include_dir::{include_dir, Dir};
use lib_utils::envs::get_env;
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use std::sync::Arc;
use std::{str::FromStr, time::Duration};
use tokio::sync::Mutex;

// region:	  --- Modules
mod error;
pub use self::error::{Error, Result};
// endregion: --- Modules

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sql/migrations");

pub type Db = Arc<Mutex<Connection>>;

pub async fn new_db_pool() -> Result<Db> {
    let db_url = if cfg!(test) {
        let test_db = format!(
            "{}/{}",
            get_env("PWD").unwrap(),
            &core_config().DB_TEST_FILE
        );
        if tokio::fs::try_exists(test_db.clone()).await? {
            tokio::fs::remove_file(test_db.clone()).await?;
        }
        test_db
    } else {
        core_config().DB_FILE.clone()
    };

    let mut conn = Connection::open(db_url)?;
    conn.pragma_update(None, "journal_mode", "WAL").unwrap();
    conn.pragma_update(None, "synchronous", "normal").unwrap();
    conn.pragma_update(None, "foreign_keys", "ON").unwrap();

    migrate(&mut conn).await?;

    let db: Db = Arc::new(Mutex::new(conn));

    Ok(db)
}

pub async fn migrate(conn: &mut Connection) -> Result<()> {
    let m = Migrations::from_directory(&MIGRATIONS_DIR)?;
    m.to_latest(conn);

    Ok(())
}
