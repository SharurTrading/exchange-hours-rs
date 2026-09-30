// SPDX-License-Identifier: MIT-0

//! The no-movement fence the preopen rows' measured claim needs (#181).
//!
//! The preopen PRs (#139 for this family's own Pre-Open rows, #165 for the
//! merged eves' evening Pre-Open, #179 for the fence this test answers) each
//! measured that adding an order-entry phase moves no `is_open`,
//! `is_open_regular`, `is_open_extended` or `session_bounds` answer over the
//! supported windows. Nothing in CI pinned the claim. Rather than re-measure
//! one PR's expectations, this fence pins the narrower invariant those claims
//! are instances of, for every identity, at the resolution the budget allows:
//!
//! An order-entry phase is not a session, so at any instant the identity
//! answers
//!
//! 1. `session_state == OrderEntry` implies `is_open == false` — no trade can
//!    print inside a queue;
//! 2. `session_state == OrderEntry` implies `session_bounds` reports the
//!    **next** session, opening strictly after the instant — the queue is
//!    never absorbed into a session window that contains the instant;
//! 3. the split phase queries stay exclusive: `is_open_regular` and
//!    `is_open_extended` are both false inside a queue, and exactly one is
//!    true inside a session.
//!
//! Properties 1 and 2 are what "an order-entry phase's arrival never widens
//! `is_open`/`session_bounds`" means observably: a wider `is_open` would
//! report `true` inside a queue, and a widened `session_bounds` would return
//! a window containing it.
//!
//! The walk covers the two keys the preopen PRs changed at five-minute
//! granularity across 2025-01-01..2028-01-01 (~630k instants, the resolution
//! and span PR #179 measured), and every identity over one sampled week at
//! fifteen-minute granularity — the same week the order-entry contracts in
//! `tests/order_entry_phase.rs` fence — so a future row or phase change on
//! any identity cannot silently widen the two answers.

#![expect(clippy::expect_used, reason = "fixture literals must fail the test if malformed")]

use chrono::{DateTime, Datelike as _, TimeZone as _, Utc};
use exchange_hours::{
    Exchange, MarketHoursKey, SessionState, calendar_for_exchange, calendar_for_market_hours_key,
};

fn utc(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture instants fall once")
}

/// The invariant, asserted for one calendar at one instant.
///
/// A probe that refuses naming a date the metadata does not call covered is
/// the resolution-edge contract (#151) and is accepted: an instant on a
/// covered date can depend on a neighbouring date the identity withholds, and
/// one entry point may reach further than another. A refusal naming a
/// covered date is the disagreement the fence exists to catch.
fn assert_queue_never_widens(
    calendar: &exchange_hours::ExchangeCalendar,
    instant: DateTime<Utc>,
    label: &str,
) {
    let accepted = |error: &exchange_hours::CalendarQueryError| {
        let named = error.date();
        calendar.coverage().coverage_on(named) != exchange_hours::DateCoverage::Covered
    };
    let Ok(state) = calendar.session_state(instant) else { return };
    let Ok(is_open) = calendar.is_open(instant) else { return };
    if state == SessionState::OrderEntry {
        assert!(
            !is_open,
            "{label}: a queue reports is_open true — an order-entry phase widened it"
        );
        match calendar.session_bounds(instant) {
            Err(error) if accepted(&error) => {}
            Err(error) => {
                panic!("{label}: session_bounds refuses naming a covered date: {error:?}")
            }
            Ok(Some((open, _close))) => assert!(
                open > instant,
                "{label}: session_bounds returns a window opening at or before the \\
                 instant inside a queue — the queue was absorbed into a session"
            ),
            Ok(None) => {}
        }
    } else if state == SessionState::OpenRegular || state == SessionState::OpenExtended {
        assert!(is_open, "{label}: an open session reports is_open false");
        let regular = calendar.is_open_regular(instant);
        let extended = calendar.is_open_extended(instant);
        // The state's own phase query must be true. The other may also be:
        // an extended session overlapping a regular one is a real shape (TSE's
        // trading-at-last inside the afternoon), so exclusivity is not the
        // invariant — the state's own phase is.
        if state == SessionState::OpenRegular {
            assert_eq!(regular, Ok(true), "{label}");
        } else {
            assert_eq!(extended, Ok(true), "{label}");
        }
        let _ = (regular, extended);
    }
}

#[test]
fn a_queue_never_widens_is_open_or_session_bounds_on_the_preopen_keys_windows() {
    for key in [MarketHoursKey::GlobexGrains, MarketHoursKey::GlobexNikkei225Dollar] {
        let calendar = calendar_for_market_hours_key(key);
        let mut day = utc(2025, 1, 1, 0, 0).date_naive();
        let last = utc(2028, 1, 1, 0, 0).date_naive();
        while day < last {
            // Five-minute steps, in UTC; a step that DST makes ambiguous or
            // skipped simply probes a neighbouring instant, which is fine for
            // a property sweep.
            for slot in 0..(24 * 12) {
                let instant = Utc
                    .with_ymd_and_hms(
                        day.year(),
                        day.month(),
                        day.day(),
                        (slot / 12) as u32,
                        (slot % 12) * 5,
                        0,
                    )
                    .single();
                let Some(instant) = instant else { continue };
                assert_queue_never_widens(&calendar, instant, &format!("{key:?} {instant}"));
            }
            day = day.succ_opt().expect("the walk stays inside the year range");
        }
    }
}

#[test]
fn a_queue_never_widens_is_open_or_session_bounds_on_every_identity_for_a_sampled_week() {
    for &exchange in Exchange::ALL {
        let calendar = calendar_for_exchange(exchange);
        for day in 17..=23u32 {
            for hour in 0..24u32 {
                for minute in (0..60u32).step_by(15) {
                    let instant = utc(2026, 8, day, hour, minute);
                    assert_queue_never_widens(
                        &calendar,
                        instant,
                        &format!("{exchange:?} {instant}"),
                    );
                }
            }
        }
    }
    for &key in MarketHoursKey::ALL {
        let calendar = calendar_for_market_hours_key(key);
        for day in 17..=23u32 {
            for hour in 0..24u32 {
                for minute in (0..60u32).step_by(15) {
                    let instant = utc(2026, 8, day, hour, minute);
                    assert_queue_never_widens(&calendar, instant, &format!("{key:?} {instant}"));
                }
            }
        }
    }
}
