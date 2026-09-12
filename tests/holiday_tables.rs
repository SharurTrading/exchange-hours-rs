// SPDX-License-Identifier: MIT-0

//! Public contracts for the built-in holiday-table engine (LAW-HOLIDAY-SCOPE).
//!
//! The engine ships in two halves. This wave is the first: the row types, the
//! `holidays!` macro and its constant-evaluation fences, the coverage gate, the
//! layering, the evidence fence and the three public accessors — with **zero
//! rows**. So the fences here are of two kinds.
//!
//! **The crate carries no holiday data.** `holiday_on` and `holiday_coverage`
//! answer `None` for every identity and every date, `without_holidays` is the
//! identity function on every answer, and the golden normal-week grids are
//! untouched. That is the whole observable behaviour change of this wave:
//! none.
//!
//! **The engine behaves as specified**, proved through what a caller can
//! actually attach. The clip path a built-in row will take — an early close
//! stated on a trade date that clips a session which opened the previous
//! evening, a closure that removes that evening leg, and a late open on each
//! of its two branches — is the same code a caller's `DayPolicy` drives, so a
//! synthetic `StaticDayPolicy` exercises it now. The coverage gate is live in
//! this wave for a caller's exception provider, so it is fenced against an
//! ungated reference in both directions.
//!
//! Per-family row fences (§4.1 of the design memo: a closed day, both sides of
//! an early close, both late-open branches, the wrap removal, the trade-date
//! consequence and both sides of the coverage window) arrive with the families
//! whose rows they test.

#![expect(
    clippy::expect_used,
    reason = "fixture literals and validated static records must fail the test if malformed"
)]

use chrono::{DateTime, Days, NaiveDate, TimeDelta, TimeZone, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, CalendarSource, DayOverride, DayPolicy, Exchange, ExchangeCalendar,
    MarketHoursKey, PolicyCalendar, SessionExceptionRecord, SessionKind, StaticDayPolicy,
    StaticSessionExceptions, calendar_for_exchange, calendar_for_market_hours_key,
};

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be a valid CT instant")
        .with_timezone(&Utc)
}

/// Every identity the crate ships, as a date-aware calendar.
fn every_calendar() -> Vec<(String, ExchangeCalendar)> {
    let mut calendars = Vec::new();
    for exchange in Exchange::ALL {
        calendars.push((
            format!("Exchange::{exchange:?}"),
            calendar_for_exchange(*exchange),
        ));
    }
    for key in MarketHoursKey::ALL {
        calendars.push((
            format!("MarketHoursKey::{key:?}"),
            calendar_for_market_hours_key(*key),
        ));
    }
    calendars
}

/// The identities whose trade-date conventions decide a gate window.
///
/// One per class of the design memo's gate-window table: the close-date
/// default, SET Thailand's prior opening date, CBOT Rough Rice's following
/// local date, and the two families that roll to the next open business date.
fn gate_window_classes() -> Vec<(&'static str, ExchangeCalendar)> {
    vec![
        ("nyse", calendar_for_exchange(Exchange::Nyse)),
        ("set_thailand", calendar_for_exchange(Exchange::SetThailand)),
        (
            "globex_equity_index",
            calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex),
        ),
        (
            "globex_rough_rice",
            calendar_for_market_hours_key(MarketHoursKey::GlobexRoughRice),
        ),
        (
            "globex_cryptocurrency",
            calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency),
        ),
        (
            "globex_event_contracts_btc",
            calendar_for_market_hours_key(MarketHoursKey::GlobexEventContractsBtc),
        ),
    ]
}

