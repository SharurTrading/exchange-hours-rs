// SPDX-License-Identifier: MIT-0

//! Fence for the fixed-snapshot `Result` -> `Option` collapse (LAW-INVARIANT
//! audit #210; the structural follow-up is #245).
//!
//! The fixed-snapshot adapters (`MarketHours`' own predicates and the free
//! `session_bounds` / `candle_end` family) collapse the shared engine's
//! `Result<Option<T>, CalendarQueryError>` into an answer with no error
//! channel, on the invariant that a detached snapshot can raise no coverage
//! error: every `CalendarQueryError` the engine produces is gated on the
//! identity's coverage metadata, which `QueryContext::fixed` never carries.
//! The invariant and its mechanism live in one place,
//! `query::schedule::fixed_answer`; this suite pins the observable consequence
//! over **every shipped identity**:
//!
//! 1. at an instant below the support floor the date-aware calendar refuses
//!    with [`CalendarQueryError::BeforeSupportFloor`] — that gate is exactly
//!    what the fixed path deliberately lacks; and
//! 2. the fixed snapshot selected at that pre-floor instant answers its own
//!    normal-week grid identically at the pre-floor samples and at twin
//!    samples one whole ISO-week multiple later, once the twin's venue-local
//!    day has crossed the floor. Were a live engine error ever to reach the
//!    collapse on shipped data, it would surface as an absence-shaped answer
//!    (`false`, `Closed`, `None`) on one side of that comparison and fail
//!    here, rather than silently.
//!
//! The wall-clock correspondence the parity needs is asserted per sample from
//! the snapshot's own public `tz` field, so a future identity whose zone
//! shifted its offset between the two eras fails loudly instead of passing a
//! vacuous parity.
//!
//! What this fence can and cannot catch: an error source that fires
//! **asymmetrically** — on pre-floor days but not at the twin, which is the
//! shape a coverage-style gate would have — shows up as an absence-shaped
//! answer on one side and fails here (mutation-verified: making
//! `require_floor_at` fire without coverage metadata fails
//! `every_venue_refuses_pre_floor_while_its_fixed_snapshot_answers_parity`
//! with `globex_equity_index: is_open disagrees`). A hypothetical error that
//! fired **symmetrically** at every instant would keep this parity and is
//! caught instead by the cross-surface contracts
//! (`seasonal_calendars::every_fixed_venue_calendar_matches_the_current_market_hours_surface`,
//! `surface_agreement`) — mutation-verified as well, with the same two
//! mutations. The two layers together cover the collapse.

use chrono::{DateTime, Datelike, Duration, Utc};
use chrono_tz::Tz;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, Exchange, MarketHours, MarketHoursKey, SUPPORT_FLOOR,
    SessionKind, calendar_for_exchange, calendar_for_market_hours_key, candle_end,
    hours_for_exchange, hours_for_market_hours_key, session_bounds,
};

/// The pre-floor anchor: a Monday UTC, at least a decade below the support
/// floor in every IANA zone, so every identity's venue-local day is below the
/// floor on both the anchor and the whole sample sweep. 2000-01-03 12:00 UTC.
const PRE_ANCHOR: DateTime<Utc> = match DateTime::<Utc>::from_timestamp(946_900_800, 0) {
    Some(instant) => instant,
    None => panic!("the pre-floor anchor must be representable"),
};

/// A whole ISO-week multiple keeps the venue-local wall clock and weekday: 364
/// days is 52 weeks, so each `k` step shifts the anchor without ever moving it
/// off its weekday.
const WEEK_STEP_DAYS: i64 = 364;

/// The smallest whole-ISO-week multiple that lands the anchor's twin at or
/// after the support floor in `tz`.
fn twin_weeks(tz: Tz) -> i64 {
    let mut weeks = 1_i64;
    loop {
        let candidate = PRE_ANCHOR + Duration::days(WEEK_STEP_DAYS * weeks);
        if candidate.with_timezone(&tz).date_naive() >= SUPPORT_FLOOR {
            return weeks;
        }
        weeks += 1;
    }
}

/// Samples a full UTC day at half-hour steps starting at `anchor`.
fn sample_instants(anchor: DateTime<Utc>) -> impl Iterator<Item = DateTime<Utc>> {
    (0..48u32).map(move |step| anchor + Duration::minutes(i64::from(step) * 30))
}

