use time::{Date, OffsetDateTime};

pub use time::format_description::well_known::Rfc3339;

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn current_date() -> Date {
    OffsetDateTime::now_utc().date()
}

pub fn format_time(time: OffsetDateTime) -> String {
    // Safe to unwrap because using a well known description
    time.format(&Rfc3339).unwrap()
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::UtcFailParse(moment.to_string()))
}

pub fn time_since_ms(tic: OffsetDateTime) -> f64 {
    let duration = now_utc() - tic;

    // duration_ms in milliseconds with microseconds precision.
    (duration.as_seconds_f64() * 1_000_000.).floor() / 1_000.
}

// region:	  --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TimeFormatFail(String),
    UtcFailParse(String),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion: --- Error

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_utc_ok() -> Result<()> {
        // -- Setup & Fixtures
        //let date_fx = Date::from_calendar_date(2024, time::Month::September, 5)?;
        let datetime_str_fx = format_time(now_utc());

        // -- Exec
        let date = parse_utc(&datetime_str_fx)?;

        // -- Check
        assert_eq!(format_time(date), datetime_str_fx);

        Ok(())
    }
}
// endregion: --- Tests
