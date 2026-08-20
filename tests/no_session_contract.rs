// SPDX-License-Identifier: MIT-0

//! The no-session contract: what every query returns for a profile with no
//! rules at all.
//!
//! A no-session profile is reachable through the public API —
//! [`hours_for_exchange_as_of`] returns one for a venue queried before its
//! go-live date (Blue Ocean ATS before 2021-10-05, NYSE Texas before
//! 2025-03-31). The contract these tests pin: every query stays total and
//! terminates, boundary queries return **`None`** — absence of a session is a
//! fact, never a fabricated pair — and no query invents state: a venue with
//! no sessions is never "open", never "in maintenance", and closed all day on
//! every day.
//!
//! Before this contract existed, `candle_end(…, Weekly)` looped effectively
//! forever (advancing 1 ns per iteration), `candle_start(…, Daily)` panicked
//! ("calendar period has no preceding daily close"), and `is_maintenance`
//! reported `true` forever because the then-degenerate `(t, t)` next-session
//! pair sits zero minutes away — the silent-default trap that motivated the
//! `Option` surface.

#![expect(
    clippy::expect_used,
    reason = "fixture constructors assert their own literals; a bad literal must fail the test"
)]

use chrono::{TimeZone, Utc};
use exchange_hours::{
    CalendarResolution, Exchange, SessionKind, candle_end, candle_start, hours_for_exchange_as_of,
    next_session_after, session_bounds,
};

/// A pre-go-live instant for Blue Ocean ATS (production launch 2021-10-05):
/// the returned profile has no regular and no extended rules.
fn no_session_hours() -> exchange_hours::MarketHours {
    let pre_launch = Utc
        .with_ymd_and_hms(2021, 3, 1, 12, 0, 0)
        .single()
        .expect("valid UTC instant");
    let hours = hours_for_exchange_as_of(Exchange::BlueOceanAts, pre_launch);
    assert!(
        hours.regular.is_empty() && hours.extended.is_empty(),
        "fixture must be a no-session profile"
    );
    hours
}

fn probe() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2021, 3, 1, 12, 0, 0)
        .single()
        .expect("valid UTC instant")
}

#[test]
fn no_session_profile_is_closed_and_never_in_maintenance() {
    let hours = no_session_hours();
    let t = probe();
    assert!(!hours.is_open(t), "no-session profile must be closed");
    assert!(
        !hours.is_maintenance(t),
        "a venue with no sessions is not 'about to reopen'; maintenance must be false"
    );
    assert!(
        hours.is_closed_all_day_on(t.with_timezone(&hours.tz).date_naive(), SessionKind::Both),
        "every day is fully closed for a no-session profile"
    );
}

#[test]
fn no_session_bounds_and_next_session_are_none() {
    let hours = no_session_hours();
    let t = probe();
    assert_eq!(
        session_bounds(&hours, t),
        None,
        "a profile with no rules has no session bounds"
    );
    assert_eq!(
        next_session_after(&hours, t),
        None,
        "a profile with no rules has no next session"
    );
}

#[test]
fn no_session_daily_candle_end_is_none() {
    let hours = no_session_hours();
    let t = probe();
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Daily),
        None,
        "no daily close exists in the horizon, so there is no bar"
    );
}

#[test]
fn no_session_weekly_and_monthly_candle_end_terminate_with_none() {
    let hours = no_session_hours();
    let t = probe();
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Weekly),
        None,
        "Weekly candle end must be None, not loop"
    );
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Monthly),
        None,
        "Monthly candle end must be None, not loop"
    );
}

#[test]
fn no_session_candle_start_is_none_not_a_panic() {
    let hours = no_session_hours();
    let t = probe();
    for res in [
        CalendarResolution::Daily,
        CalendarResolution::Weekly,
        CalendarResolution::Monthly,
    ] {
        assert_eq!(
            candle_start(&hours, t, res),
            None,
            "{res:?} candle start must be None for a no-session profile"
        );
    }
}

#[test]
fn no_session_intraday_candles_are_none_but_seconds_stays_an_offset() {
    let hours = no_session_hours();
    let t = probe();
    // Fixed-grid bars need an enclosing session; none exists. Seconds stays a
    // pure offset by contract — it never consults the calendar.
    assert_eq!(candle_end(&hours, t, CalendarResolution::Minutes(5)), None);
    assert_eq!(candle_end(&hours, t, CalendarResolution::Hours(1)), None);
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Seconds(30)),
        Some(t + chrono::Duration::seconds(30))
    );
    // The paired starts agree on bar existence.
    assert_eq!(
        candle_start(&hours, t, CalendarResolution::Minutes(5)),
        None
    );
    assert_eq!(candle_start(&hours, t, CalendarResolution::Hours(1)), None);
    assert_eq!(
        candle_start(&hours, t, CalendarResolution::Seconds(30)),
        Some(t)
    );
}

#[test]
fn zero_intervals_have_no_bar_on_any_profile() {
    // A zero interval could never advance, so both ends of the bar are None —
    // on a no-session profile and on a normal venue alike.
    use exchange_hours::{candle_end_with, candle_start_with, hours_for_exchange};
    let cme = hours_for_exchange(Exchange::Cme);
    let empty = no_session_hours();
    let t = probe();
    for res in [
        CalendarResolution::Seconds(0),
        CalendarResolution::Minutes(0),
        CalendarResolution::Hours(0),
    ] {
        for hours in [&cme, &empty] {
            assert_eq!(candle_end_with(hours, t, res, SessionKind::Both), None);
            assert_eq!(candle_start_with(hours, t, res, SessionKind::Both), None);
        }
    }
}
