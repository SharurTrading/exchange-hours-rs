// SPDX-License-Identifier: MIT-0

//! `globex_equity_index`'s built-in holiday rows, over the public surface.
//!
//! Every probe below is stated in America/Chicago wall clock — the zone CME
//! prints its trading hours in — and converted to UTC by `ct`, so a reader can
//! compare a line here against the operator's published instant without doing
//! the arithmetic. The family's normal trading day for venue-local trade date
//! `D` opens 17:00 CT on the preceding business evening, runs the 08:30-15:15
//! CT regular session and the 15:15-16:00 CT extended leg, and ends at its
//! 16:00 CT final close on `D`; the rows move that close or delete the day.
//!
//! The cases are the design memo's §4.1 seven: a closed day, both sides of an
//! early close, the late-open branch (absent in this window, and fenced as
//! absent), the wrap a closure removes, the trade-date consequence, both edges
//! of the coverage window, and `without_holidays` restoring the normal answer.
//!
//! The file holds two kinds of claim, and the coverage contract (LAW-COVERAGE)
//! separates them. Every case stated **at or above** the permanent 2025-01-01
//! support floor fences the schedule itself: the session bounds, the trade
//! date, the end-exclusive close and the candle. The eras below the floor left
//! with Stage 5 of the release plan (#117) — the rows they shipped and the
//! windows they were audited over are gone — so a pre-floor date has neither a
//! row nor an audited normal week, and the one case that probes below the floor
//! states the refusal itself, on the same dates and through the same entry
//! points the schedule probes used. This family declares the Sunday
//! 16:00-16:15 CT order-entry queue withheld (#79), which is why a probe that
//! falls outside every session can be refused as `OutsideCoveredRange` rather
//! than as the floor: see `assert_declared_refusal`.

use chrono::{DateTime, Datelike as _, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, CalendarSource, DateCoverage, EvidenceTier,
    ExceptionBlock, ExceptionBlockKind, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, SessionState, calendar_for_exchange, calendar_for_market_hours_key,
    hours_for_market_hours_key,
};

/// The family calendar under test, with its built-in table attached.
fn equity_index() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex)
}

/// A venue-local America/Chicago wall clock, converted to the UTC instant the
/// public surface takes.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
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
/// copied beside the probe: `BeforeSupportFloor` below the floor,
/// `OutsideCoveredRange` on a date the family has no sourced answer for — every
/// date before the knowledge-bound 2026-08-22 era is one, because this family
/// declares the Sunday 16:00-16:15 CT order-entry queue withheld (#79) — and
/// `UnresolvedGap` on a date the identity withholds.
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
    // One documented exception: `trade_date` resolves a containing session
    // before it consults the floor, so an instant that lies in no session falls
    // through to the order-entry scan, and that scan reports the declared
    // phase-level gap ahead of the floor. The exception reaches exactly as far
    // as a pre-floor date on an identity that declares such a gap.
    let declared_phase_gap = matches!(
        (verdict, error),
        (
            DateCoverage::BeforeSupportFloor,
            CalendarQueryError::OutsideCoveredRange { .. }
        )
    ) && !coverage.phase_gaps().is_empty();
    assert!(
        states_verdict || declared_phase_gap,
        "{day} carries the verdict {verdict:?}: {error:?} does not state it"
    );
}

/// 12:00 CT, the Monday/Thursday-holiday final close.
const NOON: u32 = 12 * 3_600;
/// 12:15 CT, the Christmas-Eve and day-after-Thanksgiving final close.
const QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;

// ---------------------------------------------------------------------------
// 1. A closed day.
// ---------------------------------------------------------------------------

