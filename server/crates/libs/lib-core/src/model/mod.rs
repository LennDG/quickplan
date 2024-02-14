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
}
