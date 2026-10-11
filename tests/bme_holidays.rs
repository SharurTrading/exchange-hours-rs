// SPDX-License-Identifier: MIT-0

//! BME's own cash calendar and principal-share half-day phases.

use chrono::{NaiveDate, TimeZone};
use chrono_tz::Europe::Madrid;
use exchange_hours::{
    CalendarQueryError, EvidenceTier, Exchange, HolidayKind, SessionKind, calendar_for_exchange,
};

fn at(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> chrono::DateTime<chrono::Utc> {
    Madrid
        .with_ymd_and_hms(year, month, day, hour, minute, second)
        .unwrap()
        .to_utc()
}

#[test]
fn each_annual_cash_calendar_closes_its_own_good_friday() {
    let calendar = calendar_for_exchange(Exchange::Bme);
    for (year, month, day) in [
        (2010, 4, 2),
        (2011, 4, 22),
        (2012, 4, 6),
        (2013, 3, 29),
        (2014, 4, 18),
        (2015, 4, 3),
        (2016, 3, 25),
        (2017, 4, 14),
        (2018, 3, 30),
        (2019, 4, 19),
        (2020, 4, 10),
        (2021, 4, 2),
        (2022, 4, 15),
        (2023, 4, 7),
        (2024, 3, 29),
        (2025, 4, 18),
        (2026, 4, 3),
    ] {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let row = calendar.holiday_on(date).unwrap();
        assert_eq!(row.kind(), HolidayKind::Closed, "{date}");
        assert_eq!(row.tier(), EvidenceTier::T1);
        assert_eq!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            Ok(true)
        );
        assert_eq!(calendar.is_open(at(year, month, day, 12, 0, 0)), Ok(false));
    }
    for year in [2010, 2021] {
        for day in [24, 31] {
            let date = NaiveDate::from_ymd_opt(year, 12, day).unwrap();
            assert_eq!(
                calendar.holiday_on(date).unwrap().kind(),
                HolidayKind::Closed
            );
        }
    }
    assert_eq!(calendar.is_open(at(2025, 12, 26, 12, 0, 0)), Ok(false));
}

#[test]
fn detailed_half_days_replace_regular_and_auction_phases() {
    let calendar = calendar_for_exchange(Exchange::Bme);
    for year in [2012, 2013, 2014, 2015, 2018, 2019, 2020, 2024, 2025] {
        for day in [24, 31] {
            let date = NaiveDate::from_ymd_opt(year, 12, day).unwrap();
            assert!(matches!(
                calendar.holiday_on(date).unwrap().kind(),
                HolidayKind::ReplacementBlocks(_)
            ));
            assert_eq!(calendar.is_open(at(year, 12, day, 8, 29, 59)), Ok(false));
            assert_eq!(calendar.is_open(at(year, 12, day, 8, 30, 0)), Ok(true));
            assert_eq!(
                calendar.is_open_regular(at(year, 12, day, 13, 54, 59)),
                Ok(true)
            );
            assert_eq!(
                calendar.is_open_regular(at(year, 12, day, 13, 55, 0)),
                Ok(false)
            );
            assert_eq!(
                calendar.is_open_extended(at(year, 12, day, 13, 55, 0)),
                Ok(true)
            );
            let close = if year >= 2024 {
                at(year, 12, day, 14, 10, 0)
            } else {
                at(year, 12, day, 14, 0, 30)
            };
            assert_eq!(
                calendar.is_open(close - chrono::Duration::seconds(1)),
                Ok(true)
            );
            assert_eq!(calendar.is_open(close), Ok(false));
            assert_eq!(calendar.is_open(at(year, 12, day, 14, 30, 0)), Ok(false));
        }
    }
}

#[test]
fn cash_calendar_has_no_invented_substitutes_and_keeps_2026_unknown() {
    let calendar = calendar_for_exchange(Exchange::Bme);
    for (year, day) in [
        (2011, 23),
        (2011, 30),
        (2016, 23),
        (2016, 30),
        (2017, 22),
        (2017, 29),
        (2022, 23),
        (2022, 30),
        (2023, 22),
        (2023, 29),
    ] {
        let date = NaiveDate::from_ymd_opt(year, 12, day).unwrap();
        assert_eq!(calendar.holiday_on(date), None);
        assert_eq!(calendar.is_open(at(year, 12, day, 16, 0, 0)), Ok(true));
    }
    for day in [24, 31] {
        let date = NaiveDate::from_ymd_opt(2026, 12, day).unwrap();
        assert_eq!(
            calendar.holiday_on(date).unwrap().kind(),
            HolidayKind::Unsourced
        );
        assert!(matches!(
            calendar.is_open(at(2026, 12, day, 12, 0, 0)),
            Err(CalendarQueryError::UnresolvedGap { .. })
        ));
    }
    assert!(matches!(
        calendar.is_open(at(2027, 1, 4, 12, 0, 0)),
        Err(CalendarQueryError::OutsideCoveredRange { .. })
    ));
}
