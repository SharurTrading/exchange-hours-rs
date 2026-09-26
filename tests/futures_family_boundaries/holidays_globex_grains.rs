// SPDX-License-Identifier: MIT-0

//! `globex_grains` built-in holiday rows, 2025-2027 (LAW-HOLIDAY-SCOPE).
//!
//! Every probe is stated in America/Chicago wall clock and converted, so a DST
//! slip in either direction fails rather than passing on a coincidence. The
//! family's sourced grid wraps — an evening leg at 19:00 CT running to 07:45 CT
//! the next morning, then an 08:30-13:20 CT day session whose close is the
//! trading day's final close — so each case below is chosen to exercise the
//! clip on the civil day the operator states it, not on the day the session
//! opened.
//!
//! The dates are CME's own: Christmas 2025 (Thursday closure, 12:05 CT
//! Christmas Eve, no Friday evening leg), the day after Thanksgiving 2025
//! (08:30-12:05 CT and nothing else) and Martin Luther King Jr. Day 2025 (a
//! Monday whose trade date does not exist, while the Monday 19:00 CT open that
//! begins Tuesday's trade date runs normally).
//!
//! The file holds two kinds of claim, and Stage 2B's coverage contract
//! (LAW-COVERAGE) separates them. Every case stated **at or above** the
//! permanent 2025-01-01 support floor still fences the schedule itself: the
//! session bounds, the trade date, the end-exclusive close and the candle. The
//! `era_*` and `wave2_*` cases state the **audited rows** the family has
//! shipped since 2010 — their dates, kinds, instants, tiers, order and the
//! era's totals — and no schedule answer for them survives, because every date
//! they audit precedes the floor. Those cases therefore state the refusal
//! itself, on the same dates and through the same entry points the schedule
//! probes used, and their row assertions carry the rest of the fence.

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, CalendarSource, CoverageGapReason, DateCoverage,
    EvidenceTier, ExceptionBlockKind, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, SessionState, calendar_for_exchange, calendar_for_market_hours_key,
    hours_for_market_hours_key,
};

const ZC: MarketHoursKey = MarketHoursKey::GlobexGrains;

/// A probe instant stated in the venue's own wall clock.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(date: (i32, u32, u32)) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn open_at(instant: DateTime<Utc>) -> bool {
    calendar_for_market_hours_key(ZC)
        .is_open(instant)
        .expect("the coverage contract must answer a covered date")
}

/// The identity-backed calendar a coverage refusal names.
///
/// `CalendarSource` is `#[non_exhaustive]`, so a match outside the crate needs
/// a wildcard arm even though the two variants below are the whole enum. The
/// `None` it produces is unreachable from anything this file can build.
fn calendar_of(source: CalendarSource) -> ExchangeCalendar {
    match source {
        CalendarSource::Exchange(exchange) => Some(calendar_for_exchange(exchange)),
        CalendarSource::MarketHoursKey(key) => Some(calendar_for_market_hours_key(key)),
        _ => None,
    }
    .expect("CalendarSource has exactly these two variants")
}

/// Asserts that an identity-backed date-aware query refused, with the error the
/// identity's own coverage declares for the venue-local date it could not
/// establish (LAW-COVERAGE).
///
/// Stage 2B fixes the support floor at 2025-01-01, so every audited row the
/// pre-2025 eras below ship is still the crate's claim while no schedule
/// question about those dates has an answer. Each probe therefore states the
/// refusal where it used to state an open, a session bound or a trade date, and
/// the verdict is read from `CalendarCoverage::coverage_on` instead of being
/// copied beside the probe: `BeforeSupportFloor` below the floor, and
/// `OutsideCoveredRange` at or above it on a date the holiday layer has no
/// audited answer for, which is where this table's windows stop at 2027-12-31.
///
/// The row, kind, tier, window and count assertions beside each probe are the
/// part of the old claim the fixed snapshot still states.
fn assert_declared_refusal<T: std::fmt::Debug>(result: Result<T, CalendarQueryError>) {
    let error = result.expect_err("the query must state its coverage refusal, not an answer");
    let coverage = calendar_of(error.source()).coverage();
    let day = error.date();
    let verdict = coverage.coverage_on(day);
    let states_verdict = matches!(
        (verdict, error),
        (
            DateCoverage::BeforeSupportFloor,
            CalendarQueryError::BeforeSupportFloor { .. }
        ) | (
            DateCoverage::OutsideCoveredRange,
            CalendarQueryError::OutsideCoveredRange { .. }
        ) | (
            DateCoverage::UnresolvedGap,
            CalendarQueryError::UnresolvedGap { .. }
        )
    );
    assert!(
        states_verdict,
        "{day} carries the verdict {verdict:?}: {error:?} does not state it"
    );
}

/// Case 1 — a closed trade date removes its whole trading day, including the
/// leg that opened the previous evening, and leaves the evening leg that begins
/// the *next* trade date alone.
#[test]
fn christmas_2025_closes_its_trade_date_and_the_prior_evening_leg() {
    let calendar = calendar_for_market_hours_key(ZC);
    let closed = day((2025, 12, 25));

    assert_eq!(
        calendar.holiday_on(closed).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );
    assert!(
        calendar
            .is_closed_trade_date(closed, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );

    for probe in [(2, 0, 0), (9, 0, 0), (12, 0, 0), (20, 0, 0)] {
        assert!(
            !open_at(ct((2025, 12, 25), probe)),
            "no session belongs to trade date 2025-12-25 at {probe:?} CT"
        );
    }

    // The Wednesday-evening leg would have carried trade date 2025-12-25.
    assert!(!open_at(ct((2025, 12, 24), (19, 0, 0))));
    assert!(!open_at(ct((2025, 12, 24), (23, 30, 0))));
}

/// Case 1, the converse — the Monday holiday's own trade date is gone, but the
/// Monday 19:00 CT open that begins Tuesday's trade date is untouched.
#[test]
fn mlk_2025_removes_the_trade_date_not_the_evening_leg_that_begins_the_next() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.holiday_on(day((2025, 1, 20))).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );
    // The Sunday-evening leg fed trade date 2025-01-20 and is deleted with it.
    assert!(!open_at(ct((2025, 1, 19), (19, 30, 0))));
    assert!(!open_at(ct((2025, 1, 20), (9, 0, 0))));
    assert!(!open_at(ct((2025, 1, 20), (12, 0, 0))));
    // The Monday-evening leg carries trade date 2025-01-21 and runs normally.
    assert!(open_at(ct((2025, 1, 20), (19, 30, 0))));
    assert!(open_at(ct((2025, 1, 21), (9, 0, 0))));
}

/// Case 2 — the instant before an early close is still open.
#[test]
fn christmas_eve_2025_is_open_one_second_before_the_early_close() {
    assert!(open_at(
        ct((2025, 12, 24), (12, 5, 0)) - Duration::seconds(1)
    ));
    assert!(open_at(ct((2025, 12, 24), (10, 0, 0))));
}

/// Case 3 — the early close is end-exclusive, ends the session and the daily
/// bar, and deletes what the normal week would have run after it.
#[test]
fn christmas_eve_2025_closes_at_1205_central() {
    let calendar = calendar_for_market_hours_key(ZC);
    let cutoff = ct((2025, 12, 24), (12, 5, 0));
    let probe = ct((2025, 12, 24), (10, 0, 0));

    assert!(!open_at(cutoff));
    assert_eq!(
        calendar
            .session_bounds(probe)
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2025, 12, 24), (8, 30, 0)), cutoff)),
    );
    assert_eq!(
        calendar
            .candle_end(probe, CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(cutoff),
    );
    // The normal week runs the day session to 13:20 CT; the row removes it.
    assert!(!open_at(ct((2025, 12, 24), (13, 0, 0))));
}

/// Case 4 — a late open stated at 08:30 CT, which is earlier than the trading
/// day's normal 19:00 CT first open, so the cutoff lands on the trade date
/// itself and deletes the prior-evening leg rather than the day session.
#[test]
fn boxing_day_2025_opens_late_at_0830_central() {
    let cutoff = ct((2025, 12, 26), (8, 30, 0));

    // Thursday 19:00 CT would normally have opened trade date 2025-12-26.
    assert!(!open_at(ct((2025, 12, 25), (19, 30, 0))));
    assert!(!open_at(ct((2025, 12, 26), (6, 0, 0))));
    assert!(!open_at(cutoff - Duration::seconds(1)));
    assert!(open_at(cutoff));
    assert_eq!(
        calendar_for_market_hours_key(ZC)
            .session_bounds(cutoff)
            .expect("the coverage contract must answer a covered date"),
        Some((cutoff, ct((2025, 12, 26), (13, 20, 0)))),
    );
}

/// Case 4 again, on the row that moves both boundaries: the day after
/// Thanksgiving 2025 is 08:30-12:05 CT and nothing else.
#[test]
fn day_after_thanksgiving_2025_is_a_single_0830_to_1205_block() {
    let calendar = calendar_for_market_hours_key(ZC);
    let open = ct((2025, 11, 28), (8, 30, 0));
    let close = ct((2025, 11, 28), (12, 5, 0));

    assert_eq!(
        calendar.holiday_on(day((2025, 11, 28))).map(Holiday::kind),
        Some(HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        }),
    );
    assert!(!open_at(ct((2025, 11, 27), (19, 30, 0))));
    assert!(!open_at(open - Duration::seconds(1)));
    assert!(open_at(open));
    assert!(open_at(close - Duration::seconds(1)));
    assert!(!open_at(close));
    assert_eq!(
        calendar
            .session_bounds(open)
            .expect("the coverage contract must answer a covered date"),
        Some((open, close))
    );
}