/// Asserts the fixed snapshot answers its own grid identically below the floor
/// and at the post-floor twin, for every predicate the fixed surface exposes.
fn assert_collapse_parity(hours: &MarketHours, identity: &str) {
    let tz = hours.tz;
    let weeks = twin_weeks(tz);
    let twin_anchor = PRE_ANCHOR + Duration::days(WEEK_STEP_DAYS * weeks);
    let shift = twin_anchor - PRE_ANCHOR;

    for sample in sample_instants(PRE_ANCHOR) {
        let twin = sample + shift;
        let sample_local = sample.with_timezone(&tz);
        let twin_local = twin.with_timezone(&tz);
        assert_eq!(
            sample_local.time(),
            twin_local.time(),
            "{identity}: the twin must carry the same venue-local wall clock"
        );
        assert_eq!(
            sample_local.date_naive().weekday(),
            twin_local.date_naive().weekday(),
            "{identity}: the twin must carry the same venue-local weekday"
        );

        assert_eq!(
            hours.is_open(sample),
            hours.is_open(twin),
            "{identity}: is_open disagrees below the floor and at the twin"
        );
        assert_eq!(
            hours.is_open_regular(sample),
            hours.is_open_regular(twin),
            "{identity}: is_open_regular disagrees below the floor and at the twin"
        );
        assert_eq!(
            hours.is_open_extended(sample),
            hours.is_open_extended(twin),
            "{identity}: is_open_extended disagrees below the floor and at the twin"
        );
        assert_eq!(
            hours.is_accepting_orders(sample),
            hours.is_accepting_orders(twin),
            "{identity}: is_accepting_orders disagrees below the floor and at the twin"
        );
        assert_eq!(
            hours.is_order_entry_only(sample),
            hours.is_order_entry_only(twin),
            "{identity}: is_order_entry_only disagrees below the floor and at the twin"
        );
        assert_eq!(
            hours.is_maintenance(sample),
            hours.is_maintenance(twin),
            "{identity}: is_maintenance disagrees below the floor and at the twin"
        );
        assert_eq!(
            hours.session_state(sample),
            hours.session_state(twin),
            "{identity}: session_state disagrees below the floor and at the twin"
        );

        let pre_bounds = session_bounds(hours, sample);
        let twin_bounds = session_bounds(hours, twin);
        let shifted = pre_bounds.map(|(open, close)| (open + shift, close + shift));
        assert_eq!(
            shifted, twin_bounds,
            "{identity}: session bounds disagree below the floor and at the twin"
        );

        for resolution in [CalendarResolution::Daily, CalendarResolution::Weekly] {
            let pre_close = candle_end(hours, sample, resolution);
            let twin_close = candle_end(hours, twin, resolution);
            let shifted_close = pre_close.map(|close| close + shift);
            assert_eq!(
                shifted_close, twin_close,
                "{identity:?}: {resolution:?} close disagrees below the floor and at the twin"
            );
        }

        let pre_day = sample_local.date_naive();
        let twin_day = twin_local.date_naive();
        assert_eq!(
            hours.is_closed_all_day_on(pre_day, SessionKind::Both),
            hours.is_closed_all_day_on(twin_day, SessionKind::Both),
            "{identity}: is_closed_all_day_on disagrees below the floor and at the twin"
        );
    }
}

/// Asserts the date-aware calendar refuses the pre-floor sweep with the floor
/// error, which is exactly the gate the fixed collapse deliberately lacks.
fn assert_date_aware_refuses(exchange: Exchange, identity: &str) {
    let calendar = calendar_for_exchange(exchange);
    for sample in sample_instants(PRE_ANCHOR) {
        assert!(
            matches!(
                calendar.is_open(sample),
                Err(CalendarQueryError::BeforeSupportFloor { .. })
            ),
            "{identity}: the date-aware calendar must refuse a pre-floor instant"
        );
    }
    let first = PRE_ANCHOR;
    assert!(
        calendar.session_bounds(first).is_err()
            && calendar.session_state(first).is_err()
            && calendar.trade_date(first).is_err()
            && calendar
                .candle_end(first, CalendarResolution::Daily)
                .is_err(),
        "{identity}: every date-aware query family must refuse a pre-floor instant"
    );
}

#[test]
fn every_venue_refuses_pre_floor_while_its_fixed_snapshot_answers_parity() {
    for exchange in Exchange::ALL {
        let identity = exchange.as_str();
        let hours = hours_for_exchange(*exchange, PRE_ANCHOR);
        assert_collapse_parity(&hours, identity);
        assert_date_aware_refuses(*exchange, identity);
    }
}

#[test]
fn every_market_hours_key_refuses_pre_floor_while_its_fixed_snapshot_answers_parity() {
    for key in MarketHoursKey::ALL {
        let identity = key.as_str();
        let hours = hours_for_market_hours_key(*key, PRE_ANCHOR);
        assert_collapse_parity(&hours, identity);

        let calendar = calendar_for_market_hours_key(*key);
        for sample in sample_instants(PRE_ANCHOR) {
            assert!(
                matches!(
                    calendar.is_open(sample),
                    Err(CalendarQueryError::BeforeSupportFloor { .. })
                ),
                "{identity}: the key-backed calendar must refuse a pre-floor instant"
            );
        }
    }
}

#[test]
fn the_collapse_covers_the_whole_fixed_predicate_surface() {
    // The fence above pins the collapse's observable contract only if it runs
    // against a non-empty identity set and a non-empty sample sweep; pin the
    // fence itself so a future refactor that empties either fails here.
    assert!(
        Exchange::ALL.len() + MarketHoursKey::ALL.len() > 64,
        "the identity sets the fence walks must not be emptied"
    );
    assert_eq!(
        sample_instants(PRE_ANCHOR).count(),
        48,
        "the fence's sample sweep must stay a full day at half-hour steps"
    );
}