/// Asserts that an overlay-carrying calendar answers exactly as the bare one
/// over a dense instant grid.
fn assert_agrees(
    label: &str,
    overlaid: PolicyCalendar<'_>,
    bare: ExchangeCalendar,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) {
    let mut instant = from;
    while instant < to {
        assert_eq!(
            overlaid.is_open(instant),
            bare.is_open(instant),
            "{label}: is_open diverged at {instant}"
        );
        assert_eq!(
            overlaid.session_state(instant),
            bare.session_state(instant),
            "{label}: session_state diverged at {instant}"
        );
        assert_eq!(
            overlaid.session_bounds(instant),
            bare.session_bounds(instant),
            "{label}: session_bounds diverged at {instant}"
        );
        assert_eq!(
            overlaid.next_session_after(instant),
            bare.next_session_after(instant),
            "{label}: next_session_after diverged at {instant}"
        );
        assert_eq!(
            overlaid.trade_date(instant),
            bare.trade_date(instant),
            "{label}: trade_date diverged at {instant}"
        );
        assert_eq!(
            overlaid.is_accepting_orders(instant),
            bare.is_accepting_orders(instant),
            "{label}: is_accepting_orders diverged at {instant}"
        );
        instant += TimeDelta::minutes(43);
    }
}

// ---------------------------------------------------------------------------
// The crate carries no holiday data.
// ---------------------------------------------------------------------------

/// LAW-HOLIDAY-SCOPE: no table ships yet, so the caller's own overlay is still
/// the only holiday layer, for every identity without exception.
#[test]
fn no_identity_ships_a_holiday_table() {
    for (label, calendar) in every_calendar() {
        assert_eq!(
            calendar.holiday_coverage(),
            None,
            "{label} claims a holiday coverage window, but no table ships"
        );
    }
}

/// A table with no coverage window can hold no row, at any date in or below
/// the crate's horizon or above its published future.
#[test]
fn no_identity_reports_a_holiday_row() {
    let calendars = every_calendar();
    let mut date = day(2009, 1, 1);
    let end = day(2029, 1, 1);
    while date < end {
        for (label, calendar) in &calendars {
            assert_eq!(
                calendar.holiday_on(date),
                None,
                "{label} reports a holiday row on {date}, but no table ships"
            );
        }
        date = date
            .checked_add_days(Days::new(11))
            .expect("the scan stays inside the representable calendar");
    }
}

/// `without_holidays` is the exact A/B control the design memo's benchmark
/// needs, so while no table ships it has to be the identity function — on the
/// accessors and on every query.
#[test]
fn without_holidays_changes_no_answer_while_no_table_ships() {
    for (label, calendar) in gate_window_classes() {
        let detached = calendar.without_holidays();
        assert_eq!(detached.source(), calendar.source());
        assert_eq!(detached.tz(), calendar.tz());
        assert_eq!(detached.holiday_coverage(), None);

        let mut instant = ct((2025, 12, 19), (0, 0, 0));
        let end = ct((2026, 1, 5), (0, 0, 0));
        while instant < end {
            assert_eq!(
                detached.is_open(instant),
                calendar.is_open(instant),
                "{label}: is_open diverged at {instant}"
            );
            assert_eq!(
                detached.session_state(instant),
                calendar.session_state(instant),
                "{label}: session_state diverged at {instant}"
            );
            assert_eq!(
                detached.session_bounds(instant),
                calendar.session_bounds(instant),
                "{label}: session_bounds diverged at {instant}"
            );
            assert_eq!(
                detached.trade_date(instant),
                calendar.trade_date(instant),
                "{label}: trade_date diverged at {instant}"
            );
            assert_eq!(
                detached.candle_end(instant, CalendarResolution::Daily),
                calendar.candle_end(instant, CalendarResolution::Daily),
                "{label}: daily close diverged at {instant}"
            );
            instant += TimeDelta::minutes(43);
        }
    }
}

/// Detaching twice is detaching once, and a detached calendar keeps its
/// identity — the `Copy` value is still the same schedule, minus one layer.
#[test]
fn without_holidays_is_idempotent_and_keeps_the_identity() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let once = calendar.without_holidays();
    assert_eq!(once.without_holidays(), once);
    assert_eq!(
        once.source(),
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex)
    );
    assert_eq!(
        once.market_hours_key(),
        Some(MarketHoursKey::GlobexEquityIndex)
    );
    assert_eq!(once.exchange(), None);
}