/// Case 5 — the removed wrap, read through the query a consumer would use:
/// after Christmas Eve's 12:05 CT close the next grain session is Friday's
/// 08:30 CT day open, because both intervening evening legs are gone.
#[test]
fn next_session_after_christmas_eve_2025_is_the_friday_day_open() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 12, 24), (12, 10, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 12, 26), (8, 30, 0))),
    );
    assert_eq!(
        calendar
            .next_session_after(ct((2025, 12, 24), (12, 10, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 12, 26), (8, 30, 0)),
            ct((2025, 12, 26), (13, 20, 0))
        )),
    );
}

/// Case 6 — the trade-date consequence. A shortened day keeps its own trade
/// date; the holiday evening's open carries the post-holiday date; and the
/// evening leg that fed a closed date has no trade date at all because it has
/// no session.
#[test]
fn holiday_rows_move_the_trade_date_the_way_cme_prints_it() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 24), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 12, 24))),
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 11, 28))),
    );
    // CME prints "19:00 open (trade date 2025-01-21)" on MLK Monday.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 20), (19, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 1, 21))),
    );
    // The Sunday leg that would have fed the closed Monday is gone entirely.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 19), (19, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        None
    );
}

/// Case 7 — both edges of the declared coverage window. Inside it a date with
/// no row is audited normal; outside it the table has no answer.
///
/// Neither edge is answerable: 2009-12-25 is below the 2025-01-01 support floor
/// and 2028-01-17 is above the table's last audited window, so the "the normal
/// week is served unmodified" half of this case is gone and each probe states
/// the refusal its own date carries.
#[test]
fn coverage_window_is_declared_and_does_not_extend() {
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a holiday table");

    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(coverage.contains(day((2010, 1, 1))));
    assert!(coverage.contains(day((2027, 12, 31))));
    assert!(!coverage.contains(day((2009, 12, 31))));
    assert!(!coverage.contains(day((2028, 1, 1))));

    assert_eq!(calendar.holiday_on(day((2009, 12, 31))), None);
    assert_eq!(calendar.holiday_on(day((2028, 1, 1))), None);

    // Christmas 2009 is a real CME closure below the window; the table says
    // nothing about it and the normal week answers. Christmas 2009 fell on a
    // Friday, whose 09:30-13:15 CT day session is over by 13:15, so the probe
    // is the Thursday morning session that traded normally into it.
    assert_eq!(calendar.holiday_on(day((2009, 12, 25))), None);
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2009, 12, 24), (9, 30, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2009, 12, 24), (12, 0, 0))),
    );

    // Martin Luther King Jr. Day 2028 is above the window; the table does not
    // extrapolate the pattern it holds for 2025, 2026 and 2027.
    assert_eq!(calendar.holiday_on(day((2028, 1, 17))), None);
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2028, 1, 17), (9, 0, 0))),
    );
}

/// The closure eves state the **complete** trading day, and the post-close queue
/// CME publishes on each of them is served rather than deleted with the closure
/// that follows.
///
/// The row is keyed to the eve's own trade date because the operator prints the
/// eve's `14:30 pcp` carrying that date; the crate dates an order-entry
/// occurrence by the session it feeds, so without the row the queue belongs to
/// the closed holiday and the neighbouring `Closed` row removes it. The
/// withheld evening instants stay closed and the eve's own prior-evening leg is
/// still traded, both asserted here.
#[test]
fn a_closure_eve_states_the_complete_day_and_keeps_its_post_close_queue() {
    const EVES: [((i32, u32, u32), &str); 14] = [
        ((2025, 4, 17), "CME-SVC-2025-04-17"),
        ((2025, 6, 18), "CME-SVC-2025-06-18"),
        ((2025, 7, 3), "CME-SVC-2025-07-03"),
        ((2025, 11, 26), "CME-SVC-2025-11-26-SAT"),
        ((2025, 12, 31), "CME-SVC-2025-12-31"),
        ((2026, 4, 2), "CME-SVC-2026-04-01"),
        ((2026, 6, 18), "CME-SVC-2026-06-18"),
        ((2026, 7, 2), "CME-SVC-2026-07-02"),
        ((2026, 11, 25), "CME-SVC-2026-11-25"),
        ((2026, 12, 31), "CME-SVC-2026-12-31"),
        ((2027, 3, 25), "CME-SVC-2027-03-25"),
        ((2027, 6, 17), "CME-SVC-2027-06-17"),
        ((2027, 11, 24), "CME-SVC-2027-11-24"),
        ((2027, 12, 23), "CME-SVC-2027-12-22"),
    ];
    // The block set: the ordinary prior-evening queue and leg, the morning
    // queue, the day session and the post-close queue.
    const EXPECTED: [(ExceptionBlockKind, i8, u32, u32); 5] = [
        (
            ExceptionBlockKind::OrderEntry,
            -1,
            16 * 3_600 + 45 * 60,
            19 * 3_600,
        ),
        (
            ExceptionBlockKind::Extended,
            -1,
            19 * 3_600,
            7 * 3_600 + 45 * 60,
        ),
        (
            ExceptionBlockKind::OrderEntry,
            0,
            8 * 3_600,
            8 * 3_600 + 30 * 60,
        ),
        (
            ExceptionBlockKind::Regular,
            0,
            8 * 3_600 + 30 * 60,
            13 * 3_600 + 20 * 60,
        ),
        (
            ExceptionBlockKind::OrderEntry,
            0,
            14 * 3_600 + 30 * 60,
            16 * 3_600,
        ),
    ];
    let calendar = calendar_for_market_hours_key(ZC);
    for (eve, document) in EVES {
        let row = calendar
            .holiday_on(day(eve))
            .unwrap_or_else(|| panic!("{eve:?} must ship a row"));
        assert_eq!(row.tier(), EvidenceTier::T2, "{eve:?}");
        assert_eq!(row.document_id(), document, "{eve:?} cites its own window");
        let HolidayKind::ReplacementBlocks(blocks) = row.kind() else {
            panic!("{eve:?} must state the whole day as replacement blocks");
        };
        assert_eq!(blocks.len(), EXPECTED.len(), "{eve:?}");
        for (block, (kind, offset, open_ssm, close_ssm)) in blocks.iter().zip(EXPECTED) {
            assert_eq!(
                (
                    block.kind(),
                    block.open_day_offset(),
                    block.open_ssm(),
                    block.close_ssm()
                ),
                (kind, offset, open_ssm, close_ssm),
                "{eve:?}"
            );
        }

        // The queue CME publishes on the eve: 14:30-16:00 CT accepts orders and
        // matches nothing, at both ends of the window.
        for time in [(15, 0, 0), (15, 59, 59)] {
            let instant = ct(eve, time);
            assert_eq!(
                calendar.session_state(instant),
                Ok(SessionState::OrderEntry),
                "{eve:?} at {time:?}"
            );
            assert_eq!(
                calendar.is_accepting_orders(instant),
                Ok(true),
                "{eve:?} at {time:?}"
            );
            assert_eq!(
                calendar.is_order_entry_only(instant),
                Ok(true),
                "{eve:?} at {time:?}"
            );
            assert!(!open_at(instant), "{eve:?} at {time:?} matches nothing");
        }
        // CME withholds the evening leg, and it stays withheld: the queue ends
        // at 16:00 and the neighbouring closure owns everything after it.
        for time in [(16, 45, 0), (17, 30, 0), (20, 0, 0)] {
            assert!(
                !open_at(ct(eve, time)),
                "{eve:?} at {time:?} has no evening leg"
            );
        }
        // The day session's own close is end-exclusive at 13:20 CT.
        assert!(open_at(ct(eve, (13, 19, 59))), "{eve:?}");
        assert!(!open_at(ct(eve, (13, 20, 0))), "{eve:?}");

        // The eve's own prior-evening leg is still traded, and it belongs to the
        // eve's trade date.
        let prior = day(eve).pred_opt().expect("the eve has a preceding day");
        let leg = US::Central
            .from_local_datetime(
                &prior
                    .and_hms_opt(20, 0, 0)
                    .expect("20:00 is a valid local time"),
            )
            .single()
            .expect("the prior evening has one 20:00 CT instant")
            .with_timezone(&Utc);
        assert_eq!(
            calendar.session_state(leg),
            Ok(SessionState::OpenExtended),
            "{eve:?}: the prior-evening leg"
        );
        assert_eq!(
            calendar.trade_date(leg),
            Ok(Some(day(eve))),
            "{eve:?}: the prior-evening leg carries the eve's trade date"
        );
    }
}

