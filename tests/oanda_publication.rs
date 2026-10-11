// SPDX-License-Identifier: MIT-0

//! Source boundary regressions from OANDA's UK hours and holiday pages.

#![expect(
    clippy::expect_used,
    reason = "validated fixture literals must fail loudly"
)]

use chrono::{DateTime, Duration, NaiveDate, Utc};
use exchange_hours::{
    CalendarQueryError, DateCoverage, ExceptionBlock, ExchangeCalendar, HolidayKind,
    MarketHoursKey as K, SessionExceptionRecord, SessionKind, SessionState,
    StaticSessionExceptions,
};

fn utc(s: &str) -> DateTime<Utc> {
    s.parse().expect("valid UTC fixture")
}

fn cal(key: K) -> ExchangeCalendar {
    ExchangeCalendar::for_market_hours_key(key)
}

#[test]
fn mlk_2025_closes_at_the_operators_printed_instant() {
    for (key, close) in [
        (K::OandaUsIndexCfd, "2025-01-20T18:00:00Z"),
        (K::OandaUsBondCfd, "2025-01-20T18:00:00Z"),
        (K::OandaUsCommodityCfd, "2025-01-20T19:30:00Z"),
        (K::OandaMetals, "2025-01-20T19:30:00Z"),
    ] {
        let close = utc(close);
        assert_eq!(cal(key).is_open(close - Duration::seconds(1)), Ok(true));
        assert_eq!(cal(key).is_open(close), Ok(false));
    }
}

#[test]
fn new_year_2025_reopens_at_1804_on_the_following_trade_date() {
    for key in [
        K::OandaUsIndexCfd,
        K::OandaUsBondCfd,
        K::OandaUsCommodityCfd,
    ] {
        let calendar = cal(key);
        let open = utc("2025-01-01T23:04:00Z");
        assert_eq!(calendar.is_open(open - Duration::seconds(1)), Ok(false));
        assert_eq!(calendar.is_open(open), Ok(true));
        assert_eq!(
            calendar.trade_date(open),
            Ok(NaiveDate::from_ymd_opt(2025, 1, 2))
        );
    }
}

#[test]
fn holy_thursday_2025_replacement_extends_to_the_printed_1700_close() {
    for key in [
        K::OandaUsIndexCfd,
        K::OandaUsBondCfd,
        K::OandaUsCommodityCfd,
        K::OandaMetals,
    ] {
        let close = utc("2025-04-17T21:00:00Z");
        assert_eq!(cal(key).is_open(close - Duration::seconds(1)), Ok(true));
        assert_eq!(cal(key).is_open(close), Ok(false));
    }
}

#[test]
fn us_2025_reopens_preserve_the_printed_1800_and_next_day_early_closes() {
    for (day, close_index, close_energy) in [
        ("2025-01-20", "2025-01-21T21:59:00Z", "2025-01-21T21:59:00Z"),
        ("2025-02-17", "2025-02-18T21:59:00Z", "2025-02-18T21:59:00Z"),
        ("2025-05-26", "2025-05-27T20:59:00Z", "2025-05-27T20:59:00Z"),
        ("2025-06-19", "2025-06-20T20:59:00Z", "2025-06-20T20:59:00Z"),
        ("2025-09-01", "2025-09-02T20:59:00Z", "2025-09-02T20:59:00Z"),
        ("2025-11-27", "2025-11-28T18:15:00Z", "2025-11-28T19:45:00Z"),
    ] {
        let summer = matches!(day, "2025-05-26" | "2025-06-19" | "2025-09-01");
        let open = utc(&format!(
            "{day}T{}:00:00Z",
            if summer { "22" } else { "23" }
        ));
        for (key, close) in [
            (K::OandaUsIndexCfd, close_index),
            (K::OandaUsBondCfd, close_index),
            (K::OandaUsCommodityCfd, close_energy),
        ] {
            let calendar = cal(key);
            assert_eq!(
                calendar.is_open(open - Duration::seconds(1)),
                Ok(false),
                "{key:?} {day}"
            );
            assert_eq!(calendar.is_open(open), Ok(true), "{key:?} {day}");
            assert_eq!(calendar.session_bounds(open), Ok(Some((open, utc(close)))));
            assert_eq!(calendar.is_open(utc(close)), Ok(false));
        }
    }
    let calendar = cal(K::OandaUsIndexCfd);
    assert_eq!(
        calendar.session_bounds(utc("2025-07-03T22:00:00Z")),
        Ok(Some((
            utc("2025-07-03T22:00:00Z"),
            utc("2025-07-04T17:00:00Z")
        )))
    );
}

