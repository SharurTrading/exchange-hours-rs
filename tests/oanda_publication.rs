// SPDX-License-Identifier: MIT-0

//! Source boundary regressions from OANDA's UK hours and holiday pages.

#![expect(
    clippy::expect_used,
    reason = "validated fixture literals must fail loudly"
)]

use chrono::{DateTime, Duration, NaiveDate, Utc};
use exchange_hours::{
    ExceptionBlock, ExchangeCalendar, MarketHoursKey as K, SessionExceptionRecord, SessionState,
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