/// The four trade dates after a mid-week closure state the `06:00 preopen` CME
/// prints instead of the ordinary `08:00` one, then the ordinary day.
///
/// A scalar `late open` moved the open correctly and stated no queue at all, so
/// every instant of the window the operator publishes answered "not accepting".
#[test]
fn the_day_after_a_closure_states_the_operators_pre_open() {
    const DATES: [(i32, u32, u32); 4] = [(2025, 1, 2), (2025, 12, 26), (2026, 1, 2), (2027, 7, 6)];
    let calendar = calendar_for_market_hours_key(ZC);
    for date in DATES {
        let row = calendar
            .holiday_on(day(date))
            .unwrap_or_else(|| panic!("{date:?} must ship a row"));
        let HolidayKind::ReplacementBlocks(blocks) = row.kind() else {
            panic!("{date:?} must state the whole day as replacement blocks");
        };
        assert_eq!(blocks.len(), 3, "{date:?}");
        assert_eq!(
            (
                blocks[0].kind(),
                blocks[0].open_day_offset(),
                blocks[0].open_ssm(),
                blocks[0].close_ssm(),
            ),
            (
                ExceptionBlockKind::OrderEntry,
                0,
                6 * 3_600,
                8 * 3_600 + 30 * 60
            ),
            "{date:?}: the operator's 06:00 CT pre-open"
        );
        assert_eq!(
            (
                blocks[1].kind(),
                blocks[1].open_ssm(),
                blocks[1].close_ssm(),
            ),
            (
                ExceptionBlockKind::Regular,
                8 * 3_600 + 30 * 60,
                13 * 3_600 + 20 * 60,
            ),
            "{date:?}: the ordinary day session"
        );

        // The printed window is order entry at both ends, and the ordinary
        // `08:00-08:30` queue is inside it rather than beside it.
        for time in [(6, 0, 0), (6, 30, 0), (7, 59, 59), (8, 0, 0), (8, 29, 59)] {
            assert_eq!(
                calendar.is_order_entry_only(ct(date, time)),
                Ok(true),
                "{date:?} at {time:?}"
            );
            assert!(
                !open_at(ct(date, time)),
                "{date:?} at {time:?} matches nothing"
            );
        }
        assert!(open_at(ct(date, (8, 30, 0))), "{date:?}");
        assert!(open_at(ct(date, (13, 19, 59))), "{date:?}");
        assert!(
            !open_at(ct(date, (13, 20, 0))),
            "{date:?}: end-exclusive close"
        );

        // The trade date's own post-close queue is stated too, and it is order
        // entry rather than a session.
        assert_eq!(
            calendar.is_order_entry_only(ct(date, (15, 0, 0))),
            Ok(true),
            "{date:?}"
        );
        assert_eq!(
            calendar.trade_date(ct(date, (10, 0, 0))),
            Ok(Some(day(date))),
            "{date:?}"
        );

        // No prior-evening leg ran. 2025-01-02's own evening falls on
        // 2025-01-01, where every query refuses below the support floor and the
        // refusal is a floor fact rather than this row's; the other three state
        // the closure outright.
        if date != (2025, 1, 2) {
            let prior = day(date).pred_opt().expect("the date has a preceding day");
            let leg = US::Central
                .from_local_datetime(
                    &prior
                        .and_hms_opt(19, 30, 0)
                        .expect("19:30 is a valid local time"),
                )
                .single()
                .expect("the prior evening has one 19:30 CT instant")
                .with_timezone(&Utc);
            assert!(
                !open_at(leg),
                "{date:?}: the closure left no prior-evening leg"
            );
        }
    }
}

/// The post-close queue's trade date is the session it feeds, not the date the
/// queue is printed on.
///
/// This is the deliberate divergence `globex_grains` declares under #152: CME's
/// service labels the `14:30 pcp` with the date it is printed on, and the crate
/// answers the next trade date because that is the session the queued orders
/// feed. The fence states both instants the evidence file quotes, so the
/// divergence cannot change silently — and it asserts the declaration that makes
/// the scope incomplete, so the inventory's `Complete?` cell cannot drift back.
#[test]
fn the_post_close_queue_carries_the_trade_date_of_the_session_it_feeds() {
    let calendar = calendar_for_market_hours_key(ZC);
    for (date, next) in [
        ((2025, 6, 10), (2025, 6, 11)),
        ((2025, 6, 13), (2025, 6, 16)),
    ] {
        for time in [(15, 0, 0), (15, 59, 59)] {
            let instant = ct(date, time);
            assert_eq!(
                calendar.session_state(instant),
                Ok(SessionState::OrderEntry),
                "{date:?} at {time:?}"
            );
            assert_eq!(
                calendar.is_accepting_orders(instant),
                Ok(true),
                "{date:?} at {time:?}"
            );
            assert_eq!(
                calendar.trade_date(instant),
                Ok(Some(day(next))),
                "{date:?} at {time:?}: CME prints the queue's own date, the crate answers the \
                 session it feeds"
            );
        }
        assert!(
            !calendar.coverage().is_complete_on(day(date)),
            "{date:?}: the declared label divergence makes every covered date incomplete"
        );
        assert_eq!(
            calendar.coverage().coverage_on(day(date)),
            DateCoverage::OutsideCoveredRange,
            "{date:?}"
        );
    }

    // The declaration is whole-domain and withholds nothing: the queue's own
    // window is answered on every covered date rather than refused.
    let declared: Vec<_> = calendar
        .coverage()
        .phase_gaps()
        .iter()
        .map(|gap| (gap.reason(), gap.closing_condition()))
        .collect();
    assert_eq!(
        declared,
        vec![(CoverageGapReason::PostCloseQueueTradeDateLabel, "#152")],
        "globex_grains declares exactly the post-close label divergence"
    );
}

/// `without_holidays` restores exactly the pre-table answer, over a dense
/// instant grid across the whole Christmas 2025 week and at each shipped row's
/// own probes. This is the consumer's escape hatch and the benchmark control.
#[test]
fn without_holidays_reproduces_the_normal_week() {
    let detached = calendar_for_market_hours_key(ZC).without_holidays();

    assert_eq!(detached.holiday_coverage(), None);
    assert_eq!(detached.holiday_on(day((2025, 12, 25))), None);
    assert!(
        detached
            .is_open(ct((2025, 12, 25), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        detached
            .is_open(ct((2025, 12, 24), (13, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        detached
            .is_open(ct((2025, 12, 25), (19, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        detached
            .is_open(ct((2025, 1, 20), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        detached
            .is_open(ct((2025, 11, 28), (13, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    let mut probe = ct((2025, 12, 21), (0, 0, 0));
    let end = ct((2025, 12, 29), (0, 0, 0));
    while probe < end {
        assert_eq!(
            detached
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            hours_for_market_hours_key(ZC, probe).is_open(probe),
            "detached calendar must equal the identity-erased snapshot at {probe}"
        );
        probe += Duration::minutes(5);
    }
}

// ---------------------------------------------------------------------------
// The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 12:00 CT, the only early final close this family's 2010-2012 block carries.
const ERA_HALF_DAY_CLOSE: u32 = 12 * 3_600;

/// The era's early close clipped the day session of a trading day that opened
/// 18:00 CT the previous evening: 2010-11-26 stopped at 12:00 CT, before the
/// regular session's own 13:15 CT close, and every other early close in the
/// block states the same instant.
///
/// The clip is a pre-floor schedule answer, so the instants the block states
/// are asserted through the rows and every probe that read the clip now states
/// the refusal.
#[test]
fn era_early_closes_end_the_day_session_at_1200_central() {
    let calendar = calendar_for_market_hours_key(ZC);
    let row = calendar
        .holiday_on(day((2010, 11, 26)))
        .expect("2010-11-26 ships a row");
    assert_eq!(
        row.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_HALF_DAY_CLOSE
        }
    );

    // The Thursday-evening leg and the trade date it carried.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2010, 11, 25), (18, 0, 0))),
    );
    assert_declared_refusal(calendar.trade_date(ct((2010, 11, 25), (18, 0, 0))));
    // Both sides of the printed close, the rest of the regular 09:30-13:15 CT
    // session, the day session's bounds and the trade date.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2010, 11, 26), (11, 59, 59))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2010, 11, 26), (12, 0, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2010, 11, 26), (13, 0, 0))),
    );
    assert_declared_refusal(calendar.session_bounds(ct((2010, 11, 26), (11, 0, 0))));
    assert_declared_refusal(calendar.trade_date(ct((2010, 11, 26), (12, 0, 0))));

    // Every other early close in the block states the same 12:00 CT instant;
    // both sides of it are pre-floor probes and state the refusal.
    for date in [(2010, 12, 31), (2011, 11, 25), (2012, 7, 3), (2012, 12, 24)] {
        let row = calendar
            .holiday_on(day(date))
            .unwrap_or_else(|| panic!("{date:?} ships a row"));
        assert_eq!(
            row.kind(),
            HolidayKind::EarlyClose {
                close_ssm: ERA_HALF_DAY_CLOSE
            },
            "{date:?}"
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (11, 59, 59))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (12, 0, 0))));
    }
}

/// The era's late opens split by where their cutoff lands, exactly as the
/// scalar vocabulary resolves it: an instant earlier than the trading day's
/// normal first open lands on the trade date itself, and one at or after it
/// lands on the preceding local date.
///
/// 2011-12-27 states 09:30 CT, earlier than the era's 18:00 CT evening open,
/// so trade date 2011-12-27 begins on its own civil day.
///
/// 2012-05-28 states 19:00 CT, at or after the 17:00 CT open in force from
/// 2012-05-20, so the cutoff lands on the preceding local date. That is exactly
/// what CME states: the sheet prints `1900 CT (Mon May 28, for trade date
/// Tue May 29)`, and the crate's grid opens the evening leg *for trade date D*
/// at 17:00 CT on `D - 1`, so on 05-28 itself the 19:00 instant withholds the
/// Sunday-evening leg that would have fed 05-28 and the ordinary 17:00 CT leg
/// still opens 05-29. Keying the row to 05-29 instead would put the cutoff at
/// 19:00 on 05-28 — precisely when that evening leg opens — and change no
/// answer at all. The residual, that CME prints 05-29 as the instant's own
/// trade date, is recorded in the family's evidence file.
///
/// Which civil day each cutoff landed on is a pre-floor schedule answer; the
/// rows state the instants, and every probe that read the landing states the
/// refusal.
#[test]
fn era_late_opens_land_where_the_normal_first_open_puts_them() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar
            .holiday_on(day((2011, 12, 27)))
            .expect("2011-12-27 ships a row")
            .kind(),
        HolidayKind::LateOpen {
            open_ssm: 9 * 3_600 + 30 * 60
        }
    );
    // The prior evening, both sides of the printed 09:30 CT open, and the
    // session bounds it produced.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2011, 12, 26), (18, 0, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2011, 12, 27), (9, 29, 59))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2011, 12, 27), (9, 30, 0))),
    );
    assert_declared_refusal(calendar.session_bounds(ct((2011, 12, 27), (9, 30, 0))));

    assert_eq!(
        calendar
            .holiday_on(day((2012, 5, 28)))
            .expect("2012-05-28 ships a row")
            .kind(),
        HolidayKind::LateOpen {
            open_ssm: 19 * 3_600
        }
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 5, 27), (18, 59, 59))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 5, 27), (19, 0, 0))),
    );
    assert_declared_refusal(calendar.session_bounds(ct((2012, 5, 27), (19, 0, 0))));
    assert_declared_refusal(calendar.trade_date(ct((2012, 5, 27), (19, 0, 0))));
    // The holiday's own evening leg, which opened at the ordinary 17:00 CT.
    assert_declared_refusal(calendar.trade_date(ct((2012, 5, 28), (17, 30, 0))));
}

