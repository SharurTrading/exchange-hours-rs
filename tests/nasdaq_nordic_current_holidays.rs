// SPDX-License-Identifier: MIT-0

//! Nordic principal-share calendar scopes and current/future half-day boundaries.

use chrono::{NaiveDate, TimeZone};
use chrono_tz::Europe::Stockholm;
use exchange_hours::{Exchange, HolidayKind, SessionKind, calendar_for_exchange};

#[test]
fn cash_calendars_keep_their_own_closures_and_publication_horizon() {
    for (exchange, count, extra) in [
        (Exchange::NasdaqCopenhagen, 20, (2026, 6, 5)),
        (Exchange::NasdaqHelsinki, 19, (2027, 12, 6)),
        (Exchange::NasdaqStockholm, 28, (2026, 1, 6)),
    ] {
        let cal = calendar_for_exchange(exchange);
        let mut observed = 0;
        let mut date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2027, 12, 31).unwrap();
        while date <= end {
            if cal.holiday_on(date).is_some() {
                observed += 1;
            }
            date = date.succ_opt().unwrap();
        }
        assert_eq!(observed, count);
        for (y, m, d) in [(2026, 4, 3), (2027, 3, 26), extra] {
            let date = NaiveDate::from_ymd_opt(y, m, d).unwrap();
            assert_eq!(cal.holiday_on(date).unwrap().kind(), HolidayKind::Closed);
            assert_eq!(cal.is_closed_trade_date(date, SessionKind::Both), Ok(true));
        }
        assert!(
            cal.is_closed_trade_date(
                NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
                SessionKind::Both
            )
            .is_err()
        );
        assert!(
            cal.is_closed_trade_date(
                NaiveDate::from_ymd_opt(2028, 1, 3).unwrap(),
                SessionKind::Both
            )
            .is_err()
        );
    }
    let sto = calendar_for_exchange(Exchange::NasdaqStockholm);
    assert!(
        sto.holiday_on(NaiveDate::from_ymd_opt(2027, 12, 6).unwrap())
            .is_none()
    );
    assert!(
        sto.holiday_on(NaiveDate::from_ymd_opt(2026, 6, 5).unwrap())
            .is_none()
    );
}

#[test]
fn stockholm_half_days_end_their_own_phases_at_operator_boundaries() {
    let cal = calendar_for_exchange(Exchange::NasdaqStockholm);
    for (y, m, d) in [
        (2026, 1, 5),
        (2026, 4, 2),
        (2026, 4, 30),
        (2026, 5, 13),
        (2026, 10, 30),
        (2027, 1, 5),
        (2027, 3, 25),
        (2027, 4, 30),
        (2027, 5, 5),
        (2027, 11, 5),
    ] {
        let at = |h, min, sec| {
            Stockholm
                .with_ymd_and_hms(y, m, d, h, min, sec)
                .unwrap()
                .to_utc()
        };
        assert!(matches!(
            cal.holiday_on(NaiveDate::from_ymd_opt(y, m, d).unwrap())
                .unwrap()
                .kind(),
            HolidayKind::ReplacementBlocks(_)
        ));
        assert_eq!(cal.is_open(at(7, 59, 59)), Ok(false));
        assert_eq!(cal.is_open(at(8, 0, 0)), Ok(true));
        assert_eq!(cal.is_open_regular(at(9, 0, 4)), Ok(false));
        assert_eq!(cal.is_open_regular(at(9, 0, 5)), Ok(true));
        assert_eq!(cal.is_open_regular(at(12, 54, 59)), Ok(true));
        assert_eq!(cal.is_open_regular(at(12, 55, 0)), Ok(false));
        assert_eq!(cal.is_open(at(12, 59, 59)), Ok(true));
        assert_eq!(cal.is_open(at(13, 0, 0)), Ok(true));
        assert_eq!(cal.is_open(at(13, 29, 59)), Ok(true));
        assert_eq!(cal.is_open(at(13, 30, 0)), Ok(false));
        assert_eq!(cal.is_open(at(14, 0, 0)), Ok(false));
    }
    // A fixed-income-only half day cannot change the principal-share clock.
    assert_eq!(
        cal.is_open(
            Stockholm
                .with_ymd_and_hms(2026, 6, 18, 15, 0, 0)
                .unwrap()
                .to_utc()
        ),
        Ok(true)
    );
}
