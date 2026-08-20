// Copyright (C) 2026 Kevin Monaghan. All rights reserved.
//
// This file is proprietary and confidential.
// Unauthorized copying, use, modification, distribution, or disclosure of this file,
// via any medium, is strictly prohibited except under a written agreement with the
// copyright owner.

//! The no-session contract: what every query returns for a profile with no
//! rules at all.
//!
//! A no-session profile is reachable through the public API —
//! [`hours_for_exchange_as_of`] returns one for a venue queried before its
//! go-live date (Blue Ocean ATS before 2021-10-05, NYSE Texas before
//! 2025-03-31). The contract these tests pin: every query stays total and
//! terminates, boundary queries return their **degenerate** value (the query
//! instant itself, i.e. `open == close` / `end == t`), and no query invents
//! state — a venue with no sessions is never "open", never "in maintenance",
//! and closed all day on every day.
//!
//! Before this contract existed, `candle_end(…, Weekly)` looped effectively
//! forever (advancing 1 ns per iteration), `candle_start(…, Daily)` panicked
//! ("calendar period has no preceding daily close"), and `is_maintenance`
//! reported `true` forever because the degenerate next-session pair sits zero
//! minutes away.

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
fn no_session_bounds_and_next_session_are_degenerate() {
    let hours = no_session_hours();
    let t = probe();
    assert_eq!(
        session_bounds(&hours, t),
        (t, t),
        "bounds must be the documented degenerate (t, t) pair"
    );
    assert_eq!(
        next_session_after(&hours, t),
        (t, t),
        "next session must be the documented degenerate (t, t) pair"
    );
}

#[test]
fn no_session_daily_candle_end_returns_its_input() {
    let hours = no_session_hours();
    let t = probe();
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Daily),
        t,
        "Daily candle end must degenerate to t when no close exists in the horizon"
    );
}

#[test]
fn no_session_weekly_and_monthly_candle_end_terminate_and_return_their_input() {
    let hours = no_session_hours();
    let t = probe();
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Weekly),
        t,
        "Weekly candle end must degenerate to t, not loop"
    );
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Monthly),
        t,
        "Monthly candle end must degenerate to t, not loop"
    );
}

#[test]
fn no_session_candle_start_is_degenerate_not_a_panic() {
    let hours = no_session_hours();
    let t = probe();
    for res in [
        CalendarResolution::Daily,
        CalendarResolution::Weekly,
        CalendarResolution::Monthly,
    ] {
        assert_eq!(
            candle_start(&hours, t, res),
            t,
            "{res:?} candle start must degenerate to t for a no-session profile"
        );
    }
}

#[test]
fn no_session_intraday_candles_stay_degenerate() {
    let hours = no_session_hours();
    let t = probe();
    // Fixed-grid bars clamp to the (degenerate) enclosing session, so the end
    // never advances; Seconds stays a pure offset by contract.
    assert_eq!(candle_end(&hours, t, CalendarResolution::Minutes(5)), t);
    assert_eq!(candle_end(&hours, t, CalendarResolution::Hours(1)), t);
    assert_eq!(
        candle_end(&hours, t, CalendarResolution::Seconds(30)),
        t + chrono::Duration::seconds(30)
    );
}