/// Thanksgiving Friday 2012 is the era's one late-open-and-early-close row: no
/// prior-evening leg and a single 09:30-12:00 CT block, after which the next
/// session is the ordinary Sunday 17:00 CT open.
///
/// The row is the claim that survives; each boundary the block described is a
/// pre-floor answer and is stated as the refusal.
#[test]
fn era_late_open_and_early_close_2012_11_23_is_one_0930_to_1200_block() {
    let calendar = calendar_for_market_hours_key(ZC);
    let holiday = calendar
        .holiday_on(day((2012, 11, 23)))
        .expect("2012-11-23 ships a row");
    assert_eq!(
        holiday.kind(),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 9 * 3_600 + 30 * 60,
            close_ssm: ERA_HALF_DAY_CLOSE,
        }
    );

    // The Wednesday-evening leg that would have carried this trade date, both
    // sides of the 09:30-12:00 block, its bounds and the next open.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 11, 22), (17, 0, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 11, 23), (9, 29, 59))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 11, 23), (9, 30, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 11, 23), (11, 59, 59))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 11, 23), (12, 0, 0))),
    );
    assert_declared_refusal(calendar.session_bounds(ct((2012, 11, 23), (10, 0, 0))));
    assert_declared_refusal(calendar.next_session_open_after(ct((2012, 11, 23), (12, 10, 0))));
    // Thanksgiving Day itself is a full closure in this family: CME's 2012
    // sheet prints the reopening for Friday 2012-11-23 at 09:30 CT.
    assert_eq!(
        calendar
            .holiday_on(day((2012, 11, 22)))
            .expect("2012-11-22 ships a row")
            .kind(),
        HolidayKind::Closed
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2012, 11, 22), (9, 0, 0))),
    );
}

/// A date inside the widened window with no row is audited normal under both of
/// the era's grids: 2010's 18:00 CT evening open and, from 2012-05-20, the
/// 17:00 CT one.
///
/// With both dates below the floor, the equality the attached and detached
/// calendars used to satisfy is not observable — both refuse every probe — so
/// what remains is that each date ships no row.
#[test]
fn era_dates_without_rows_are_audited_normal_under_both_grids() {
    let calendar = calendar_for_market_hours_key(ZC);
    let detached = calendar.without_holidays();

    for (date, previous_day, evening_open) in [
        ((2010, 6, 15), (2010, 6, 14), (18, 0)),
        ((2012, 6, 15), (2012, 6, 14), (17, 0)),
    ] {
        assert_eq!(calendar.holiday_on(day(date)), None, "{date:?}");
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC)
                .is_open(ct(previous_day, (evening_open.0, evening_open.1, 0))),
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (10, 0, 0))));
        assert_declared_refusal(
            calendar.session_bounds(ct(previous_day, (evening_open.0, evening_open.1, 0))),
        );
        assert_declared_refusal(
            detached.session_bounds(ct(previous_day, (evening_open.0, evening_open.1, 0))),
        );
    }
}

/// The widened window's edges as the module declares them: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// carrying no row.
///
/// The schedule half of this fence cannot survive the floor. 2009-12-25 is
/// below 2025-01-01, so neither the attached calendar's "the crate serves the
/// normal Wednesday" nor the detached calendar's agreement with it is
/// observable any more; both refuse, and that is what the probes now state.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = calendar_for_market_hours_key(ZC);
    let detached = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");

    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(!coverage.contains(day((2009, 12, 25))));

    assert_eq!(calendar.holiday_on(day((2009, 12, 25))), None);
    assert_declared_refusal(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_declared_refusal(detached.is_open(ct((2009, 12, 25), (10, 0, 0))));
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// 08:30 CT is this era's day-session open, and the three day-after-Thanksgiving
/// rows carry it as a late open beside the 12:05 CT early close: CME withdrew
/// the prior-evening leg and printed the morning session instead.
///
/// The instants live in the rows and are still asserted; the boundaries those
/// rows moved are below the floor, so each probe states the refusal.
#[test]
fn wave2_day_after_thanksgiving_opens_late_and_closes_early() {
    let calendar = calendar_for_market_hours_key(ZC);

    for date in [(2016, 11, 25), (2017, 11, 24), (2018, 11, 23)] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: 8 * 3_600 + 30 * 60,
                close_ssm: 12 * 3_600 + 5 * 60,
            }),
            "{date:?}"
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (8, 29, 0))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (8, 30, 0))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (12, 4, 0))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (12, 5, 0))));
        // The prior-evening leg that would have fed this trade date is gone:
        // the evening before is the holiday itself.
        let eve = (date.0, date.1, date.2 - 1);
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(eve, (19, 30, 0))));
        assert!(
            calendar.holiday_on(day(eve)).map(Holiday::kind) == Some(HolidayKind::Closed),
            "{date:?}: the eve must ship the closure"
        );
        // These three dates are Fridays: the grid has no Friday-evening leg,
        // so the next session was the Sunday 19:00 CT open into Monday's trade
        // date, exactly as CME publishes it. Below the floor it is refused.
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC)
                .is_open(ct((date.0, date.1, date.2 + 3), (19, 30, 0))),
        );
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC)
                .trade_date(ct((date.0, date.1, date.2 + 3), (19, 30, 0))),
        );
    }
}

/// The era's plain early closes are 12:05 CT with the ordinary evening leg
/// intact, and 2018-12-26 opens late at 08:30 CT after the Christmas closure.
///
/// Both shapes are pre-floor schedule answers: the rows state the instants, and
/// every probe that read the leg or the moved open states the refusal.
#[test]
fn wave2_early_closes_and_the_late_open_land_on_the_printed_instants() {
    let calendar = calendar_for_market_hours_key(ZC);

    for (date, eve) in [
        ((2016, 12, 23), (2016, 12, 22)),
        ((2017, 7, 3), (2017, 7, 2)),
        ((2018, 7, 3), (2018, 7, 2)),
        ((2018, 12, 24), (2018, 12, 23)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 12 * 3_600 + 5 * 60
            }),
            "{date:?}"
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(eve, (19, 30, 0))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (12, 4, 0))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (12, 5, 0))));
    }

    assert_eq!(
        calendar.holiday_on(day((2018, 12, 26))).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60
        })
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2018, 12, 26), (8, 29, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2018, 12, 26), (8, 30, 0))),
    );
    assert_declared_refusal(calendar.trade_date(ct((2018, 12, 26), (9, 0, 0))));
}