/// Christmas 2025 falls on a Thursday and CME publishes no equity-index
/// session for it: the trade date is gone, so nothing the crate would have
/// assigned to it survives — including the Wednesday-evening leg.
#[test]
fn christmas_2025_is_a_closed_trade_date_with_no_session_of_its_own() {
    let calendar = equity_index();

    let holiday = calendar
        .holiday_on(day(2025, 12, 25))
        .expect("2025-12-25 ships a row");
    assert_eq!(holiday.kind(), HolidayKind::Closed);
    assert_eq!(holiday.tier(), EvidenceTier::T2);
    assert_eq!(holiday.document_id(), "CME-SVC-2025-12-24");

    assert!(
        calendar
            .is_closed_trade_date(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );

    // Three probes inside the civil day, before the evening leg that belongs
    // to the *next* trade date opens at 17:00 CT.
    for time in [(3, 0, 0), (10, 0, 0), (16, 0, 0)] {
        assert!(
            !calendar
                .is_open(ct((2025, 12, 25), time))
                .expect("the coverage contract must answer a covered date"),
            "2025-12-25 {time:?} CT must be closed"
        );
    }

    // The civil day is not wholly closed: 17:00 CT opens trade date 12-26.
    assert!(
        !calendar
            .is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// 2 and 3. An early close, on each side of the cutoff.
// ---------------------------------------------------------------------------

/// The day after Thanksgiving 2025 closes 12:15 CT. The trading day opened
/// 17:00 CT on Thursday, so the clip has to land on the Friday and take the
/// 15:15-16:00 CT leg — which opens after the cutoff — with it.
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "midnight is written in the table fence's own `h * 3_600 + m * 60` grammar, \
              because the fence rejects a bare zero"
)]
#[test]
fn the_day_after_thanksgiving_2025_closes_at_1215_central() {
    // Thanksgiving Day publishes no final close of its own, so 2025-11-28 now
    // owns the span from Wednesday evening and its own regular session is cut at
    // the same 12:15 the earlier `EarlyClose` row stated.
    static EXPECTED: [ExceptionBlock; 9] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 8 * 3_600 + 30 * 60),
        ExceptionBlock::regular(-1, 8 * 3_600 + 30 * 60, 12 * 3_600),
        ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
        ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
        ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
        ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 8 * 3_600 + 30 * 60),
        ExceptionBlock::regular(0, 8 * 3_600 + 30 * 60, QUARTER_PAST_NOON),
    ];
    let calendar = equity_index();
    let holiday = calendar
        .holiday_on(day(2025, 11, 28))
        .expect("2025-11-28 ships a row");
    assert_eq!(holiday.kind(), HolidayKind::ReplacementBlocks(&EXPECTED));
    assert_eq!(holiday.document_id(), "CME-SVC-2025-11-26");

    // One second before the cutoff, and at it: closes are end-exclusive.
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The rest of the regular session is gone, and so is the extended leg
    // that would have opened at 15:15 CT.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (14, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    assert_eq!(
        calendar
            .session_bounds(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 11, 28), (8, 30, 0)),
            ct((2025, 11, 28), (12, 15, 0))
        ))
    );
    assert_eq!(
        calendar
            .candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 11, 28), (12, 15, 0)))
    );
}

/// Martin Luther King Jr. Day 2025 is the memo's §1.1 worked example: CME
/// prints `12:00 preopen`, matching stops, and the trading day that opened
/// 17:00 CT on Sunday ends there instead of at 16:00 CT.
#[test]
fn martin_luther_king_day_2025_clips_the_sunday_evening_trading_day() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2025, 1, 20))
            .expect("2025-01-20 ships a row")
            .kind(),
        HolidayKind::EarlyClose { close_ssm: NOON }
    );

    // The Sunday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert!(
        calendar
            .is_open(ct((2025, 1, 19), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 1, 20), (11, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 1, 20), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 1, 20), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // 17:00 CT that evening opens trade date 2025-01-21, which is normal.
    assert!(
        calendar
            .is_open(ct((2025, 1, 20), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 1, 20), (12, 5, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 1, 20), (17, 0, 0)))
    );
}