#[test]
fn brent_2025_reopens_at_2000_et_including_friday_early_close() {
    for (open, close) in [
        ("2025-05-27T00:00:00Z", "2025-05-27T20:59:00Z"),
        ("2025-06-20T00:00:00Z", "2025-06-20T20:59:00Z"),
        ("2025-09-02T00:00:00Z", "2025-09-02T20:59:00Z"),
        ("2025-11-28T01:00:00Z", "2025-11-28T20:00:00Z"),
    ] {
        let calendar = cal(K::OandaBrent);
        let open = utc(open);
        assert_eq!(calendar.is_open(open - Duration::seconds(1)), Ok(false));
        assert_eq!(calendar.is_open(open), Ok(true));
        assert_eq!(calendar.session_bounds(open), Ok(Some((open, utc(close)))));
        assert_eq!(calendar.is_open(utc(close)), Ok(false));
    }
}

#[test]
fn brent_monday_bounds_include_the_sunday_open_in_both_dst_regimes() {
    for (instant, open, close) in [
        (
            "2026-06-08T11:00:00Z",
            "2026-06-07T22:01:00Z",
            "2026-06-08T20:59:00Z",
        ),
        (
            "2026-01-12T11:00:00Z",
            "2026-01-11T23:01:00Z",
            "2026-01-12T21:59:00Z",
        ),
    ] {
        assert_eq!(
            cal(K::OandaBrent).session_bounds(utc(instant)),
            Ok(Some((utc(open), utc(close))))
        );
    }
}

#[test]
fn split_session_search_selects_the_nearest_reopen_before_a_later_normal_open() {
    let base = cal(K::OandaFx);
    let date = NaiveDate::from_ymd_opt(2026, 10, 12).unwrap();
    let blocks = [
        ExceptionBlock::regular(-1, 17 * 3600 + 5 * 60, 13 * 3600 + 59 * 60),
        ExceptionBlock::regular(0, 14 * 3600 + 5 * 60, 16 * 3600 + 59 * 60),
    ];
    let records = [SessionExceptionRecord::replace_sessions(date, &blocks)];
    let replacements = StaticSessionExceptions::new(base.source(), date, date, &records).unwrap();
    let calendar = base.with_session_exceptions(&replacements).unwrap();
    assert_eq!(
        calendar.next_session_open_after(utc("2026-10-12T18:00:00Z")),
        Ok(Some(utc("2026-10-12T18:05:00Z")))
    );
}

#[test]
fn bonds_have_their_own_good_friday_and_carter_day_closes() {
    for (key, close) in [
        (K::OandaUsIndexCfd, "2026-04-03T13:14:00Z"),
        (K::OandaUsBondCfd, "2026-04-03T15:14:00Z"),
        (K::OandaUsBondCfd, "2025-01-09T18:15:00Z"),
    ] {
        let close = utc(close);
        assert_eq!(cal(key).is_open(close - Duration::seconds(1)), Ok(true));
        assert_eq!(cal(key).is_open(close), Ok(false));
    }
    let calendar = cal(K::OandaUsBondCfd);
    let reopen = utc("2025-01-09T23:00:00Z");
    assert_eq!(calendar.is_open(reopen - Duration::seconds(1)), Ok(false));
    assert_eq!(calendar.is_open(reopen), Ok(true));
    assert_eq!(
        calendar.session_bounds(reopen),
        Ok(Some((reopen, utc("2025-01-10T21:59:00Z"))))
    );
    assert_eq!(K::OandaUsBondCfd.as_str().parse(), Ok(K::OandaUsBondCfd));
}

#[test]
fn bonds_do_not_inherit_the_index_only_independence_eve_arrangement() {
    let bonds = cal(K::OandaUsBondCfd);
    let indices = cal(K::OandaUsIndexCfd);
    let index_close = utc("2025-07-03T17:15:00Z");
    assert_eq!(indices.is_open(index_close), Ok(false));
    assert_eq!(bonds.is_open(index_close), Ok(true));
    assert_eq!(
        bonds.session_bounds(index_close),
        Ok(Some((
            utc("2025-07-02T22:01:00Z"),
            utc("2025-07-03T20:59:00Z")
        )))
    );
    let reopen = utc("2025-07-03T22:01:00Z");
    assert_eq!(bonds.is_open(reopen - Duration::seconds(1)), Ok(false));
    assert_eq!(bonds.is_open(reopen), Ok(true));
    assert_eq!(
        bonds.session_bounds(reopen),
        Ok(Some((reopen, utc("2025-07-04T17:00:00Z"))))
    );
}