/// Every early close the era ships states 12:05 CT, and every combined row states
/// 08:30 CT beside it: the instants, not only the kinds, are the fence.
///
/// The venue tables and the family counts both survive a one-minute change to an
/// early close's instant, so this walks the era and pins each one.
#[test]
fn wave2_every_early_close_instant_is_the_printed_one() {
    let calendar = calendar_for_market_hours_key(ZC);
    let (mut plain, mut combined) = (0_usize, 0_usize);
    let mut date = day((2016, 1, 1));
    while date <= day((2018, 12, 31)) {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::EarlyClose { close_ssm }) => {
                assert_eq!(close_ssm, 12 * 3_600 + 5 * 60, "{date}");
                plain += 1;
            }
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm,
                close_ssm,
            }) => {
                assert_eq!(open_ssm, 8 * 3_600 + 30 * 60, "{date}");
                assert_eq!(close_ssm, 12 * 3_600 + 5 * 60, "{date}");
                combined += 1;
            }
            Some(HolidayKind::LateOpen { open_ssm }) => {
                assert_eq!(open_ssm, 8 * 3_600 + 30 * 60, "{date}");
            }
            _ => {}
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the era stays inside the representable calendar");
    }
    assert_eq!((plain, combined), (5, 3), "the era's early closes");
}

/// The era's shape counts, and its closures.
///
/// The counts, the rows' kinds and the closure date set are unchanged; the
/// schedule the closing paragraph describes is pre-floor and is refused.
#[test]
fn wave2_ships_twenty_seven_closures_and_no_unsourced_row() {
    let calendar = calendar_for_market_hours_key(ZC);
    let (mut closures, mut early_closes, mut late_opens, mut combined) = (0, 0, 0, 0);
    let mut date = day((2016, 1, 1));
    while date <= day((2018, 12, 31)) {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::Closed) => closures += 1,
            Some(HolidayKind::EarlyClose { .. }) => early_closes += 1,
            Some(HolidayKind::LateOpen { .. }) => late_opens += 1,
            Some(HolidayKind::LateOpenAndEarlyClose { .. }) => combined += 1,
            None => {}
            Some(other) => panic!("{date}: the era ships no {other:?}"),
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the era stays inside the representable calendar");
    }
    assert_eq!(
        (closures, early_closes, late_opens, combined),
        (27, 5, 1, 3)
    );

    // A closure removed the day session and the prior-evening leg, and the
    // Monday-evening leg that followed opened the next trade date: all three
    // are pre-floor probes and state the refusal.
    assert_eq!(
        calendar.holiday_on(day((2016, 3, 25))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2016, 3, 24), (19, 30, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2016, 3, 25), (9, 0, 0))),
    );
    assert_declared_refusal(calendar.trade_date(ct((2016, 3, 27), (20, 0, 0))));
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 19:00 CT, the ordinary evening leg that opens the next trade date.
const ERA_EVENING_OPEN: u32 = 19 * 3_600;
/// 08:30 CT, the day session's open and the late-open instant.
const ERA_DAY_OPEN: u32 = 8 * 3_600 + 30 * 60;
/// 13:20 CT, the day session's close, which every late open keeps.
const ERA_DAY_CLOSE: u32 = 13 * 3_600 + 20 * 60;
/// 12:05 CT, the era's only early final close.
const ERA_HALF_DAY_FIVE_PAST: u32 = 12 * 3_600 + 5 * 60;

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence: the sweep below walks the whole window
/// and compares against this list row for row, so a dropped, added or moved
/// row fails as loudly as a wrong instant. A sample would let a slipped close
/// move unnoticed on the dates nobody probed, and this family ships more
/// unusual rows than any other — six late opens, three late-open-and-early
/// closes and one early close.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2022, 1, 17), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 2, 21), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 5, 30), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 6, 20), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2022, 9, 5), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 11, 24), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 11, 25),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: ERA_DAY_OPEN,
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
        },
        EvidenceTier::T1,
    ),
    ((2022, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 16), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 5, 29), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 6, 19), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2023, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2023, 9, 4), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 11, 23), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2023, 11, 24),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: ERA_DAY_OPEN,
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2023, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2024, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2024, 1, 15), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 2, 19), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 5, 27), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 6, 19), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 7, 4), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T2,
    ),
    ((2024, 9, 2), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 11, 28), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 11, 29),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: ERA_DAY_OPEN,
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
        },
        EvidenceTier::T2,
    ),
    ((2024, 12, 25), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T2,
    ),
];

/// `ct` for a date the walk computed rather than spelled, so day arithmetic
/// cannot drift out of step with a hand-written tuple.
fn ct_on(date: NaiveDate, time: (u32, u32, u32)) -> DateTime<Utc> {
    ct((date.year(), date.month(), date.day()), time)
}

/// `ct_on` for an instant the module states in venue-local seconds since
/// midnight, so the sweep reads the same constant the row does.
fn ct_at(date: NaiveDate, ssm: u32) -> DateTime<Utc> {
    ct_on(date, (ssm / 3_600, (ssm % 3_600) / 60, ssm % 60))
}

fn day_before(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
///
/// The row half is unchanged — order, kind, instant, tier and the era's shape
/// all fail on a dropped, added or moved row. Every date the walk visits is
/// pre-floor, so the boundaries the branch probes fence are stated as the
/// refusal the coverage contract returns.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = calendar_for_market_hours_key(ZC);
    let mut index = 0_usize;
    let (mut late_opens, mut combined, mut early_closes) = (0_usize, 0_usize, 0_usize);
    let (mut closures, mut unsourced) = (0_usize, 0_usize);
    let mut date = day((2022, 1, 1));
    while date <= day((2024, 12, 31)) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, tier) = ERA_ROWS[index];
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the era's rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            match kind {
                HolidayKind::EarlyClose { close_ssm } => {
                    early_closes += 1;
                    assert_eq!(close_ssm, ERA_HALF_DAY_FIVE_PAST, "{date}");
                    let cutoff = ct_on(date, (12, 5, 0));
                    // The evening leg that opened this trade date, and the
                    // trade date it carried.
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC)
                            .is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
                    );
                    assert_declared_refusal(
                        calendar.trade_date(ct_on(day_before(date), (19, 30, 0))),
                    );
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC).is_open(cutoff - Duration::seconds(1)),
                    );
                    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(cutoff));
                    assert_declared_refusal(calendar.session_bounds(ct_on(date, (9, 0, 0))));
                    // The ordinary 13:20 CT day-session close, which the row
                    // does not reach.
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC).is_open(ct_at(date, ERA_DAY_CLOSE)),
                    );
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late_opens += 1;
                    assert_eq!(open_ssm, ERA_DAY_OPEN, "{date}");
                    let open = ct_on(date, (8, 30, 0));
                    // The prior-evening leg, both sides of the 08:30 CT day
                    // open, the session bounds, both sides of the ordinary
                    // 13:20 CT close, and the trade date.
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC)
                            .is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
                    );
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC)
                            .is_open(ct_on(day_before(date), (19, 30, 0))),
                    );
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC).is_open(open - Duration::seconds(1)),
                    );
                    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open));
                    assert_declared_refusal(calendar.session_bounds(open));
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC)
                            .is_open(ct_at(date, ERA_DAY_CLOSE) - Duration::seconds(1)),
                    );
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC).is_open(ct_at(date, ERA_DAY_CLOSE)),
                    );
                    assert_declared_refusal(calendar.trade_date(open));
                }
                HolidayKind::LateOpenAndEarlyClose {
                    open_ssm,
                    close_ssm,
                } => {
                    combined += 1;
                    assert_eq!(
                        (open_ssm, close_ssm),
                        (ERA_DAY_OPEN, ERA_HALF_DAY_FIVE_PAST),
                        "{date}"
                    );
                    let open = ct_on(date, (8, 30, 0));
                    let close = ct_on(date, (12, 5, 0));
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC)
                            .is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
                    );
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC).is_open(open - Duration::seconds(1)),
                    );
                    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open));
                    assert_declared_refusal(
                        calendar_for_market_hours_key(ZC).is_open(close - Duration::seconds(1)),
                    );
                    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(close));
                    assert_declared_refusal(calendar.session_bounds(open));
                    assert_declared_refusal(calendar.trade_date(open));
                }
                HolidayKind::Closed => closures += 1,
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: the era ships no {other:?}"),
            }
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_ROWS.len(), "every planned row ships");
    assert_eq!(
        (late_opens, combined, early_closes, closures, unsourced),
        (6, 3, 1, 26, 3),
        "the era's shape"
    );
}

/// The three days after Thanksgiving carry both boundaries — no prior-evening
/// leg and a single 08:30-12:05 CT block — and each follows CME's own
/// Thanksgiving closure.
///
/// The rows at each date, and the closure the operator pairs with them, are the
/// claims that survive; the boundaries they moved are below the floor and are
/// refused.
#[test]
fn era_2022_2024_days_after_thanksgiving_open_late_and_close_early() {
    let calendar = calendar_for_market_hours_key(ZC);

    for (date, thanksgiving) in [
        ((2022, 11, 25), (2022, 11, 24)),
        ((2023, 11, 24), (2023, 11, 23)),
        ((2024, 11, 29), (2024, 11, 28)),
    ] {
        let open = ct(date, (8, 30, 0));
        let close = ct(date, (12, 5, 0));
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: ERA_DAY_OPEN,
                close_ssm: ERA_HALF_DAY_FIVE_PAST,
            }),
            "{date:?}"
        );
        // The Thursday closure the operator pairs with it.
        assert_eq!(
            calendar.holiday_on(day(thanksgiving)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{thanksgiving:?}"
        );
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC).is_open(ct(thanksgiving, (19, 0, 0))),
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (7, 45, 0))));
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC).is_open(open - Duration::seconds(1)),
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open));
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC).is_open(close - Duration::seconds(1)),
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(close));
        assert_declared_refusal(calendar.session_bounds(open));
        assert_declared_refusal(calendar.trade_date(close - Duration::seconds(1)));
        // The block was followed by the ordinary Sunday 19:00 CT open into
        // Monday's trade date, not by a Friday-evening leg; below the floor the
        // query states its refusal and the Sunday instant is not claimable.
        assert_declared_refusal(calendar.next_session_open_after(close + Duration::minutes(1)));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (19, 0, 0))));
    }
}

