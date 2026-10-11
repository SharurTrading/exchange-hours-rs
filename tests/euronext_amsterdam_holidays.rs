// SPDX-License-Identifier: MIT-0

//! Amsterdam's own holiday calendar, read from the operator's cash calendars.

use chrono::{NaiveDate, TimeZone};
use chrono_tz::Europe::Amsterdam;
use exchange_hours::{
    CalendarQueryError, EvidenceTier, Exchange, HolidayKind, SessionKind, calendar_for_exchange,
};

#[test]
fn annual_closures_and_half_day_eras_use_amsterdams_operator_scope() {
    let calendar = calendar_for_exchange(Exchange::EuronextAmsterdam);
    // Annual operator editions, including Amsterdam's explicit column in 2022+.
    for (year, month, day) in [
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
        let holiday = calendar.holiday_on(date).unwrap();
        assert_eq!(holiday.kind(), HolidayKind::Closed, "{date}");
        assert_eq!(holiday.tier(), EvidenceTier::T1);
        assert_eq!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            Ok(true)
        );
        let noon = Amsterdam
            .with_ymd_and_hms(year, month, day, 12, 0, 0)
            .unwrap()
            .to_utc();
        assert_eq!(calendar.is_open(noon), Ok(false));
    }
    // The independent Dutch AEX appendix rows (2010/2024/2025) end TAL at 14:05.
    for (year, month, day, hour, minute) in [
        (2010, 12, 24, 14, 5),
        (2011, 12, 23, 17, 35),
        (2012, 12, 24, 14, 0),
        (2013, 12, 24, 14, 0),
        (2014, 12, 24, 14, 5),
        (2024, 12, 24, 14, 5),
        (2025, 12, 31, 14, 5),
    ] {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let close = hour * 3_600 + minute * 60;
        assert_eq!(
            calendar.holiday_on(date).unwrap().kind(),
            HolidayKind::EarlyClose { close_ssm: close }
        );
        let at = Amsterdam
            .with_ymd_and_hms(year, month, day, hour, minute, 0)
            .unwrap()
            .to_utc();
        let before = at - chrono::Duration::seconds(1);
        assert_eq!(calendar.is_open(before), Ok(true));
        assert_eq!(calendar.is_open(at), Ok(false));
    }
}

#[test]
fn amsterdam_does_not_inherit_dublin_substitute_half_days() {
    let calendar = calendar_for_exchange(Exchange::EuronextAmsterdam);
    for (year, month, day) in [
        (2022, 12, 23),
        (2022, 12, 30),
        (2023, 12, 22),
        (2023, 12, 29),
    ] {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(calendar.holiday_on(date), None);
        let afternoon = Amsterdam
            .with_ymd_and_hms(year, month, day, 16, 0, 0)
            .unwrap()
            .to_utc();
        assert_eq!(calendar.is_open(afternoon), Ok(true));
    }
}

#[test]
fn announced_2026_half_days_remain_typed_gaps() {
    let calendar = calendar_for_exchange(Exchange::EuronextAmsterdam);
    for day in [24, 31] {
        let date = NaiveDate::from_ymd_opt(2026, 12, day).unwrap();
        assert_eq!(
            calendar.holiday_on(date).unwrap().kind(),
            HolidayKind::Unsourced
        );
        let noon = Amsterdam
            .with_ymd_and_hms(2026, 12, day, 12, 0, 0)
            .unwrap()
            .to_utc();
        assert!(matches!(
            calendar.is_open(noon),
            Err(CalendarQueryError::UnresolvedGap { .. })
        ));
    }
}
