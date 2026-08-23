// SPDX-License-Identifier: MIT-0

//! The [`SessionRule`] domain contract: what [`SessionRule::new`] and
//! [`SessionRule::validate`] accept and reject.
//!
//! The SSM domain is `open_ssm in 0..86_400`, `close_ssm in 0..=86_400` (the
//! `86_400` sentinel closes a same-day session exactly at next local
//! midnight — the shape the 24×7 profile uses), and at least one enabled
//! weekday. Equal endpoints encode one complete local-day span. Struct-literal
//! and serde construction bypass the checks by design; `validate` is the
//! recheck for those paths.

use std::borrow::Cow;

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use chrono_tz::{America, Pacific};
use exchange_hours::{
    CalendarResolution, Exchange, MarketHours, SessionKind, SessionRule, SessionRuleError,
    calendar_for_exchange, candle_end, candle_start, hours_for_exchange, next_session_after,
    session_bounds,
};

const MON_FRI: [bool; 7] = [true, true, true, true, true, false, false];
const NO_DAYS: [bool; 7] = [false; 7];

#[test]
fn new_accepts_a_same_day_rule() {
    let rule = SessionRule::new(MON_FRI, 9 * 3600 + 30 * 60, 16 * 3600).expect("valid rule");
    assert_eq!(rule.open_ssm, 9 * 3600 + 30 * 60);
    assert_eq!(rule.close_ssm, 16 * 3600);
    assert_eq!(rule.days, MON_FRI);
    assert!(!rule.wraps_to_next_day());
}

#[test]
fn new_accepts_a_wrap_rule() {
    // 17:00 -> 08:30 next day (Globex shape).
    let rule =
        SessionRule::new(MON_FRI, 17 * 3600, 8 * 3600 + 30 * 60).expect("wrap rules are valid");
    assert!(rule.wraps_to_next_day());
}

#[test]
fn new_accepts_the_full_day_sentinel_close() {
    // 0 -> 86_400 is the always-open shape: closes exactly at next midnight.
    SessionRule::new([true; 7], 0, 86_400).expect("the 86_400 close sentinel is valid");
}

#[test]
fn new_rejects_an_out_of_range_open() {
    assert_eq!(
        SessionRule::new(MON_FRI, 86_400, 3600),
        Err(SessionRuleError::OpenOutOfRange { open_ssm: 86_400 })
    );
}

#[test]
fn new_rejects_an_out_of_range_close() {
    assert_eq!(
        SessionRule::new(MON_FRI, 3600, 86_401),
        Err(SessionRuleError::CloseOutOfRange { close_ssm: 86_401 })
    );
}

#[test]
fn new_accepts_equal_endpoints_as_a_complete_local_day() {
    let rule = SessionRule::new(MON_FRI, 3600, 3600).expect("complete local-day rule is valid");
    assert_eq!(rule.open_ssm, 3600);
    assert_eq!(rule.close_ssm, 3600);
    assert_eq!(rule.validate(), Ok(()));
    assert!(rule.wraps_to_next_day());
}

#[test]
fn new_rejects_an_all_disabled_mask() {
    assert_eq!(
        SessionRule::new(NO_DAYS, 3600, 7200),
        Err(SessionRuleError::NoEnabledDays)
    );
}

#[test]
fn validate_rechecks_a_literal_or_deserialized_rule() {
    // `SessionRule` is `#[non_exhaustive]`, so an out-of-crate struct literal no
    // longer compiles and `new` cannot be bypassed that way. Deserialization is
    // the remaining route that skips `new`, so that is what `validate` must
    // still recheck - exercised below.

    let deserialized: SessionRule = serde_json::from_str(
        r#"{"days":[true,true,true,true,true,false,false],"open_ssm":34200,"close_ssm":57600}"#,
    )
    .expect("valid JSON shape");
    assert_eq!(deserialized.validate(), Ok(()));

    // Serde does not enforce the domain; `validate` is the recheck that must
    // reject an out-of-domain payload.
    let out_of_domain_wire: SessionRule = serde_json::from_str(
        r#"{"days":[true,true,true,true,true,false,false],"open_ssm":86400,"close_ssm":3600}"#,
    )
    .expect("serde accepts the shape regardless of domain");
    assert_eq!(
        out_of_domain_wire.validate(),
        Err(SessionRuleError::OpenOutOfRange { open_ssm: 86_400 })
    );
}

#[test]
fn error_display_names_the_violated_bound() {
    let err = SessionRule::new(MON_FRI, 90_000, 3600).expect_err("out of range");
    let msg = err.to_string();
    assert!(
        msg.contains("90000"),
        "message should carry the value: {msg}"
    );
    assert!(
        msg.contains("86400"),
        "message should carry the bound: {msg}"
    );
}