/// The counter-example the late-open ruling rests on: Christmas 2022 fell on a
/// Sunday, so the observed closure on Monday 2022-12-26 carried the operator's
/// ordinary 19:00 CT evening leg for trade date 2022-12-27, and 2022-12-27
/// therefore ships no late open and opens normally.
///
/// The row set is the claim that survives: the closure on 12-26 and the absent
/// row on 12-27. The leg that carried 12-27, and the ordinary day it opened, are
/// pre-floor answers and are refused.
#[test]
fn era_2022_12_27_opens_normally_because_the_holiday_carried_the_evening_leg() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.holiday_on(day((2022, 12, 26))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(
        calendar.holiday_on(day((2022, 12, 27))),
        None,
        "the evening leg on the holiday means this trade date opens normally"
    );

    // The holiday's own 19:00 CT leg, which carried 2022-12-27.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2022, 12, 26), (18, 59, 59))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2022, 12, 26), (19, 0, 0))),
    );
    assert_declared_refusal(calendar.trade_date(ct((2022, 12, 26), (19, 0, 0))));

    // 2022-12-27 was an ordinary trading day: the overnight leg ran to its
    // 07:45 CT pause, the 08:30-13:20 CT day session ran in full, and 13:20
    // ended the trade date. Every one of those probes is pre-floor.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2022, 12, 27), (7, 44, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2022, 12, 27), (8, 0, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2022, 12, 27), (8, 30, 0))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2022, 12, 27), (13, 19, 59))),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2022, 12, 27), (13, 20, 0))),
    );
    assert_declared_refusal(calendar.session_bounds(ct((2022, 12, 27), (9, 0, 0))));
    assert_declared_refusal(calendar.next_session_open_after(ct((2022, 12, 26), (10, 0, 0))));

    // The contrast: Christmas 2024's closure carried no evening leg, so its
    // successor is one of the six late opens.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2024, 12, 25), (19, 0, 0))),
    );
    assert_eq!(
        calendar.holiday_on(day((2024, 12, 26))).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN
        })
    );
}

/// A closure removes the trade date and the leg that opened it the previous
/// evening, and the family's next session after it is its ordinary 19:00 CT
/// evening open — except where the successor itself ships a late open, which
/// removes that evening leg too and reopens at 08:30 CT.
///
/// Stage 2B refuses every date this walk covers, so the schedule shape the
/// paragraph above describes is no longer claimable: each probe states the
/// refusal, and `ERA_ROWS` above remains the fence for which dates ship a
/// closure and what the era's total is.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = calendar_for_market_hours_key(ZC);
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(*date);
        assert_declared_refusal(calendar.is_closed_trade_date(date, SessionKind::Both));
        // The evening leg that would have carried this trade date, and the
        // trade date's own session: overnight, day and close.
        assert_declared_refusal(calendar.is_open(ct_at(day_before(date), ERA_EVENING_OPEN)));
        assert_declared_refusal(calendar.is_open(ct_on(day_before(date), (19, 30, 0))));
        assert_declared_refusal(calendar.is_open(ct_on(date, (2, 0, 0))));
        assert_declared_refusal(calendar.is_open(ct_on(date, (9, 0, 0))));
        assert_declared_refusal(calendar.is_open(ct_on(date, (13, 19, 0))));
        assert_declared_refusal(calendar.trade_date(ct_on(date, (10, 0, 0))));
        assert_declared_refusal(calendar.next_session_open_after(ct_on(date, (10, 0, 0))));
        // Whether the successor's own late open left the civil day wholly
        // closed was a claim about the era's grid, and the floor withholds it
        // with the rest.
        assert_declared_refusal(calendar.is_closed_all_day_on(date, SessionKind::Both));
    }
    assert_eq!(closures, 26, "the era's closures");
}

/// Every query about an `Unsourced` date is refused, and refused identically by
/// the detached calendar: the row states that the date was audited and makes no
/// scheduling claim, but below the 2025-01-01 floor neither calendar has an
/// answer to compare, so the "the row clips nothing" equality this helper used
/// to fence is no longer observable. What remains is the row's own claim —
/// kind and tier — plus the refusal at every probe the old comparison used.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = calendar_for_market_hours_key(ZC);
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    assert_declared_refusal(calendar.is_closed_trade_date(date, SessionKind::Both));
    assert_declared_refusal(detached.is_closed_trade_date(date, SessionKind::Both));

    for probe in [
        ct_on(day_before(date), (19, 30, 0)),
        ct_on(date, (9, 0, 0)),
        ct_on(date, (13, 19, 0)),
        ct_on(date, (19, 30, 0)),
    ] {
        assert_declared_refusal(calendar.is_open(probe));
        assert_declared_refusal(detached.is_open(probe));
        assert_declared_refusal(calendar.trade_date(probe));
        assert_declared_refusal(detached.trade_date(probe));
        assert_declared_refusal(calendar.session_bounds(probe));
        assert_declared_refusal(detached.session_bounds(probe));
        assert_declared_refusal(calendar.next_session_open_after(probe));
        assert_declared_refusal(detached.next_session_open_after(probe));
        assert_declared_refusal(calendar.candle_end(probe, CalendarResolution::Daily));
        assert_declared_refusal(detached.candle_end(probe, CalendarResolution::Daily));
    }
}

#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = calendar_for_market_hours_key(ZC);
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_unsourced_changes_nothing(date, row, EvidenceTier::T2);
    }
}

/// The 2022-2024 window sits fifth in the declared coverage, and the 2013-2015
/// interval below the 2016-2018 wave is a window of its own since this wave
/// shipped. (The 2019-2021 interval this test used to fence became a window of
/// its own when that wave shipped; the section below fences it.)
///
/// The window list and the shipped rows at each date are the claims that
/// survive; the sessions those rows shut are below the floor and are refused.
#[test]
fn era_2022_2024_window_sits_fifth_and_the_2013_2015_era_is_audited() {
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
            (day((2013, 1, 1)), day((2015, 12, 31))),
            (day((2016, 1, 1)), day((2018, 12, 31))),
            (day((2019, 1, 1)), day((2021, 12, 31))),
            (day((2022, 1, 1)), day((2024, 12, 31))),
            (day((2025, 1, 1)), day((2027, 12, 31))),
        ]
    );
    assert!(coverage.contains(day((2022, 1, 1))));
    assert!(coverage.contains(day((2024, 12, 31))));
    assert!(coverage.contains(day((2021, 12, 31))));
    assert_eq!(calendar.holiday_on(day((2022, 1, 1))), None);
    assert_eq!(calendar.holiday_on(day((2024, 12, 31))), None);

    // The 2013-2015 wave shipped after this test was written: 2015-12-24 is
    // its 12:05 CT early close and 2015-12-25 its Christmas closure.
    assert_eq!(
        calendar.holiday_on(day((2015, 12, 24))).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 5 * 60
        })
    );
    assert_eq!(
        calendar.holiday_on(day((2015, 12, 25))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 24), (13, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 25), (10, 0, 0))));
    // Christmas 2015 is the 2013-2015 wave's own closure, and 2015-12-24 the
    // 12:05 CT early close it shipped with it.
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 25), (9, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2015, 12, 24), (13, 0, 0))));
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. Each
/// branch's own fence lives in the `assert_era_*` helper it calls, and every
/// date the walk visits precedes the 2025-01-01 floor, so those helpers state
/// the refusal at each boundary the era's rows used to describe.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = calendar_for_market_hours_key(ZC);
    let (mut late_opens, mut combined, mut early_closes) = (0_usize, 0_usize, 0_usize);
    let (mut closures, mut unsourced) = (0_usize, 0_usize);
    let mut rows = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
        if let Some(row) = calendar.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::EarlyClose { close_ssm } => {
                    early_closes += 1;
                    assert_eq!(close_ssm, ERA_HALF_DAY_FIVE_PAST, "{date}");
                    assert_era_early_close(calendar, date);
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late_opens += 1;
                    assert_eq!(open_ssm, ERA_DAY_OPEN, "{date}");
                    assert_era_late_open(calendar, date);
                }
                HolidayKind::LateOpenAndEarlyClose {
                    open_ssm,
                    close_ssm,
                } => {
                    combined += 1;
                    assert_eq!(
                        (open_ssm, close_ssm),
                        (ERA_DAY_OPEN, ERA_HALF_DAY_FIVE_PAST),
                        "{date}"
                    );
                    assert_era_late_open_and_early_close(calendar, date);
                }
                HolidayKind::Closed => {
                    closures += 1;
                    assert_declared_refusal(calendar.is_closed_trade_date(date, SessionKind::Both));
                    assert_era_closure(calendar, date);
                }
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: this era ships no {other:?}"),
            }
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 42, "the era's rows");
    assert_eq!(
        (late_opens, combined, early_closes, closures, unsourced),
        (5, 3, 4, 27, 3),
        "the era's shape"
    );
}