/// The overlay wrapper reports the crate's own table, not the caller's layers,
/// and detaching it keeps both overlays attached.
#[test]
fn policy_calendar_mirrors_the_builtin_accessors() {
    let overrides = [DayOverride::closed(day(2025, 12, 25))];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let records = [SessionExceptionRecord::known_normal(day(2025, 12, 24))];
    let exceptions = StaticSessionExceptions::new(
        CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex),
        day(2025, 12, 1),
        day(2025, 12, 31),
        &records,
    )
    .expect("the fixture table is valid");

    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_day_policy(&policy)
        .with_session_exceptions(&exceptions)
        .expect("the fixture is scoped to this calendar");

    // The caller closed 2025-12-25 and audited 2025-12-24. Neither is a
    // built-in row, and the built-in accessors say so.
    assert_eq!(calendar.holiday_on(day(2025, 12, 25)), None);
    assert_eq!(calendar.holiday_on(day(2025, 12, 24)), None);
    assert_eq!(calendar.holiday_coverage(), None);

    let detached = calendar.without_holidays();
    assert!(detached.has_day_policy());
    assert!(detached.has_session_exceptions());
    assert_eq!(detached.holiday_coverage(), None);
    assert!(detached.is_closed_trade_date(day(2025, 12, 25), SessionKind::Both));
}

// ---------------------------------------------------------------------------
// The coverage gate.
// ---------------------------------------------------------------------------

/// The gate is a *claim* about each identity's trade-date conventions: an
/// occurrence can only be changed by a record for a date its own convention
/// could assign it to. This is what makes the claim true rather than hoped.
///
/// A provider whose window is remote must be gated out, and a provider whose
/// window covers the grid must not be — and both must answer exactly as the
/// bare calendar, because neither holds a record. Divergence in the first case
/// is an unsound gate; divergence in the second is an unsound overlay path.
#[test]
fn the_coverage_gate_is_sound_for_every_trade_date_convention() {
    for (label, calendar) in gate_window_classes() {
        let source = calendar.source();
        let from = ct((2026, 4, 10), (0, 0, 0));
        let to = ct((2026, 4, 24), (0, 0, 0));

        // Remote window: every gate window formed inside the grid misses it,
        // so the gate exits on one binary search.
        let remote = StaticSessionExceptions::new(source, day(2019, 1, 1), day(2019, 12, 31), &[])
            .expect("an empty record slice is valid");
        let gated = calendar
            .with_session_exceptions(&remote)
            .expect("the fixture is scoped to this calendar");
        assert_agrees(label, gated, calendar, from, to);

        // Covering window with no records: the gate opens and the full
        // derivation runs, which must reach the same answer.
        let covering =
            StaticSessionExceptions::new(source, day(2026, 1, 1), day(2026, 12, 31), &[])
                .expect("an empty record slice is valid");
        let ungated = calendar
            .with_session_exceptions(&covering)
            .expect("the fixture is scoped to this calendar");
        assert_agrees(label, ungated, calendar, from, to);
    }
}

/// A window that ends the day before the grid starts still has to be consulted
/// for the first opening day in it: a wrapped session opening on `D` can carry
/// trade date `D + 1`, and SET Thailand's night phase can carry `D - 1`.
///
/// The fence is the answer, not the cost: a gate that clipped its own window by
/// a day would silently drop a real record at the edge of a caller's coverage.
#[test]
fn the_gate_window_reaches_the_neighbouring_trade_dates() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);
    let source = calendar.source();

    // Monday 2026-04-20's trading day opens Sunday 2026-04-19 at 17:00 CT. A
    // record keyed to Monday must reach that Sunday-opening occurrence.
    let records = [SessionExceptionRecord::closed(day(2026, 4, 20))];
    let table = StaticSessionExceptions::new(source, day(2026, 4, 20), day(2026, 4, 20), &records)
        .expect("the fixture table is valid");
    let overlaid = calendar
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");

    assert!(calendar.is_open(ct((2026, 4, 19), (18, 0, 0))));
    assert!(
        !overlaid.is_open(ct((2026, 4, 19), (18, 0, 0))),
        "a one-day coverage window on Monday must still reach Sunday evening"
    );
    assert!(!overlaid.is_open(ct((2026, 4, 20), (10, 0, 0))));
    // The Monday-evening leg belongs to Tuesday and is untouched.
    assert!(overlaid.is_open(ct((2026, 4, 20), (18, 0, 0))));
}