#[test]
fn carters_close_and_reopen_follow_the_dated_twenty_four_hour_context() {
    let calendar = cal(K::OandaUsIndexCfd);
    let close = utc("2025-01-09T14:30:00Z");
    assert_eq!(calendar.is_open(close - Duration::seconds(1)), Ok(true));
    assert_eq!(calendar.is_open(close), Ok(false));
    let reopen = utc("2025-01-09T23:00:00Z");
    assert_eq!(calendar.is_open(reopen - Duration::seconds(1)), Ok(false));
    assert_eq!(calendar.is_open(reopen), Ok(true));
    assert_eq!(
        calendar.session_bounds(utc("2025-01-10T12:00:00Z")),
        Ok(Some((reopen, utc("2025-01-10T21:59:00Z"))))
    );
}

#[test]
fn an_incomplete_memorial_reopen_refuses_the_following_trade_date() {
    let date = NaiveDate::from_ymd_opt(2026, 5, 26).unwrap();
    for key in [K::OandaUsIndexCfd, K::OandaUsBondCfd] {
        let calendar = cal(key);
        assert_eq!(
            calendar.coverage().coverage_on(date),
            DateCoverage::UnresolvedGap
        );
        let memorial_day = NaiveDate::from_ymd_opt(2026, 5, 25).unwrap();
        assert_eq!(
            calendar.holiday_on(memorial_day).unwrap().kind(),
            HolidayKind::EarlyClose {
                close_ssm: 12 * 3_600 + 59 * 60
            }
        );
        for instant in [
            "2026-05-25T16:58:59Z",
            "2026-05-25T22:01:00Z",
            "2026-05-26T12:00:00Z",
        ] {
            let instant = utc(instant);
            assert!(matches!(calendar.is_open(instant),
                Err(CalendarQueryError::UnresolvedGap { date: withheld, .. }) if withheld == date
            ));
            assert!(matches!(calendar.session_bounds(instant),
                Err(CalendarQueryError::UnresolvedGap { date: withheld, .. }) if withheld == date
            ));
        }
    }
}

#[test]
fn previous_session_search_uses_the_last_replacement_close_for_gap_state() {
    let base = cal(K::OandaFx);
    let date = NaiveDate::from_ymd_opt(2026, 10, 12).unwrap();
    let blocks = [
        ExceptionBlock::regular(0, 0, 3_600),
        ExceptionBlock::regular(0, 15 * 3_600, 16 * 3_600 + 59 * 60),
    ];
    let records = [SessionExceptionRecord::replace_sessions(date, &blocks)];
    let replacements = StaticSessionExceptions::new(base.source(), date, date, &records).unwrap();
    let calendar = base.with_session_exceptions(&replacements).unwrap();
    let gap = utc("2026-10-12T21:00:00Z");
    assert_eq!(calendar.session_state(gap), Ok(SessionState::Maintenance));
    assert_eq!(calendar.is_maintenance(gap), Ok(true));
}

#[test]
fn spot_metals_holiday_reopens_use_the_literal_group_and_gold_silver_exception() {
    let calendar = cal(K::OandaMetals);
    for open in [
        "2025-01-01T23:04:00Z",
        "2025-01-20T23:00:00Z",
        "2025-02-17T23:00:00Z",
        "2025-05-26T22:00:00Z",
        "2025-06-19T22:00:00Z",
        "2025-09-01T22:00:00Z",
        "2025-11-27T23:00:00Z",
        "2025-12-25T23:01:00Z",
        "2026-01-01T23:01:00Z",
        "2026-01-19T23:01:00Z",
        "2026-02-16T23:01:00Z",
        "2026-05-25T22:05:00Z",
        "2026-09-07T22:01:00Z",
    ] {
        let open = utc(open);
        assert_eq!(
            calendar.is_open(open - Duration::seconds(1)),
            Ok(false),
            "{open}"
        );
        assert_eq!(calendar.is_open(open), Ok(true), "{open}");
        assert_eq!(
            calendar
                .session_bounds(open)
                .expect("covered bounds")
                .unwrap()
                .0,
            open
        );
    }
    let close = utc("2025-11-28T19:45:00Z");
    assert_eq!(calendar.is_open(close - Duration::seconds(1)), Ok(true));
    assert_eq!(calendar.is_open(close), Ok(false));
}