/// An early close ended the day session at the printed instant and clipped the
/// evening leg that opened the trade date rather than deleting it.
///
/// Both are pre-floor answers, so every probe below states the refusal in place
/// of the boundary it used to fence.
fn assert_era_early_close(calendar: ExchangeCalendar, date: NaiveDate) {
    let cutoff = ct_at(date, ERA_HALF_DAY_FIVE_PAST);
    // The evening leg that opened this trade date, and the trade date it
    // carried.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
    );
    assert_declared_refusal(calendar.trade_date(ct_on(day_before(date), (19, 30, 0))));
    // Both sides of the printed close, the day session's bounds, the daily
    // candle, the trade date, and the ordinary 13:20 CT close the row does not
    // reach. The `inside` probe sits just within the session so the printed
    // close cannot make the query answer `None` instead.
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(cutoff - Duration::seconds(1)),
    );
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(cutoff));
    let inside = cutoff - Duration::minutes(1);
    assert_declared_refusal(calendar.session_bounds(inside));
    assert_declared_refusal(calendar.candle_end(inside, CalendarResolution::Daily));
    assert_declared_refusal(calendar.trade_date(cutoff - Duration::seconds(1)));
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct_at(date, ERA_DAY_CLOSE)));
}

/// A late open deleted the prior-evening leg the closure before it printed no
/// reopen for, and the day session that followed opened at 08:30 CT on its own
/// civil date and still ended at the ordinary 13:20 CT close.
///
/// Both are pre-floor answers, so every probe below states the refusal.
fn assert_era_late_open(calendar: ExchangeCalendar, date: NaiveDate) {
    let open = ct_on(date, (8, 30, 0));
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct_on(day_before(date), (19, 30, 0))),
    );
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open - Duration::seconds(1)));
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open));
    assert_declared_refusal(calendar.session_bounds(open));
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC)
            .is_open(ct_at(date, ERA_DAY_CLOSE) - Duration::seconds(1)),
    );
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct_at(date, ERA_DAY_CLOSE)));
    assert_declared_refusal(calendar.trade_date(open));
}

/// The day-after-Thanksgiving shape: neither boundary was reachable from the
/// ordinary grid — no prior-evening leg, no 07:45 CT pause — and the single
/// block ran 08:30 to 12:05 CT.
///
/// Both are pre-floor answers, so every probe below states the refusal.
fn assert_era_late_open_and_early_close(calendar: ExchangeCalendar, date: NaiveDate) {
    let open = ct_on(date, (8, 30, 0));
    let close = ct_on(date, (12, 5, 0));
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
    );
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct_on(date, (7, 45, 0))));
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open - Duration::seconds(1)));
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open));
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(close - Duration::seconds(1)),
    );
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(close));
    assert_declared_refusal(calendar.session_bounds(open));
    assert_declared_refusal(calendar.trade_date(open));
}

/// A closure took the whole trade date — the evening leg that would have
/// carried it, the overnight session, the day session and the close.
///
/// Every date the caller passes is pre-floor, so each probe states the refusal.
fn assert_era_closure(calendar: ExchangeCalendar, date: NaiveDate) {
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
    );
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct_on(day_before(date), (19, 30, 0))),
    );
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct_on(date, (2, 0, 0))));
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct_on(date, (9, 0, 0))));
    assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct_on(date, (13, 19, 0))));
    assert_declared_refusal(calendar.trade_date(ct_on(date, (10, 0, 0))));
}

/// The day-after-closure shapes this wave derives, date for date: five 08:30 CT
/// late opens and three 08:30-12:05 CT blocks on the Thanksgiving Fridays, each
/// paired with the closure the operator printed beside it and neither reachable
/// from the ordinary 19:00-13:20 CT grid.
///
/// The derived dates and their pairings are the claim that survives; the
/// boundaries those rows moved are pre-floor and are refused.
#[test]
fn era_2019_2021_day_after_closure_shapes_are_the_derived_dates() {
    let calendar = calendar_for_market_hours_key(ZC);

    for (date, closure) in [
        ((2019, 1, 2), (2019, 1, 1)),
        ((2019, 7, 5), (2019, 7, 4)),
        ((2019, 12, 26), (2019, 12, 25)),
        ((2020, 1, 2), (2020, 1, 1)),
        ((2021, 7, 6), (2021, 7, 5)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpen {
                open_ssm: ERA_DAY_OPEN
            }),
            "{date:?}"
        );
        assert_eq!(
            calendar.holiday_on(day(closure)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{closure:?}"
        );
        // The successor's day session opened at 08:30 CT on its own civil
        // date, with no evening leg surviving the closure: both are pre-floor
        // answers, so the probes state the refusal.
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(closure, (19, 0, 0))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (8, 29, 59))));
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (8, 30, 0))));
    }

    for (date, thanksgiving) in [
        ((2019, 11, 29), (2019, 11, 28)),
        ((2020, 11, 27), (2020, 11, 26)),
        ((2021, 11, 26), (2021, 11, 25)),
    ] {
        let open = ct(date, (8, 30, 0));
        let close = ct(date, (12, 5, 0));
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: ERA_DAY_OPEN,
                close_ssm: ERA_HALF_DAY_FIVE_PAST,
            }),
            "{date:?}"
        );
        assert_eq!(
            calendar.holiday_on(day(thanksgiving)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{thanksgiving:?}"
        );
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC).is_open(ct(thanksgiving, (19, 0, 0))),
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(ct(date, (7, 45, 0))));
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC).is_open(open - Duration::seconds(1)),
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(open));
        assert_declared_refusal(
            calendar_for_market_hours_key(ZC).is_open(close - Duration::seconds(1)),
        );
        assert_declared_refusal(calendar_for_market_hours_key(ZC).is_open(close));
        assert_declared_refusal(calendar.session_bounds(open));
        assert_declared_refusal(calendar.trade_date(close - Duration::seconds(1)));
    }
}

/// Every `Unsourced` row the era ships changes no answer — or would, if the
/// date were answerable: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing, but no query about it has an answer to
/// compare (see `assert_unsourced_changes_nothing`).
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = calendar_for_market_hours_key(ZC);
    let mut checked = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
        if let Some(row) = calendar.holiday_on(date)
            && row.kind() == HolidayKind::Unsourced
        {
            assert_unsourced_changes_nothing(date, row, EvidenceTier::T1);
            checked += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(checked, 3, "the era's `Unsourced` rows");
}

/// The era is its own declared window: 2019-01-01 and 2021-12-31 are inside
/// it, while 2018-12-31 and 2022-01-01 belong to the waves either side and lie
/// outside it.
///
/// Both edges are below the floor, so their own answers are not claimable; the
/// shipped rows at each edge are, and the last day's probe states the refusal.
#[test]
fn era_2019_2021_window_edges_answer_as_the_module_declares() {
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");
    let era = (day((2019, 1, 1)), day((2021, 12, 31)));

    assert!(
        coverage.windows().contains(&era),
        "the 2019-2021 window is declared as a window of its own"
    );
    let inside = |date: NaiveDate| era.0 <= date && date <= era.1;
    assert!(inside(day((2019, 1, 1))));
    assert!(inside(day((2021, 12, 31))));
    assert!(
        !inside(day((2018, 12, 31))),
        "2018-12-31 belongs to the 2016-2018 wave, not to this era"
    );
    assert!(
        !inside(day((2022, 1, 1))),
        "2022-01-01 belongs to the 2022-2024 wave, not to this era"
    );

    // The era's own edges carry their rows: its first day is the shipped New
    // Year closure, and its last is an ordinary Friday this table audited.
    assert_eq!(
        calendar.holiday_on(day((2019, 1, 1))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(calendar.holiday_on(day((2021, 12, 31))), None);
    assert_declared_refusal(
        calendar_for_market_hours_key(ZC).is_open(ct((2021, 12, 31), (9, 0, 0))),
    );
    // The neighbouring dates, which other waves audit, carry no row here.
    assert_eq!(calendar.holiday_on(day((2018, 12, 31))), None);
    assert_eq!(calendar.holiday_on(day((2022, 1, 1))), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
            (day((2013, 1, 1)), day((2015, 12, 31))),
            (day((2016, 1, 1)), day((2018, 12, 31))),
            (day((2019, 1, 1)), day((2021, 12, 31))),
            (day((2022, 1, 1)), day((2024, 12, 31))),
            (day((2025, 1, 1)), day((2027, 12, 31))),
        ]
    );

    // Every shipped row of the era is inside the era's own window: the walk
    // reads the module, and the declared window is what must contain it.
    let mut rows = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
        if calendar.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 42, "the era's rows");
    for probe in [day((2018, 12, 31)), day((2022, 1, 1))] {
        assert_eq!(calendar.holiday_on(probe), None, "{probe}");
    }
}