// ---------------------------------------------------------------------------
// The clip path a built-in row will take, driven by a caller's DayPolicy.
// ---------------------------------------------------------------------------

/// An early close is stated on the **trade date**, so it lands on the correct
/// civil day for a session that opened the previous evening, and an occurrence
/// that would begin after the cutoff disappears rather than inverting.
///
/// This is the design memo's §1.3 table, on the family it was written for:
/// `globex_equity_index`'s Sunday-to-Thursday 17:00 CT leg wraps to 08:30 CT,
/// the regular session runs 08:30-15:15 CT, and a 15:15-16:00 CT extended leg
/// closes the trading day.
#[test]
fn an_early_close_clips_a_trading_day_that_opened_the_previous_evening() {
    let overrides = [DayOverride::early_close(
        day(2025, 11, 28),
        12 * 3_600 + 15 * 60,
    )];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    // Before the cutoff, inside the regular session.
    assert!(calendar.is_open(ct((2025, 11, 28), (12, 14, 59))));
    // At the cutoff: closes are end-exclusive.
    assert!(!calendar.is_open(ct((2025, 11, 28), (12, 15, 0))));
    // The remainder of the regular session is gone.
    assert!(!calendar.is_open(ct((2025, 11, 28), (14, 0, 0))));
    // So is the 15:15-16:00 CT leg, which opens after the cutoff: it is
    // dropped rather than inverted.
    assert!(!calendar.is_open(ct((2025, 11, 28), (15, 30, 0))));
    // The trading day now ends at the cutoff.
    assert_eq!(
        calendar.candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2025, 11, 28), (12, 15, 0)))
    );
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), (10, 0, 0))),
        Some(day(2025, 11, 28))
    );
    // The Thursday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert!(calendar.is_open(ct((2025, 11, 27), (18, 0, 0))));
    // The next session is Sunday's reopen, not a same-day remainder.
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 11, 28), (12, 20, 0))),
        Some(ct((2025, 11, 30), (17, 0, 0)))
    );
}

/// A closed trade date removes its complete trading day, including the leg that
/// opened the previous evening — and leaves the next trade date's leg, which
/// opens on the holiday itself, alone.
#[test]
fn a_closed_trade_date_removes_the_previous_evenings_wrap() {
    let overrides = [DayOverride::closed(day(2025, 12, 25))];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(calendar.is_closed_trade_date(day(2025, 12, 25), SessionKind::Both));
    // Wednesday evening fed trade date Thursday; it is gone.
    assert!(!calendar.is_open(ct((2025, 12, 24), (17, 30, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 25), (10, 0, 0))));
    // Thursday evening feeds trade date Friday; it is untouched.
    assert!(calendar.is_open(ct((2025, 12, 25), (18, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), (18, 0, 0))),
        Some(day(2025, 12, 26))
    );
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 12, 24), (16, 30, 0))),
        Some(ct((2025, 12, 25), (17, 0, 0)))
    );
    // The civil day is not wholly closed, because the next trade date's
    // session opens inside it. `is_closed_trade_date` is the holiday question.
    assert!(!calendar.is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both));
}