#[test]
fn equal_endpoints_span_one_local_day_across_dst_transitions() {
    let sunday = [false, false, false, false, false, false, true];
    let rule = SessionRule::new(sunday, 0, 0).expect("complete local-day rule");
    let hours = MarketHours::new(
        Exchange::Unknown,
        America::New_York,
        Cow::Owned(vec![rule]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let et = |date: (i32, u32, u32), time: (u32, u32, u32)| {
        America::New_York
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("valid New York fixture")
            .with_timezone(&Utc)
    };

    for (day, next_day, elapsed_hours) in [
        ((2026, 3, 8), (2026, 3, 9), 23),
        ((2026, 11, 1), (2026, 11, 2), 25),
    ] {
        let open = et(day, (0, 0, 0));
        let close = et(next_day, (0, 0, 0));
        assert_eq!(session_bounds(&hours, open), Some((open, close)));
        assert_eq!(close - open, Duration::hours(elapsed_hours));
        assert!(hours.is_open(close - Duration::nanoseconds(1)));
        assert!(
            !hours.is_open(close),
            "the local-day close is end-exclusive"
        );
    }
}

#[test]
fn a_rule_collapsed_by_a_dst_gap_is_not_a_session_or_candle() {
    let sunday = [false, false, false, false, false, false, true];
    let rule = SessionRule::new(sunday, 2 * 3600 + 15 * 60, 2 * 3600 + 45 * 60)
        .expect("valid Sunday rule");
    let hours = MarketHours::new(
        Exchange::Unknown,
        America::New_York,
        Cow::Owned(vec![rule]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let et = |date: (i32, u32, u32), time: (u32, u32, u32)| {
        America::New_York
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("valid New York fixture")
            .with_timezone(&Utc)
    };

    let before_gap = et((2025, 3, 9), (1, 0, 0));
    let next_open = et((2025, 3, 16), (2, 15, 0));
    let next_close = et((2025, 3, 16), (2, 45, 0));
    let expected = Some((next_open, next_close));

    assert_eq!(next_session_after(&hours, before_gap), expected);
    assert_eq!(session_bounds(&hours, before_gap), expected);
    assert_eq!(
        candle_end(&hours, before_gap, CalendarResolution::Daily),
        Some(next_close)
    );
    assert_eq!(
        candle_start(&hours, before_gap, CalendarResolution::Daily),
        Some(next_open)
    );
    assert!(next_open < next_close);
    assert!(hours.is_closed_all_day_on(
        NaiveDate::from_ymd_opt(2025, 3, 9).expect("valid date"),
        SessionKind::Both,
    ));
}

#[test]
fn a_dst_gap_cannot_invert_a_session_or_calendar_candle() {
    let sunday = [false, false, false, false, false, false, true];
    let rule = SessionRule::new(sunday, 2 * 3600 + 45 * 60 + 45, 2 * 3600 + 50 * 60 + 15)
        .expect("valid Sunday rule");
    let hours = MarketHours::new(
        Exchange::Unknown,
        America::New_York,
        Cow::Owned(vec![rule]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let et = |date: (i32, u32, u32), time: (u32, u32, u32)| {
        America::New_York
            .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("valid New York fixture")
            .with_timezone(&Utc)
    };

    let before_gap = et((2025, 3, 9), (1, 0, 0));
    let next_open = et((2025, 3, 16), (2, 45, 45));
    let next_close = et((2025, 3, 16), (2, 50, 15));
    let expected = Some((next_open, next_close));

    assert_eq!(next_session_after(&hours, before_gap), expected);
    assert_eq!(session_bounds(&hours, before_gap), expected);
    assert_eq!(
        candle_start(&hours, before_gap, CalendarResolution::Daily),
        Some(next_open)
    );
    assert_eq!(
        candle_end(&hours, before_gap, CalendarResolution::Daily),
        Some(next_close)
    );
    assert!(next_open < next_close);
}

#[test]
fn a_partially_skipped_rule_opens_at_the_first_real_second() {
    let sunday = [false, false, false, false, false, false, true];
    let rule = SessionRule::new(sunday, 2 * 3600 + 15 * 60 + 15, 3 * 3600 + 15 * 60)
        .expect("valid second-granularity rule");
    let hours = MarketHours::new(
        Exchange::Unknown,
        America::New_York,
        Cow::Owned(vec![rule]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let et = |time: (u32, u32, u32)| {
        America::New_York
            .with_ymd_and_hms(2025, 3, 9, time.0, time.1, time.2)
            .single()
            .expect("valid New York fixture")
            .with_timezone(&Utc)
    };

    let first_real_second = et((3, 0, 0));
    let close = et((3, 15, 0));
    assert!(hours.is_open_regular(first_real_second));
    assert_eq!(
        session_bounds(&hours, first_real_second),
        Some((first_real_second, close))
    );
    assert!(!hours.is_open_regular(first_real_second - Duration::nanoseconds(1)));
}

#[test]
fn a_partially_skipped_close_lands_on_the_first_real_second() {
    let sunday = [false, false, false, false, false, false, true];
    let rule = SessionRule::new(sunday, 3600 + 30 * 60, 2 * 3600 + 45 * 60 + 45)
        .expect("valid second-granularity rule");
    let hours = MarketHours::new(
        Exchange::Unknown,
        America::New_York,
        Cow::Owned(vec![rule]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let et = |time: (u32, u32, u32)| {
        America::New_York
            .with_ymd_and_hms(2025, 3, 9, time.0, time.1, time.2)
            .single()
            .expect("valid New York fixture")
            .with_timezone(&Utc)
    };

    let open = et((1, 30, 0));
    let first_real_second = et((3, 0, 0));
    assert_eq!(
        session_bounds(&hours, open),
        Some((open, first_real_second))
    );
    assert!(hours.is_open_regular(first_real_second - Duration::nanoseconds(1)));
    assert!(!hours.is_open_regular(first_real_second));
}

#[test]
fn a_wholly_skipped_civil_date_has_an_empty_closed_window() {
    let always_open = hours_for_exchange(Exchange::Unknown);
    let calendar = calendar_for_exchange(Exchange::Unknown);
    let day = |day| NaiveDate::from_ymd_opt(2011, 12, day).expect("valid Apia fixture date");

    for existing_day in [day(29), day(31)] {
        assert!(!always_open.is_closed_all_day_in_calendar(
            existing_day,
            Pacific::Apia,
            SessionKind::Both,
        ));
        assert!(!calendar.is_closed_all_day_in_calendar(
            existing_day,
            Pacific::Apia,
            SessionKind::Both,
        ));
    }
    assert!(always_open.is_closed_all_day_in_calendar(day(30), Pacific::Apia, SessionKind::Both,));
    assert!(calendar.is_closed_all_day_in_calendar(day(30), Pacific::Apia, SessionKind::Both,));
}

#[test]
fn a_fall_back_session_spans_both_copies_of_the_repeated_hour() {
    let sunday = [false, false, false, false, false, false, true];
    let rule =
        SessionRule::new(sunday, 3600 + 30 * 60, 3600 + 45 * 60).expect("valid Sunday fold rule");
    let hours = MarketHours::new(
        Exchange::Unknown,
        America::New_York,
        Cow::Owned(vec![rule]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let utc = |time: (u32, u32, u32)| {
        Utc.with_ymd_and_hms(2025, 11, 2, time.0, time.1, time.2)
            .single()
            .expect("valid UTC fixture")
    };
    let expected = Some((utc((5, 30, 0)), utc((6, 45, 0))));
    assert_eq!(
        America::New_York
            .with_ymd_and_hms(2025, 11, 2, 1, 30, 0)
            .earliest()
            .expect("ambiguous open has an earliest mapping")
            .with_timezone(&Utc),
        utc((5, 30, 0))
    );
    assert_eq!(
        America::New_York
            .with_ymd_and_hms(2025, 11, 2, 1, 45, 0)
            .latest()
            .expect("ambiguous close has a latest mapping")
            .with_timezone(&Utc),
        utc((6, 45, 0))
    );

    for repeated_hour_instant in [utc((5, 50, 0)), utc((6, 20, 0))] {
        assert_eq!(session_bounds(&hours, repeated_hour_instant), expected);
        assert!(hours.is_open_regular(repeated_hour_instant));
    }
    assert!(!hours.is_open_regular(utc((5, 29, 59))));
    assert!(!hours.is_open_regular(utc((6, 45, 0))));
}

#[test]
fn a_wrapping_session_uses_the_latest_fall_back_close() {
    let saturday = [false, false, false, false, false, true, false];
    let rule =
        SessionRule::new(saturday, 23 * 3600, 3600 + 30 * 60).expect("valid Saturday wrap rule");
    let hours = MarketHours::new(
        Exchange::Unknown,
        America::New_York,
        Cow::Owned(vec![rule]),
        Cow::Borrowed(&[]),
        true,
        true,
    );
    let utc = |date: (i32, u32, u32), time: (u32, u32, u32)| {
        Utc.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
            .single()
            .expect("valid UTC fixture")
    };
    let open = utc((2025, 11, 2), (3, 0, 0));
    let close = utc((2025, 11, 2), (6, 30, 0));
    let first_copy_after_nominal_close = utc((2025, 11, 2), (5, 45, 0));

    assert_eq!(
        session_bounds(&hours, first_copy_after_nominal_close),
        Some((open, close))
    );
    assert!(hours.is_open_regular(first_copy_after_nominal_close));
    assert!(!hours.is_open_regular(close));
}
