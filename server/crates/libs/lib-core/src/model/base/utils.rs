use lib_utils::time::now_utc;
use modql::field::{SeaField, SeaFields};
use sea_query::IntoIden;
use time::OffsetDateTime;

use crate::model::fields::{Timestamp, TimestampIden};

use super::DbBmc;

/// This method is called when the model controller will create the entity
pub fn prep_fields_for_create<MC>(fields: &mut SeaFields)
where
    MC: DbBmc,
{
    if MC::has_creation_timestamp() {
        add_timestamp_for_create(fields);
    }
}

/// Update the timestamps info for create
/// (e.g., cid, ctime, and mid, mtime will be updated with the same values)
fn add_timestamp_for_create(fields: &mut SeaFields) {
    let now = Timestamp::now();
    fields.push(SeaField::new(TimestampIden::Ctime.into_iden(), now.into()));
}