/// Good Friday 2026 is the one date CME itself flags as an exception, for the
/// U.S. employment release: equity index closes 08:15 CT, before its own
/// regular session would have opened, so the whole day session disappears and
/// only the Thursday-evening leg survives.
#[test]
fn good_friday_2026_closes_equity_index_at_0815_central() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2026, 4, 3))
            .expect("2026-04-03 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 15 * 60
        }
    );

    assert!(
        calendar
            .is_open(ct((2026, 4, 3), (8, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 3), (8, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 3), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2026, 4, 3), (2, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 4, 2), (17, 0, 0)), ct((2026, 4, 3), (8, 15, 0))))
    );
}

// ---------------------------------------------------------------------------
// 4. The late-open branch.
// ---------------------------------------------------------------------------

/// No `globex_equity_index` row in this coverage window moves a first open.
///
/// CME reopens every one of these holidays at the family's ordinary 17:00 CT,
/// and the one sourced *later* first open the corpus carries for this family —
/// the 05:00 CT Saturday sessions of 2026-06-20, 2026-07-04 and 2027-06-19 —
/// would have to **create** a session the normal week does not have, which the
/// scalar vocabulary cannot state and which ships as a declared gap instead
/// (design memo D7). This fence keeps that a recorded fact rather than an
/// accident: a late-open row appearing here without its evidence fails.
#[test]
fn no_late_open_row_ships_in_the_2025_2027_window() {
    let calendar = equity_index();
    let mut date = day(2025, 1, 1);
    let last = day(2027, 12, 31);

    while date <= last {
        if let Some(holiday) = calendar.holiday_on(date) {
            // `ReplacementBlocks` joined the vocabulary in Stage 4: it states a
            // complete trading day for a date a scalar row cannot describe, and
            // it is not a late open. The fence this test draws is unchanged.
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed
                        | HolidayKind::EarlyClose { .. }
                        | HolidayKind::ReplacementBlocks(_)
                ),
                "{date}: this window ships closures, early closes and complete-day \
                 replacements, not {:?}",
                holiday.kind()
            );
        }
        date = date
            .succ_opt()
            .expect("the window ends well before the epoch bound");
    }

    // The Saturday sessions are rows now, on the trade date each carries: the
    // session itself is stated by the following Monday's replacement row, and
    // the Saturday holds no holiday row of its own.
    for (saturday, trade_date) in [
        ((2026, 6, 20), (2026, 6, 22)),
        ((2026, 7, 4), (2026, 7, 6)),
        ((2027, 6, 19), (2027, 6, 21)),
    ] {
        assert!(
            calendar
                .is_open(ct(saturday, (10, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{saturday:?}: the Saturday session must be open"
        );
        assert_eq!(
            calendar
                .trade_date(ct(saturday, (10, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some(day(trade_date.0, trade_date.1, trade_date.2)),
            "{saturday:?}: the session carries the following Monday"
        );
        assert_eq!(
            calendar.holiday_on(day(saturday.0, saturday.1, saturday.2)),
            None,
            "{saturday:?}: the session is stated on its trade date, not the Saturday"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. A wrap removed by a closure.
// ---------------------------------------------------------------------------

/// Christmas Eve 2025 closes 12:15 CT and does not reopen, because the
/// Wednesday 17:00 CT leg would have carried trade date 2025-12-25 and that
/// date is closed. The next open is the Thursday-evening leg, on the holiday
/// itself, for trade date 2025-12-26.
#[test]
fn the_christmas_2025_closure_removes_the_previous_evenings_wrap() {
    let calendar = equity_index();

    assert_eq!(
        calendar
            .holiday_on(day(2025, 12, 24))
            .expect("2025-12-24 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: QUARTER_PAST_NOON
        }
    );

    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The normal 17:00 CT open of Wednesday evening is gone with its trade date.
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 12, 24), (12, 20, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 12, 25), (17, 0, 0))),
        "the next open is the holiday evening's leg, not a same-day remainder"
    );
}

// ---------------------------------------------------------------------------
// 6. The trade-date consequence.
// ---------------------------------------------------------------------------

/// A shortened day keeps its own trade date; the leg that opens on a closed
/// date carries the date after it.
///
/// The two probes that land **outside** every session do not survive this
/// family's declared phase-level gap. `trade_date` resolves a containing
/// session before it consults the floor, so an instant in no session falls
/// through to the order-entry scan, and that scan is withheld for every date
/// before the knowledge-bound 2026-08-22 era (#79: the Sunday 16:00-16:15 CT
/// queue this family does not serve). Both therefore state the refusal their
/// own coverage declares rather than the `None` they used to answer. The three
/// probes inside a session are answered normally and are unchanged.
#[test]
fn holiday_rows_move_the_trade_date_only_where_the_operator_does() {
    let calendar = equity_index();

    // Inside the shortened day after Thanksgiving 2025.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    // The shortened Martin Luther King Jr. Day 2025 carries the *next* trade
    // date: CME publishes no final close for the holiday, so the operator labels
    // the whole span from Sunday evening with 2025-01-21.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 20), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 1, 21))
    );
    // The evening open on closed Christmas Day 2025 belongs to the Friday.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );
    // Nothing is assigned to the closed trade date itself, and the evening
    // before that closure has no session at all to assign: both are refused.
    assert_declared_refusal(calendar.trade_date(ct((2025, 12, 25), (10, 0, 0))));
    assert_declared_refusal(calendar.trade_date(ct((2025, 12, 24), (18, 0, 0))));
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// The window is a claim about every date inside it, so both its edges are
/// fenced: one day out, the table has no answer.
///
/// The schedule half of the old fence is gone with the floor and the audit
/// edge: the identity-backed calendar refuses both 2024-12-25 (below the
/// 2025-01-01 support floor) and 2028-01-01 (above the audited window), so the
/// "the crate and the pre-table answer agree" comparison is not available. The
/// detached calendar states the normal week on its own, as it always did.
#[test]
fn the_coverage_window_is_exactly_2025_through_2027() {
    let calendar = equity_index();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_equity_index ships a table");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2025, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));
    // The table audits the single floor-onward era, so `contains` is true on
    // every date from 2025-01-01 to 2027-12-31 and false outside it. A date
    // with no row inside the window answers `None` as an audited normal date
    // rather than as silence; below the floor it answers `None` as no answer
    // at all.
    assert!(coverage.contains(day(2026, 6, 10)));
    assert_eq!(calendar.holiday_on(day(2026, 6, 10)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);

    // Christmas Day 2024 is a CME Globex closure one day below the window — a
    // row the module shipped until Stage 5 removed the pre-floor eras — and
    // New Year's Day 2028 one day above it. The identity-backed calendar
    // refuses both; the detached calendar claims no complete calendar and so
    // still states the normal week it serves, where 2028-01-01 is a Saturday
    // the family never trades.
    let bare = calendar.without_holidays();
    assert_declared_refusal(calendar.is_open(ct((2024, 12, 25), (10, 0, 0))));
    assert_declared_refusal(calendar.is_open(ct((2028, 1, 1), (10, 0, 0))));
    assert!(
        !bare
            .is_open(ct((2028, 1, 1), (10, 0, 0)))
            .expect("a detached calendar states the normal week it serves")
    );
}

// ---------------------------------------------------------------------------
// 8. `without_holidays` restores the normal answer.
// ---------------------------------------------------------------------------

/// The escape hatch is exact: detaching the table reproduces the pre-table
/// answer over a dense grid across the Christmas 2025 week, and the two
/// calendars must actually disagree inside it, or the grid proves nothing.
#[test]
fn without_holidays_restores_the_normal_week_across_the_christmas_2025_week() {
    let calendar = equity_index();
    let bare = calendar.without_holidays();

    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2025, 12, 25)), None);

    // The normal week has a session at each of these instants; the table
    // removes or shortens all three.
    assert!(
        bare.is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2025, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2025, 11, 28), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (15, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // Over the whole week, at five-minute resolution, the detached calendar
    // reproduces the pre-table engine exactly — a detached `MarketHours`
    // snapshot carries no identity and therefore no table, so it is the
    // control — while the attached one differs on at least one probe.
    let mut instant = ct((2025, 12, 21), (0, 0, 0));
    let end = ct((2025, 12, 28), (0, 0, 0));
    let mut differed = false;
    while instant < end {
        let pre_table =
            hours_for_market_hours_key(MarketHoursKey::GlobexEquityIndex, instant).is_open(instant);
        assert_eq!(
            bare.is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            pre_table,
            "without_holidays diverged from the pre-table answer at {instant}"
        );
        differed |= calendar
            .is_open(instant)
            .expect("the coverage contract must answer a covered date")
            != pre_table;
        instant += Duration::minutes(5);
    }
    assert!(
        differed,
        "the table must change an answer inside the Christmas 2025 week"
    );
}

// ---------------------------------------------------------------------------
// The removed pre-floor rows (Stage 5, #117)
//
// The eras before the permanent 2025-01-01 support floor left in Stage 5 of the
// release plan; their rows and their audited windows are gone from the module.
// What the plan requires of the removal is asserted here against a
// representative of each removed era: no row, and the below-floor refusal —
// never an answer read from the removed history.
// ---------------------------------------------------------------------------

/// A pre-floor holiday request is refused, never answered from removed history.
///
/// Each date below is a row the module shipped before Stage 5 removed it — the
/// 2010-04-02 Good Friday early close, the 2012-07-03 Independence-Day-eve
/// early close, the 2015-12-25 and 2024-12-25 closures, the 2019-06-19
/// `Unsourced` row and the 2018-12-26 late open. `holiday_on` answers `None`
/// for all of them (the table has no answer below the floor), the coverage no
/// longer contains any of them, and the date-aware query returns the explicit
/// `BeforeSupportFloor` error (LAW-COVERAGE), never a closure or a normal week
/// read from the removed rows.
#[test]
fn pre_floor_rows_refuse_instead_of_answering() {
    let calendar = equity_index();

    for (year, month, date) in [
        (2010, 4, 2),
        (2012, 7, 3),
        (2015, 12, 25),
        (2018, 12, 26),
        (2019, 6, 19),
        (2024, 12, 24),
        (2024, 12, 25),
    ] {
        let removed = day(year, month, date);
        assert!(
            !calendar
                .holiday_coverage()
                .expect("the family ships a coverage window")
                .contains(removed),
            "{removed} is below the floor and outside the retained window"
        );
        assert_eq!(
            calendar.holiday_on(removed),
            None,
            "{removed}: the removed row ships no holiday_on answer"
        );
        assert!(
            matches!(
                calendar.is_closed_trade_date(removed, SessionKind::Both),
                Err(CalendarQueryError::BeforeSupportFloor { .. })
            ),
            "{removed}: the query refuses the removed row's date below the floor"
        );
        assert_declared_refusal(calendar.is_open(ct((year, month, date), (10, 0, 0))));
    }

    // The evening leg that opens on 2024-12-31 belongs to trade date
    // 2025-01-01, whose own `Closed` row the module still ships — the retained
    // row carries the whole trading day including that wrap, so no pre-floor
    // row is load-bearing at or after the floor.
    assert_eq!(
        calendar.holiday_on(day(2024, 12, 31)),
        None,
        "2024-12-31 ships no row: its evening leg is trade date 2025-01-01's"
    );
    assert_eq!(
        calendar.holiday_on(day(2025, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "trade date 2025-01-01 keeps its own closure, wrap included"
    );
}

// ---------------------------------------------------------------------------
// The Saturday-session trade dates (Stage 4, #116)
//
// CME states a Saturday session on three trade dates in this window, each
// carrying the following Monday. Every expectation below is read from the
// service windows the rows cite: two of the three trade dates are printed by
// two windows — the Saturday session by one, the Sunday legs by the window that
// starts on that Sunday — and
// `a_saturday_rows_sunday_legs_are_sourced_from_a_window_that_prints_them` pins
// that the evidence rows name both. The family's envelope is the 2021-06-27
// removal of the 15:15-15:30 halt, so the overnight leg, the regular session
// and the post-regular slice are one continuous matching span; the row splits
// that span into ordered blocks at the regular boundaries so the regular phase
// survives the replacement, and the assertions below pin the phase as well as
// the span.
// ---------------------------------------------------------------------------

/// A venue-local calendar date stated as `(year, month, day)`.
type Ymd = (i32, u32, u32);

/// The three trade dates CME states a Saturday session for, with the Saturday
/// each session opens on and the Friday whose early close precedes it.
const SATURDAY_SESSION_DATES: [(Ymd, Ymd, Ymd); 3] = [
    ((2026, 6, 22), (2026, 6, 20), (2026, 6, 19)),
    ((2026, 7, 6), (2026, 7, 4), (2026, 7, 3)),
    ((2027, 6, 21), (2027, 6, 19), (2027, 6, 18)),
];

#[test]
fn the_equity_index_states_its_saturday_sessions_on_the_trade_date() {
    let calendar = equity_index();
    for (trade_date, saturday, friday) in SATURDAY_SESSION_DATES {
        assert_eq!(
            day(saturday.0, saturday.1, saturday.2).weekday(),
            Weekday::Sat
        );
        assert_eq!(
            day(trade_date.0, trade_date.1, trade_date.2).weekday(),
            Weekday::Mon
        );
        assert!(
            matches!(
                calendar
                    .holiday_on(day(trade_date.0, trade_date.1, trade_date.2))
                    .map(Holiday::kind),
                Some(HolidayKind::ReplacementBlocks(_))
            ),
            "{trade_date:?} must carry a replacement row"
        );
        assert_eq!(
            calendar
                .holiday_on(day(trade_date.0, trade_date.1, trade_date.2))
                .map(Holiday::document_id),
            Some(match trade_date {
                (2026, 6, 22) => "CME-SVC-2026-06-18",
                (2026, 7, 6) => "CME-SVC-2026-07-03",
                _ => "CME-SVC-2027-06-17",
            }),
            "{trade_date:?}: the row cites the window its instants were read from"
        );

        // The Saturday session: 05:00 open, 17:00 end-exclusive close, and the
        // following Monday as its trade date.
        let saturday_open = ct(saturday, (5, 0, 0));
        let saturday_close = ct(saturday, (17, 0, 0));
        assert!(
            calendar
                .is_open(saturday_open)
                .expect("2026 and 2027 are covered dates"),
            "{saturday:?}: the Saturday session is not open"
        );
        assert_eq!(
            calendar
                .session_bounds(ct(saturday, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((saturday_open, saturday_close)),
            "{saturday:?}: the Saturday session's own bounds"
        );
        assert!(
            !calendar
                .is_open(saturday_close)
                .expect("2026 and 2027 are covered dates"),
            "{saturday:?}: the close is end-exclusive"
        );
        assert_eq!(
            calendar
                .trade_date(ct(saturday, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some(day(trade_date.0, trade_date.1, trade_date.2))
        );

        // The Friday before it closes early at 12:00, ending the session that
        // opened Thursday evening. The operator dates that session to this row's
        // trade date, so the Friday morning is the Monday's, not the holiday's.
        assert!(
            calendar
                .is_open(ct(friday, (11, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{friday:?}: the eve must be open before the early close"
        );
        assert!(
            !calendar
                .is_open(ct(friday, (12, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{friday:?}: the early close is end-exclusive"
        );
        assert_eq!(
            calendar
                .trade_date(ct(friday, (11, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some(day(trade_date.0, trade_date.1, trade_date.2)),
            "{friday:?}: the early-close morning belongs to the trade date the \
             operator prints on it, which is the Saturday session's"
        );
        // And no Friday-evening session is claimed: CME publishes none.
        assert!(
            !calendar
                .is_open(ct(friday, (18, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{friday:?}: no Friday-evening open is published for these dates"
        );

        // The Sunday-Monday part survives the row, and its regular session does
        // too: the envelope is one continuous span whose 08:30-15:15 slice is
        // also the family's `regular` session, and the block must not have
        // deleted it.
        let sunday = (saturday.0, saturday.1, saturday.2 + 1);
        assert!(
            calendar
                .is_open(ct(sunday, (18, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{sunday:?}: the Sunday-evening open was deleted by the row"
        );
        assert!(
            calendar
                .is_open(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the regular session was deleted by the row"
        );
        // The phase is the row's, not the ordinary week's: a replacement
        // replaces the complete trade date and the block scan selects by kind,
        // so a set that stated only the extended envelope would answer
        // `OpenExtended` here and move the regular bounds to the next day.
        assert!(
            calendar
                .is_open_regular(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: 10:00 CT must be inside the regular session the row states"
        );
        assert_eq!(
            calendar
                .session_state(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            SessionState::OpenRegular,
            "{trade_date:?}: the row must keep the regular phase on the trade date"
        );
        assert_eq!(
            calendar
                .session_bounds_with(ct(trade_date, (10, 0, 0)), SessionKind::Regular)
                .expect("2026 and 2027 are covered dates"),
            Some((ct(trade_date, (8, 30, 0)), ct(trade_date, (15, 15, 0)))),
            "{trade_date:?}: the regular bounds must be the trade date's own 08:30-15:15 CT"
        );
        assert!(
            calendar
                .is_open(ct(trade_date, (15, 45, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the post-regular slice was deleted by the row"
        );
        assert!(
            !calendar
                .is_open(ct(trade_date, (16, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the final close is end-exclusive"
        );
        assert_eq!(
            calendar
                .trade_date(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some(day(trade_date.0, trade_date.1, trade_date.2))
        );

        // The gap between the Saturday session and the Sunday open is closed.
        for hour in [17_u32, 20, 23] {
            assert!(
                !calendar
                    .is_open(ct(saturday, (hour, 0, 0)))
                    .expect("2026 and 2027 are covered dates"),
                "{saturday:?}: {hour}:00 falls between the blocks and must not be open"
            );
        }
    }
}

/// A Saturday-session row's Sunday legs must be sourced from a window that
/// actually prints them.
///
/// Two of these rows span **two** windows: the window that carries the Saturday
/// session stops at that Saturday and prints no Sunday entry at all, so the
/// Sunday Pre-Open, the Sunday-17:00 open and the trade date's `16:00 closed`
/// come from the window that starts on the Sunday. The third row's window
/// happens to run through its Sunday and does print the legs, so it needs only
/// the one.
///
/// This fence exists because an independent review found the two-window rows
/// claiming a Sunday pair that their cited artifact does not contain. It pins
/// the distinction that the per-row citation check cannot see.
#[test]
fn a_saturday_rows_sunday_legs_are_sourced_from_a_window_that_prints_them() {
    // (trade date, the window that prints the Saturday session, the window that
    // prints the Sunday legs)
    for (trade_date, saturday_window, sunday_window) in [
        ("2026-06-22", "CME-SVC-2026-06-18", "CME-SVC-2026-06-21"),
        ("2026-07-06", "CME-SVC-2026-07-03", "CME-SVC-2026-07-03"),
        ("2027-06-21", "CME-SVC-2027-06-17", "CME-SVC-2027-06-20"),
    ] {
        let evidence = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("docs/evidence/globex_equity_index.md"),
        )
        .expect("the evidence file must be readable");
        for window in [saturday_window, sunday_window] {
            assert!(
                evidence.contains(&format!("| `{window}` |")),
                "{trade_date}: {window} must be a recorded document"
            );
        }
        let row = evidence
            .lines()
            .find(|line| line.starts_with(&format!("| {trade_date} |")))
            .unwrap_or_else(|| panic!("{trade_date} must have an evidence row"));
        assert!(
            row.contains(saturday_window),
            "{trade_date}: the row must cite the Saturday session's window"
        );
        // Where the two differ, the row must name the second window too: the
        // legs the first does not print are not in it.
        if sunday_window != saturday_window {
            assert!(
                row.contains(sunday_window),
                "{trade_date}: the row must name {sunday_window}, which is the window \
                 that prints the Sunday legs"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The 2025-11-28 morning Pre-Open: order entry, not trading (issue #156).
// ---------------------------------------------------------------------------

/// 2025-11-28 is the one trade date in this table where the operator publishes a
/// second Pre-Open — `07:00 preopen; 07:30 open` — and CME's own event
/// vocabulary defines `preopen` as *"Order Entry, modification, and cancel are
/// allowed. **No order matching.**"*. The row therefore states `07:00-07:30` CT
/// as an `order_entry` window and matching resumes at the `07:30` `open`.
///
/// This is the fence for that reading. Before the correction the whole morning
/// was one continuous `extended` block, so `is_open` answered `true` at 07:15
/// and `session_state` answered `OpenExtended`: the crate claimed matching in a
/// window the operator prints as order-entry-only. The 2026 and 2027 Thanksgiving
/// Fridays publish the close line alone, so they must NOT gain this queue — the
/// per-row block assertions in this file's `shipped_rows` fence cover that, and
/// the 2026-11-27 and 2027-11-26 probes below pin it behaviourally.
#[test]
fn the_2025_thanksgiving_friday_serves_its_0700_pre_open_as_order_entry_only() {
    let calendar = equity_index();
    let holiday = calendar
        .holiday_on(day(2025, 11, 28))
        .expect("2025-11-28 ships a row");
    let HolidayKind::ReplacementBlocks(blocks) = holiday.kind() else {
        panic!(
            "2025-11-28 must be a replacement row, not {:?}",
            holiday.kind()
        );
    };

    // The row states the queue itself: one order-entry block covering exactly
    // 07:00-07:30 CT on the trade date. Its `open_day_offset` is 0 because the
    // Wednesday-evening run's wrapped block opens numerically later in the day,
    // and the table fence requires the list to be non-decreasing by opening day
    // then open time.
    let queues: Vec<_> = blocks
        .iter()
        .filter(|block| {
            block.kind() == ExceptionBlockKind::OrderEntry
                && block.open_day_offset() == 0
                && block.open_ssm() == 7 * 3_600
                && block.close_ssm() == 7 * 3_600 + 30 * 60
        })
        .collect();
    assert_eq!(
        queues.len(),
        1,
        "the row must state exactly one 07:00-07:30 CT order-entry window"
    );

    // Matching is off in the queue and on after its `open`.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (7, 15, 0)))
            .expect("the coverage contract must answer a covered date"),
        "07:15 CT is the operator's Pre-Open and must not report matching"
    );
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), (7, 45, 0)))
            .expect("the coverage contract must answer a covered date"),
        "07:45 CT is inside continuous trading and must report matching"
    );
    // The trade date does not move: the day still carries 2025-11-28.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (6, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );

    // The 2026 and 2027 Thanksgiving Fridays publish the final close alone, so
    // the same morning must still report matching there.
    for (year, month, date) in [(2026, 11, 27), (2027, 11, 26)] {
        assert!(
            calendar
                .is_open(ct((year, month, date), (7, 15, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} publishes no Pre-Open and must stay open at 07:15 CT"
        );
    }
}
