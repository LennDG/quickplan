use serde::{Deserialize, Serialize};
use time::{Date, Month, Weekday};

fn get_dates_for_month(month: Month, year: i32) -> Vec<Date> {
    (1..32)
        .map(|day| Date::from_calendar_date(year, month, day))
        .take_while(|date_result| date_result.is_ok())
        .filter_map(|date| date.ok()) // Basically just unwraps by throwing away errors, but errors have already been removed by the take_while
        .collect()
}

// This adds the dates of the previous and next months until
// the first day is a monday and the last day a sunday
// for fitting on the calendar
fn pad_dates(mut dates: Vec<Date>) -> Vec<Date> {
    if dates.is_empty() {
        return dates;
    }
    let mut padded = vec![];

    // Pad dates backwards until the first day is Monday
    let mut first = *dates.first().unwrap();
    while first.weekday() != Weekday::Monday {
        if let Some(previous_day) = first.previous_day() {
            first = previous_day;
            padded.insert(0, first);
        } else {
            break;
        }
    }

    // Append the existing dates to the left padding
    padded.extend_from_slice(&dates);

    // Pad the dates forward until the last day is Sunday
    let mut last = *dates.last().unwrap();
    while last.weekday() != Weekday::Sunday {
        if let Some(next_day) = last.next_day() {
            last = next_day;
            padded.push(last);
        } else {
            break;
        }
    }

    padded
}

pub fn calender_month_dates(month: Month, year: i32) -> Vec<Date> {
    pad_dates(get_dates_for_month(month, year))
}

pub fn next_calendar_month(month: &Month, year: i32) -> (Month, i32) {
    match month {
        Month::December => (Month::January, year + 1),
        _ => (month.next(), year),
    }
}

pub fn previous_calendar_month(month: &Month, year: i32) -> (Month, i32) {
    match month {
        Month::January => (Month::December, year - 1),
        _ => (month.previous(), year),
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_pad_dates() -> Result<()> {
        let feb_2024_dates = pad_dates(get_dates_for_month(Month::February, 2024));
        assert_eq!(feb_2024_dates.len(), 35);
        assert_eq!(feb_2024_dates.first().unwrap().weekday(), Weekday::Monday);
        assert_eq!(feb_2024_dates.last().unwrap().weekday(), Weekday::Sunday);

        let feb_2027_dates = pad_dates(get_dates_for_month(Month::February, 2027));
        assert_eq!(feb_2027_dates.len(), 28);
        assert_eq!(feb_2027_dates.first().unwrap().weekday(), Weekday::Monday);
        assert_eq!(feb_2027_dates.last().unwrap().weekday(), Weekday::Sunday);

        let jan_dates = pad_dates(get_dates_for_month(Month::January, 2024));
        assert_eq!(jan_dates.len(), 35);
        assert_eq!(jan_dates.first().unwrap().weekday(), Weekday::Monday);
        assert_eq!(jan_dates.last().unwrap().weekday(), Weekday::Sunday);

        let april_dates = pad_dates(get_dates_for_month(Month::April, 2024));
        assert_eq!(april_dates.len(), 35);
        assert_eq!(april_dates.first().unwrap().weekday(), Weekday::Monday);
        assert_eq!(april_dates.last().unwrap().weekday(), Weekday::Sunday);

        Ok(())
    }

    #[test]
    fn test_get_dates_for_month() -> Result<()> {
        let feb_2024_dates = get_dates_for_month(Month::February, 2024);
        assert_eq!(feb_2024_dates.len(), 29);

        let jan_dates = get_dates_for_month(Month::January, 2024);
        assert_eq!(jan_dates.len(), 31);

        let april_dates = get_dates_for_month(Month::April, 2024);
        assert_eq!(april_dates.len(), 30);
        assert_eq!(april_dates[0].weekday(), Weekday::Monday);

        Ok(())
    }
}
// endregion: --- Tests
