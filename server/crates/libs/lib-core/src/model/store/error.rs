use std::io;

use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    FailToCreatePool(String),
    FailToCreateDb(String),

    // -- External Modules
    #[from]
    Rusqlite(#[serde_as(as = "DisplayFromStr")] rusqlite::Error),
    #[from]
    RusqliteMigrations(#[serde_as(as = "DisplayFromStr")] rusqlite_migration::Error),
    #[from]
    Io(#[serde_as(as = "DisplayFromStr")] io::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