/// The 2019-2021 rows as the block records them: date, kind and tier in order,
/// handwritten here rather than read back from the module. The era-wide sweep
/// counts kinds and instants, which a row moved to another audited date with
/// the same kind and instant would leave unchanged; this pins the date set
/// itself, in the order `holiday_on` must answer it.
const ERA_2019_2021_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2019, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2019, 1, 21), HolidayKind::Closed, EvidenceTier::T1),
    ((2019, 2, 18), HolidayKind::Closed, EvidenceTier::T1),
    ((2019, 4, 19), HolidayKind::Closed, EvidenceTier::T1),
    ((2019, 5, 27), HolidayKind::Closed, EvidenceTier::T1),
    ((2019, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2019, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2019, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2019, 9, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2019, 11, 28), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 11, 29),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2019, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2020, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2020, 1, 20), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 2, 17), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 4, 10), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 5, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2020, 7, 2),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2020, 7, 3), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 9, 7), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 11, 26), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 11, 27),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2020, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 1, 18), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 2, 15), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 4, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 5, 31), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    ((2021, 7, 5), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2021, 7, 6),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2021, 9, 6), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 11, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2021, 11, 26),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2021, 12, 24), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2019_2021_rows_are_the_audited_date_kind_and_tier_set() {
    let calendar = calendar_for_market_hours_key(ZC);
    let mut index = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, tier) = *ERA_2019_2021_ROWS.get(index).unwrap_or_else(|| {
                panic!("{date}: a row ships in the 2019-2021 window that the block does not record")
            });
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the 2019-2021 rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2019_2021_ROWS.len(), "every recorded row ships");
}

// ---------------------------------------------------------------------------
// The 2013-2015 rows.
// ---------------------------------------------------------------------------

/// The era-wide sweep: every row the 2013-2015 window ships, read from the
/// module, with both sides of every instant it states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the side of the instant it moved from,
/// and a kind this family does not ship fails outright. The count tuple is the
/// era's shape as the block records it; the handwritten table below pins the
/// date set the counts cannot see. Every instant the sweep probes belongs to a
/// pre-2025 date, so those probes state the refusal.
#[test]
fn era_2013_2015_sweeps_every_shipped_row_kind_and_instant() {
    let venue = calendar_for_market_hours_key(ZC);
    let (mut closed, mut early, mut late, mut both) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = day((2013, 1, 1));
    while date <= day((2015, 12, 31)) {
        if let Some(row) = venue.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::Closed => {
                    closed += 1;
                    assert_declared_refusal(venue.is_closed_trade_date(date, SessionKind::Both));
                    assert_declared_refusal(
                        venue.is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
                    );
                }
                HolidayKind::EarlyClose { close_ssm } => {
                    early += 1;
                    let (h, m, s) = (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let cutoff = ct_on(date, (h, m, s));
                    assert_declared_refusal(venue.is_open(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(cutoff));
                    assert_declared_refusal(venue.trade_date(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(
                        venue.candle_end(cutoff - Duration::minutes(1), CalendarResolution::Daily),
                    );
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late += 1;
                    let (h, m, s) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let open = ct_on(date, (h, m, s));
                    // The leg that would have carried this trade date opened at
                    // 19:00 CT the evening before, which is the hour the row
                    // moved; both sides of the printed open, and the trade date
                    // the day session carried.
                    assert_declared_refusal(
                        venue.is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
                    );
                    assert_declared_refusal(venue.is_open(open - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(open));
                    assert_declared_refusal(venue.trade_date(open + Duration::hours(1)));
                }
                HolidayKind::LateOpenAndEarlyClose {
                    open_ssm,
                    close_ssm,
                } => {
                    both += 1;
                    let (oh, om, os) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let (ch, cm, cs) =
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let open = ct_on(date, (oh, om, os));
                    let cutoff = ct_on(date, (ch, cm, cs));
                    assert_declared_refusal(
                        venue.is_open(ct_at(day_before(date), ERA_EVENING_OPEN)),
                    );
                    assert_declared_refusal(venue.is_open(open - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(open));
                    assert_declared_refusal(venue.is_open(cutoff - Duration::seconds(1)));
                    assert_declared_refusal(venue.is_open(cutoff));
                }
                other => panic!("{date}: this era ships no {other:?}"),
            }
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        (closed, early, late, both),
        (27, 6, 7, 3),
        "the era's shape"
    );
}

/// The era is its own declared window: 2013-01-01 is inside it and 2012-12-31
/// and 2016-01-01 belong to the waves either side and lie outside it.
#[test]
fn era_2013_2015_window_edges_answer_as_the_module_declares() {
    let venue = calendar_for_market_hours_key(ZC);
    let coverage = venue
        .holiday_coverage()
        .expect("globex_grains ships a table");
    let era = (day((2013, 1, 1)), day((2015, 12, 31)));

    assert!(
        coverage.windows().contains(&era),
        "the 2013-2015 window is declared as a window of its own"
    );
    assert!(coverage.contains(era.0));
    assert!(coverage.contains(era.1));

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is the last date the window declares.
    assert_eq!(
        venue.holiday_on(day((2013, 1, 1))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        coverage.contains(day((2015, 12, 31))),
        "the era's last day is inside the declared window"
    );
    // The neighbouring dates are their own eras' business: 2012-12-31 is
    // audited normal by the wave below, 2016-01-01 opens the next declared
    // window, and neither is this era.
    assert_eq!(venue.holiday_on(day((2012, 12, 31))), None);
    assert!(
        coverage.contains(day((2016, 1, 1))),
        "2016-01-01 is the next declared window's first day"
    );
    // A date below the January-2010 floor is outside every window, so this
    // table has no answer for it at all.
    assert!(!coverage.contains(day((2009, 12, 31))));
    assert_eq!(venue.holiday_on(day((2009, 12, 31))), None);
}

/// The family's coverage names its windows in order, and the 2013-2015 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2013_2015_window_is_declared_in_order_and_bounds_every_row() {
    let venue = calendar_for_market_hours_key(ZC);
    let coverage = venue
        .holiday_coverage()
        .expect("globex_grains ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
            (day((2013, 1, 1)), day((2015, 12, 31))),
            (day((2016, 1, 1)), day((2018, 12, 31))),
            (day((2019, 1, 1)), day((2021, 12, 31))),
            (day((2022, 1, 1)), day((2024, 12, 31))),
            (day((2025, 1, 1)), day((2027, 12, 31))),
        ]
    );

    let mut rows = 0_usize;
    let mut date = day((2013, 1, 1));
    while date <= day((2015, 12, 31)) {
        if venue.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 43, "the era's rows");
    assert_eq!(venue.holiday_on(day((2012, 12, 31))), None);
    assert!(!coverage.contains(day((2028, 1, 1))));
    assert_eq!(venue.holiday_on(day((2028, 1, 1))), None);
}

/// The 2013-2015 rows as the block records them: date, kind and tier in order,
/// handwritten here rather than read back from the module. The era-wide sweep
/// counts kinds and instants, which a row moved to another audited date with
/// the same kind and instant would leave unchanged; this pins the date set
/// itself, in the order `holiday_on` must answer it.
const ERA_2013_2015_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2013, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 9 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2013, 1, 21), HolidayKind::Closed, EvidenceTier::T1),
    ((2013, 2, 18), HolidayKind::Closed, EvidenceTier::T1),
    ((2013, 3, 29), HolidayKind::Closed, EvidenceTier::T1),
    ((2013, 5, 27), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2013, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2013, 9, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2013, 11, 28), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 11, 29),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2013, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 1, 20), HolidayKind::Closed, EvidenceTier::T1),
    ((2014, 2, 17), HolidayKind::Closed, EvidenceTier::T1),
    ((2014, 4, 18), HolidayKind::Closed, EvidenceTier::T1),
    ((2014, 5, 26), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2014, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 7, 7),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 9, 1), HolidayKind::Closed, EvidenceTier::T1),
    ((2014, 11, 27), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 11, 28),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2014, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2015, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2015, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2015, 1, 19), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 2, 16), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 4, 3), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 5, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2015, 7, 2),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2015, 7, 3), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 9, 7), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 11, 26), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2015, 11, 27),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 5 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2015, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2013_2015_rows_are_the_audited_date_kind_and_tier_set() {
    let venue = calendar_for_market_hours_key(ZC);
    let mut index = 0_usize;
    let mut date = day((2013, 1, 1));
    while date <= day((2015, 12, 31)) {
        if let Some(row) = venue.holiday_on(date) {
            let (expected, kind, tier) = *ERA_2013_2015_ROWS.get(index).unwrap_or_else(|| {
                panic!("{date}: a row ships in the 2013-2015 window that the block does not record")
            });
            assert_eq!(
                (date.year(), date.month(), date.day()),
                expected,
                "the 2013-2015 rows must ship in order, with none added"
            );
            assert_eq!(row.kind(), kind, "{date}");
            assert_eq!(row.tier(), tier, "{date}");
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2013_2015_ROWS.len(), "every recorded row ships");
}