/// A late open's wall clock is disambiguated against the trading day's own
/// first open, and both branches must be exercised: the branch choice is
/// data-dependent and a flip is a silent 24-hour error.
#[test]
fn a_late_open_resolves_on_both_of_its_branches() {
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    // Branch one: 19:00 CT is at or after the trading day's normal 17:00 CT
    // first open, so the cutoff lands on the **preceding** local date.
    let evening = [DayOverride::late_open(day(2025, 12, 26), 19 * 3_600)];
    let policy = StaticDayPolicy::new(&evening).expect("the fixture records are valid");
    let calendar = base.with_day_policy(&policy);
    assert!(base.is_open(ct((2025, 12, 25), (18, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 25), (18, 0, 0))));
    assert!(calendar.is_open(ct((2025, 12, 25), (19, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), (19, 0, 0))),
        Some(day(2025, 12, 26))
    );

    // Branch two: 09:00 CT is before that first open, so the cutoff lands on
    // the trade date itself and the whole evening leg goes.
    let morning = [DayOverride::late_open(day(2025, 12, 26), 9 * 3_600)];
    let policy = StaticDayPolicy::new(&morning).expect("the fixture records are valid");
    let calendar = base.with_day_policy(&policy);
    assert!(!calendar.is_open(ct((2025, 12, 25), (20, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 26), (8, 45, 0))));
    assert!(calendar.is_open(ct((2025, 12, 26), (9, 0, 0))));
    // The regular session now opens at the cutoff. It is its own session --
    // this family does not join adjacent phases -- so the 15:15 CT regular
    // close still bounds it.
    assert_eq!(
        calendar.session_bounds(ct((2025, 12, 26), (10, 0, 0))),
        Some((
            ct((2025, 12, 26), (9, 0, 0)),
            ct((2025, 12, 26), (15, 15, 0))
        ))
    );
}

/// One trade date can carry both a late open and an early close, and their
/// numeric order is deliberately unconstrained: a wrapped trading day opens on
/// the preceding local date at a numerically later wall clock — 19:00 — than
/// its final close on the trade date — 12:00.
///
/// This is why the `holidays!` instant fence checks each boundary's range and
/// **not** `open < close`: a fence on their order would reject the sourced
/// shape this test encodes.
#[test]
fn a_late_open_and_an_early_close_compose_on_one_trade_date() {
    let overrides = [DayOverride::late_open_and_early_close(
        day(2025, 12, 26),
        19 * 3_600,
        12 * 3_600,
    )];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let calendar =
        calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex).with_day_policy(&policy);

    assert!(!calendar.is_open(ct((2025, 12, 25), (18, 0, 0))));
    assert!(calendar.is_open(ct((2025, 12, 25), (19, 0, 0))));
    assert!(calendar.is_open(ct((2025, 12, 26), (11, 59, 59))));
    assert!(!calendar.is_open(ct((2025, 12, 26), (12, 0, 0))));
    // The evening leg opens at the late-open cutoff on the preceding local
    // date and still hands over at 08:30 CT.
    assert_eq!(
        calendar.session_bounds(ct((2025, 12, 25), (20, 0, 0))),
        Some((
            ct((2025, 12, 25), (19, 0, 0)),
            ct((2025, 12, 26), (8, 30, 0))
        ))
    );
    // The regular session is clipped by the early close on the trade date.
    assert_eq!(
        calendar.session_bounds(ct((2025, 12, 26), (10, 0, 0))),
        Some((
            ct((2025, 12, 26), (8, 30, 0)),
            ct((2025, 12, 26), (12, 0, 0))
        ))
    );
}

// ---------------------------------------------------------------------------
// Layering and precedence.
// ---------------------------------------------------------------------------

/// A caller's replacement record resolves the trading day before anything
/// clips it, and the caller's own `DayPolicy` then overlays the replacement
/// exactly as it overlays a normal week.
#[test]
fn a_caller_replacement_resolves_the_day_before_a_clip_applies() {
    let source = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    let blocks = [exchange_hours::ExceptionBlock::regular(
        0,
        9 * 3_600,
        14 * 3_600,
    )];
    let records = [SessionExceptionRecord::replace_sessions(
        day(2026, 4, 20),
        &blocks,
    )];
    let table = StaticSessionExceptions::new(source, day(2026, 4, 1), day(2026, 4, 30), &records)
        .expect("the fixture table is valid");
    let overrides = [DayOverride::early_close(day(2026, 4, 20), 11 * 3_600)];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");

    let replaced = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar");
    assert_eq!(
        replaced.session_bounds(ct((2026, 4, 20), (10, 0, 0))),
        Some((ct((2026, 4, 20), (9, 0, 0)), ct((2026, 4, 20), (14, 0, 0))))
    );

    let clipped = replaced.with_day_policy(&policy);
    assert_eq!(
        clipped.session_bounds(ct((2026, 4, 20), (10, 0, 0))),
        Some((ct((2026, 4, 20), (9, 0, 0)), ct((2026, 4, 20), (11, 0, 0)))),
        "the policy clips the replacement, and never widens it"
    );
    assert!(!clipped.is_open(ct((2026, 4, 20), (11, 0, 0))));
}

