use lib_utils::time::now_utc;
use modql::field::{Field, Fields};
use sea_query::IntoIden;

use super::{DbBmc, TimestampIden};

/// This method is called when the model controller will create the entity
pub fn prep_fields_for_create<MC>(fields: &mut Fields)
where
    MC: DbBmc,
{
    if MC::has_creation_timestamp() {
        add_timestamp_for_create(fields);
    }
}

/// Update the timestamps info for create
/// (e.g., cid, ctime, and mid, mtime will be updated with the same values)
fn add_timestamp_for_create(fields: &mut Fields) {
    let now = now_utc();
    fields.push(Field::new(TimestampIden::Ctime.into_iden(), now.into()));
}