#[test]
fn archived_2024_us_holidays_preserve_literal_reopens_and_friday_close() {
    for key in [K::OandaUsIndexCfd, K::OandaUsBondCfd] {
        let calendar = cal(key);
        for (close, reopen, end) in [
            (
                "2024-05-27T17:00:00Z",
                "2024-05-27T22:00:00Z",
                "2024-05-28T20:59:00Z",
            ),
            (
                "2024-06-19T17:00:00Z",
                "2024-06-19T22:00:00Z",
                "2024-06-20T20:59:00Z",
            ),
            (
                "2024-09-02T17:00:00Z",
                "2024-09-02T22:00:00Z",
                "2024-09-03T20:59:00Z",
            ),
            (
                "2024-11-28T18:00:00Z",
                "2024-11-28T23:00:00Z",
                "2024-11-29T18:15:00Z",
            ),
        ] {
            let close = utc(close);
            let reopen = utc(reopen);
            assert_eq!(calendar.is_open(close - Duration::seconds(1)), Ok(true));
            assert_eq!(calendar.is_open(close), Ok(false));
            assert_eq!(calendar.is_open(reopen - Duration::seconds(1)), Ok(false));
            assert_eq!(calendar.is_open(reopen), Ok(true));
            assert_eq!(
                calendar.session_bounds(reopen),
                Ok(Some((reopen, utc(end))))
            );
            assert_eq!(calendar.is_open(utc(end)), Ok(false));
        }
        assert_eq!(
            calendar.is_closed_trade_date(
                NaiveDate::from_ymd_opt(2024, 3, 29).unwrap(),
                SessionKind::Both
            ),
            Ok(true)
        );
        let december = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        assert!(calendar.holiday_on(december).is_none());
    }
}

#[test]
fn archived_2024_independence_eve_names_indices_without_changing_bonds() {
    let index = cal(K::OandaUsIndexCfd);
    let bonds = cal(K::OandaUsBondCfd);
    let close = utc("2024-07-03T17:15:00Z");
    assert_eq!(index.is_open(close - Duration::seconds(1)), Ok(true));
    assert_eq!(index.is_open(close), Ok(false));
    assert_eq!(bonds.is_open(close), Ok(true));
    for (calendar, open) in [
        (index, "2024-07-03T22:00:00Z"),
        (bonds, "2024-07-03T22:01:00Z"),
    ] {
        let open = utc(open);
        assert_eq!(calendar.is_open(open - Duration::seconds(1)), Ok(false));
        assert_eq!(calendar.is_open(open), Ok(true));
        assert_eq!(
            calendar.session_bounds(open),
            Ok(Some((open, utc("2024-07-04T17:00:00Z"))))
        );
        let reopen = utc("2024-07-04T22:00:00Z");
        assert_eq!(calendar.is_open(reopen - Duration::seconds(1)), Ok(false));
        assert_eq!(calendar.is_open(reopen), Ok(true));
        assert_eq!(
            calendar.session_bounds(reopen),
            Ok(Some((reopen, utc("2024-07-05T20:59:00Z"))))
        );
    }
}

#[test]
fn archived_2024_christmas_keeps_1804_reopens_and_the_missing_bond_close_explicit() {
    let index = cal(K::OandaUsIndexCfd);
    let close = utc("2024-12-24T18:14:00Z");
    assert_eq!(index.is_open(close - Duration::seconds(1)), Ok(true));
    assert_eq!(index.is_open(close), Ok(false));
    let open = utc("2024-12-25T23:04:00Z");
    assert_eq!(index.is_open(open - Duration::seconds(1)), Ok(false));
    assert_eq!(index.is_open(open), Ok(true));
    assert_eq!(
        index.session_bounds(open),
        Ok(Some((open, utc("2024-12-26T21:59:00Z"))))
    );
    let bonds = cal(K::OandaUsBondCfd);
    let date = NaiveDate::from_ymd_opt(2024, 12, 24).unwrap();
    assert_eq!(
        bonds.holiday_on(date).unwrap().kind(),
        HolidayKind::Unsourced
    );
    assert!(
        matches!(bonds.is_open(close), Err(CalendarQueryError::UnresolvedGap { date: withheld, .. }) if withheld == date)
    );
    let date = NaiveDate::from_ymd_opt(2024, 12, 26).unwrap();
    assert_eq!(
        bonds.holiday_on(date).unwrap().kind(),
        HolidayKind::LateOpen {
            open_ssm: 18 * 3_600 + 4 * 60
        }
    );
}