/// `KnownNormal` is not an assertion that the layers below are wrong.
///
/// A provider returns `KnownNormal` both for a date it audited and found normal
/// and for a covered date it holds no record for, so the two are
/// indistinguishable at the trait level. Treating it as suppression would
/// silently disable every layer below for any caller who attaches a provider.
/// Suppression is `ReplaceSessions`; the coarse undo is `without_holidays`.
#[test]
fn a_known_normal_record_suppresses_nothing_below_it() {
    let source = CalendarSource::MarketHoursKey(MarketHoursKey::GlobexEquityIndex);
    let records = [SessionExceptionRecord::known_normal(day(2026, 4, 20))];
    let table = StaticSessionExceptions::new(source, day(2026, 4, 1), day(2026, 4, 30), &records)
        .expect("the fixture table is valid");
    let overrides = [DayOverride::closed(day(2026, 4, 20))];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");

    let calendar = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
        .with_session_exceptions(&table)
        .expect("the fixture is scoped to this calendar")
        .with_day_policy(&policy);

    assert!(
        calendar.is_closed_trade_date(day(2026, 4, 20), SessionKind::Both),
        "an audited-normal record must not veto the layer below it"
    );
    assert!(!calendar.is_open(ct((2026, 4, 20), (10, 0, 0))));
}

/// An out-of-range boundary makes a trade date **unavailable**, which is not
/// the same as closing it.
///
/// An invalid record is not evidence that the operator was shut, so it must not
/// feed the following-business-day roll: CME cryptocurrency keeps assigning its
/// continuous trading to that date rather than rolling past it. A closed record
/// on the same date does roll.
#[test]
fn an_out_of_range_boundary_is_unavailable_and_never_rolls_a_trade_date() {
    struct OutOfRange(NaiveDate);

    impl DayPolicy for OutOfRange {
        fn is_closed(&self, _trade_date: NaiveDate) -> bool {
            false
        }

        fn early_close_ssm(&self, trade_date: NaiveDate) -> Option<u32> {
            (trade_date == self.0).then_some(86_401)
        }
    }

    // 2026-06-19 is a Friday inside the permanent 24/7 cryptocurrency era, and
    // its trading is continuous: the Thursday-evening block's trade date is
    // what moves when a layer closes the Friday.
    let friday = day(2026, 6, 19);
    let base = calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency);
    let probe = ct((2026, 6, 18), (20, 0, 0));
    assert_eq!(base.trade_date(probe), Some(day(2026, 6, 19)));

    let invalid = OutOfRange(friday);
    let unavailable = base.with_day_policy(&invalid);
    assert!(
        !unavailable.is_open(probe),
        "an unavailable trade date takes its sessions with it"
    );
    assert_eq!(
        unavailable.trade_date(probe),
        None,
        "an invalid record must not roll a trade date; the date stays assigned \
         and unavailable, so nothing is left to report"
    );

    let overrides = [DayOverride::closed(friday)];
    let policy = StaticDayPolicy::new(&overrides).expect("the fixture records are valid");
    let closed = base.with_day_policy(&policy);
    assert!(
        closed.is_open(probe),
        "the roll exists so a closure deletes a trade date, never a day of trading"
    );
    assert_eq!(
        closed.trade_date(probe),
        Some(day(2026, 6, 22)),
        "a closed record rolls the continuous week to the next open business date"
    );
}
