use lib_utils::time::Rfc3339;
use modql::field::Fields;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;

// region:	  --- User Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct User {
    // -- Relations
    pub id: i64,

    // -- Properties
    pub name: String,

    // -- Timestamps
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
}

pub struct UserForCreate {
    pub plan_id: i64,
    pub name: String,
}
// endregion: --- User Types