#[test]
fn archived_2024_pre_witness_reopens_do_not_invent_final_closes() {
    for key in [K::OandaUsIndexCfd, K::OandaUsBondCfd] {
        let calendar = cal(key);
        // The UK holiday sections give 18:00 reopens, but no final close.
        // The first normal-grid witness is later, on 2024-02-21.
        for (month, day) in [(1, 2), (1, 16), (2, 20)] {
            let date = NaiveDate::from_ymd_opt(2024, month, day).unwrap();
            assert_eq!(
                calendar.holiday_on(date).unwrap().kind(),
                HolidayKind::Unsourced
            );
            assert!(
                calendar
                    .is_closed_trade_date(date, SessionKind::Both)
                    .is_err()
            );
        }
        // Printed holiday scalar closes remain known independently of that gap.
        for (month, day) in [(1, 15), (2, 19)] {
            let date = NaiveDate::from_ymd_opt(2024, month, day).unwrap();
            assert_eq!(
                calendar.holiday_on(date).unwrap().kind(),
                HolidayKind::EarlyClose {
                    close_ssm: 13 * 3_600
                }
            );
        }
    }
}

#[test]
fn archived_2024_us_commodities_keep_their_own_opening_and_literal_holidays() {
    let calendar = cal(K::OandaUsCommodityCfd);
    for (open, close) in [
        ("2024-05-27T22:00:00Z", "2024-05-28T20:59:00Z"),
        ("2024-06-19T22:00:00Z", "2024-06-20T20:59:00Z"),
        ("2024-07-04T22:00:00Z", "2024-07-05T20:59:00Z"),
        ("2024-09-02T22:00:00Z", "2024-09-03T20:59:00Z"),
        ("2024-11-28T23:00:00Z", "2024-11-29T19:45:00Z"),
        ("2024-12-25T23:04:00Z", "2024-12-26T21:59:00Z"),
    ] {
        let open = utc(open);
        let close = utc(close);
        assert_eq!(calendar.is_open(open - Duration::seconds(1)), Ok(false));
        assert_eq!(calendar.is_open(open), Ok(true));
        assert_eq!(calendar.session_bounds(open), Ok(Some((open, close))));
        assert_eq!(calendar.is_open(close), Ok(false));
    }
    let close = utc("2024-12-24T18:44:00Z");
    assert_eq!(calendar.is_open(close - Duration::seconds(1)), Ok(true));
    assert_eq!(calendar.is_open(close), Ok(false));
    let normal = utc("2024-07-03T22:01:00Z");
    assert_eq!(calendar.is_open(normal), Ok(true));
    let date = NaiveDate::from_ymd_opt(2024, 12, 26).unwrap();
    assert_eq!(
        calendar.holiday_on(date).unwrap().kind(),
        HolidayKind::LateOpen {
            open_ssm: 18 * 3_600 + 4 * 60
        }
    );
}

#[test]
fn archived_2024_brent_preserves_explicit_thanksgiving_and_missing_reopens() {
    let calendar = cal(K::OandaBrent);
    let open = utc("2024-11-29T01:00:00Z");
    let close = utc("2024-11-29T20:00:00Z");
    assert_eq!(calendar.is_open(open - Duration::seconds(1)), Ok(false));
    assert_eq!(calendar.is_open(open), Ok(true));
    assert_eq!(calendar.session_bounds(open), Ok(Some((open, close))));
    assert_eq!(calendar.is_open(close), Ok(false));
    for (month, day) in [(4, 1), (5, 28), (6, 20), (7, 3), (7, 5), (8, 26), (9, 3)] {
        let date = NaiveDate::from_ymd_opt(2024, month, day).unwrap();
        assert_eq!(
            calendar.holiday_on(date).unwrap().kind(),
            HolidayKind::Unsourced
        );
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .is_err()
        );
    }
    for (month, day) in [(5, 27), (6, 19), (7, 4), (9, 2)] {
        let date = NaiveDate::from_ymd_opt(2024, month, day).unwrap();
        assert_eq!(
            calendar.holiday_on(date).unwrap().kind(),
            HolidayKind::EarlyClose {
                close_ssm: 18 * 3_600 + 30 * 60
            }
        );
    }
    for (month, day) in [(5, 1), (5, 6)] {
        let date = NaiveDate::from_ymd_opt(2024, month, day).unwrap();
        assert_eq!(calendar.holiday_on(date), None);
        assert_eq!(
            calendar.is_open(utc(if day == 1 {
                "2024-05-01T12:00:00Z"
            } else {
                "2024-05-06T12:00:00Z"
            })),
            Ok(true)
        );
    }
}
