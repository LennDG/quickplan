use lib_utils::time::{format_time, now_utc};
use rusqlite::types::{FromSql, FromSqlError};
use sea_query::Iden;
use serde::Deserialize;
use time::{macros::format_description, Date, OffsetDateTime};

// region:	  --- Timestamp
#[derive(Debug, Clone)]
pub struct Timestamp(OffsetDateTime);

#[derive(Iden)]
pub enum TimestampIden {
    Ctime,
}

impl Timestamp {
    pub fn now() -> Self {
        Self(now_utc())
    }
}

impl FromSql for Timestamp {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                let s = std::str::from_utf8(t).map_err(|_| FromSqlError::InvalidType)?;
                let datetime = lib_utils::time::parse_utc(s)
                    .map_err(|err| FromSqlError::Other(Box::new(err)))?;
                Ok(Self(datetime))
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl From<Timestamp> for sea_query::Value {
    fn from(Timestamp(odt): Timestamp) -> Self {
        sea_query::Value::String(Some(Box::new(format_time(odt))))
    }
}
// endregion: --- Timestamp

// region:	  --- ModelDate
#[derive(Debug, Clone, Deserialize)]
pub struct ModelDate(Date);

impl ModelDate {
    pub fn new(date: Date) -> Self {
        Self(date)
    }

    pub fn date(&self) -> Date {
        self.0
    }
}

impl FromSql for ModelDate {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                let s = std::str::from_utf8(t).map_err(|_| FromSqlError::InvalidType)?;
                let date = Date::parse(s, &format_description!("[year]-[month]-[day]"))
                    .map_err(|_| FromSqlError::InvalidType)?;
                Ok(Self(date))
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl From<ModelDate> for sea_query::Value {
    fn from(ModelDate(date): ModelDate) -> Self {
        sea_query::Value::String(Some(Box::new(date.to_string())))
    }
}
// endregion: --- ModelDate
