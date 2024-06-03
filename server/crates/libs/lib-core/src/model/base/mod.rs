// region:	  --- Modules

pub mod crud_fns;
pub mod utils;

use modql::SIden;
use sea_query::{Iden, IntoIden, TableRef};

use super::ModelManager;

// endregion: --- Modules

// region:    --- SeaQuery Idens

#[derive(Iden)]
pub enum CommonIden {
    Id,
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

    fn has_web_id() -> bool {
        false
    }
}
