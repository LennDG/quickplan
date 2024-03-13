use crate::ctx::Ctx;
use crate::model::ModelManager;
use lib_utils::envs::get_env;
use lib_utils::time;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

type Db = SqlitePool;

// sql files
const SQL_DIR: &str = "sql/dev_init";

pub async fn init_dev_db(db_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

    // -- Get the sql_dir
    // Note: This is because cargo test and cargo run won't give the same
    //       current_dir given the worspace layout.
    let current_dir = std::env::current_dir().unwrap();
    let v: Vec<_> = current_dir.components().collect();
    let path_comp = v.get(v.len().wrapping_sub(3));
    let base_dir = if Some(true) == path_comp.map(|c| c.as_os_str() == "crates") {
        v[..v.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };

    let sql_dir = base_dir.join(SQL_DIR);

    // -- Get sql files.
    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // -- SQL Execute each file.
    let sqlite_dev_url = &format!("sqlite://{}/db/{}.db", get_env("PWD")?, db_name);
    let app_db = new_db(sqlite_dev_url).await?;

    for path in paths {
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".sql") {
            pexec(&app_db, &path).await?;
        }
    }

    Ok(())
}

async fn pexec(db: &Db, file: &Path) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");

    // -- Read the file.
    let content = fs::read_to_string(file)?;

    // FIXME: Make the split more sql proof.
    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await.map_err(|e| {
            println!("pexec error while running:\n{sql}");
            println!("cause:\n{e}");
            e
        })?;
    }

    Ok(())
}

async fn new_db(db_con_url: &str) -> Result<Db, sqlx::Error> {
    if Sqlite::database_exists(db_con_url).await? {
        Sqlite::drop_database(db_con_url).await?
    }

    Sqlite::create_database(db_con_url).await?;

    let options = SqliteConnectOptions::from_str(db_con_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .read_only(false);

    SqlitePoolOptions::new().connect_with(options).await
}
