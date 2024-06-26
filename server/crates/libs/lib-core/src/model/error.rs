use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use super::{fields::WebId, store};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },

    PlanUrlNotFound {
        url_id: String,
    },

    UserWebIdNotFound {
        web_id: WebId,
    },

    // -- Modules
    #[from]
    Store(store::Error),

    // -- Externals
    #[from]
    Rusqlite(#[serde_as(as = "DisplayFromStr")] rusqlite::Error),
    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
