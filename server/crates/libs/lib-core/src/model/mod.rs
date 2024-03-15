//! Model Layer
//!
//! - Model layer normalizes the app data type structures
//!   and access
//! - All Application code data access goes through the model layer
//! - ModelManager holds internal states/resources needed by the
//!   ModelControllers to access data (db, redis, ...)
//! - Model Controllers implement the CRUD and other data access methods
//! - Framework like Axum uses the ModelManager as an AppState
//! - ModelManager is designed to be passed as argument to Model Controllers.

// region:    --- Modules
mod base;
mod error;
mod store;

pub mod plan;
pub mod user;
pub mod user_date;

use std::{fs, path::PathBuf};

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};
// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    // sqlx db pool reference only for model layer.
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }

    // DEV: seed dev database
    pub async fn dev_seed(&self) {
        const SEED_SQL: &str = "crates/libs/lib-core/sql/dev_init/0001_dev_seed.sql";

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

        let sql = base_dir.join(SEED_SQL);

        let content = fs::read_to_string(sql).unwrap();

        sqlx::query(&content).execute(&self.db).await.unwrap();
    }
}
