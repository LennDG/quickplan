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

pub mod fields;
pub mod plan;
pub mod user;
pub mod user_date;

use tracing::info;

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
        const SEED_SQL: &str = include_str!("../../sql/dev_init/dev_seed.sql");

        let db = self.db().lock().await;
        let stmt = db.execute_batch(SEED_SQL);
        match stmt {
            Ok(_) => (),
            Err(_) => info!(
                "{:<12} - Dev Seed not succesful, DB may be in weird state",
                "DEV_SEED"
            ),
        }
    }
}
