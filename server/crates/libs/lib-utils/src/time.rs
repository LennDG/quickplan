use time::{Date, Duration, OffsetDateTime};

pub use time::format_description::well_known::Rfc3339;

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn current_date() -> Date {
    OffsetDateTime::now_utc().date()
}

pub fn date_minus_two() -> Date {
    current_date() - Duration::days(2)
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap()
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::DateFailParse(moment.to_string()))
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
    DateFailParse(String),
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
