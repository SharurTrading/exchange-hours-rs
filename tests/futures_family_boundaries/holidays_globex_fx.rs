// SPDX-License-Identifier: MIT-0

//! `globex_fx` built-in holiday rows, 2025-2027, through the public surface.
//!
//! The family's grid is one wrapping Sunday-to-Thursday 17:00→16:00 CT matching
//! block, so every trading day opens on the previous local evening and every
//! row below is keyed by the crate's own venue-local trade date, never by the
//! operator's event date. Probes are stated in `America/Chicago` wall clock and
//! converted, because that is the clock the operator publishes in and the only
//! one in which a 12:45 CT close is recognisable.
//!
//! The design memo's §4.1 asks for seven cases per family. Six of them have
//! real rows here; the seventh — a late open — has none, because CME never
//! reopens standard-grid FX later than its normal 17:00 CT in this window. That
//! absence is fenced rather than skipped: every shipped row's kind is asserted,
//! so a late open cannot appear without a test changing.

use chrono::{DateTime, Datelike as _, Days, NaiveDate, TimeDelta, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DateCoverage, EvidenceTier, ExceptionBlock,
    ExceptionBlockKind, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_market_hours_key,
};

/// The family under test, as a date-aware calendar.
fn fx() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexFx)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// A probe instant, stated in the venue's own wall clock and converted.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous CT instant")
        .with_timezone(&Utc)
}

/// The verdict every probe below the permanent 2025-01-01 support floor now
/// carries (LAW-COVERAGE).
///
/// The eras below the floor left with Stage 5 of the release plan (#117): the
/// rows they shipped and the windows they were audited over are gone, so a
/// pre-floor date has neither a row nor an audited normal week, and the
/// refusal is what the contract states instead.
const BELOW_FLOOR: DateCoverage = DateCoverage::BeforeSupportFloor;

/// The verdict a date at or after the floor carries when the identity has no
/// sourced answer for it: the holiday layer has no audited window covering the
/// date, or a declared phase-level gap applies to it.
const OUTSIDE_COVERAGE: DateCoverage = DateCoverage::OutsideCoveredRange;

/// Asserts that an identity-backed query **refused** its date, with exactly the
/// verdict the shipped contract declares for it.
///
/// This is the assertion Stage 2B leaves where these tests used to read a
/// schedule: `CalendarQueryError` is how the crate says "I have no sourced
/// answer here", and LAW-COVERAGE requires that to be an explicit error rather
/// than a claim that a market is open or closed. Nothing is swallowed — an
/// answer where a refusal is due fails here, and so does a refusal carrying a
/// verdict other than the one the contract declares.
///
/// The verdict differs by *query* as well as by date, and that is a fact worth
/// stating: the floor-addressed entry points check the floor first, while
/// [`ExchangeCalendar::trade_date`] resolves the containing session before it
/// judges, so an instant with no containing session is refused by the declared
/// phase-level gap rather than by the floor. Each site below therefore names the
/// verdict it expects rather than deriving one here.
#[track_caller]
fn assert_refused<T: std::fmt::Debug>(
    query: Result<T, CalendarQueryError>,
    expected: DateCoverage,
) {
    // `expect_err` *is* the assertion: it aborts the test, naming the value,
    // when the contract answered a date it should have refused. The root
    // harness's `#![expect(clippy::expect_used)]` is what admits it here, as it
    // does every fixture constructor in this suite.
    let error = query.expect_err("the contract must refuse this date, but it answered");
    assert!(
        matches!(
            (expected, error),
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
        ),
        "the contract must refuse this date as {expected:?}, not {error}"
    );
}

/// Every trade date the module ships a row for, with the kind it ships.
///
/// This is the handwritten fence the charter asks for: it is compared against
/// the shipped table, never generated from it, so a row that appears, vanishes
/// or changes kind fails here.
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "midnight is written in the table fence's own `h * 3_600 + m * 60` grammar, \
              because the fence rejects a bare zero"
)]
fn shipped_rows() -> Vec<(NaiveDate, HolidayKind)> {
    // The Saturday-session rows: Thursday 16:45-17:00 CT Pre-Open queue and the
    // 17:00 CT session the Friday early close ends, Saturday 05:00-17:00 CT, the
    // Sunday Pre-Open queue and the Sunday-17:00-to-Monday-16:00 matching
    // session. The instants are the operator's, read from the window each row
    // cites.
    static BLOCKS: [ExceptionBlock; 5] = [
        ExceptionBlock::order_entry(-4, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-4, 17 * 3_600, 12 * 3_600),
        ExceptionBlock::extended(-2, 5 * 3_600, 17 * 3_600),
        ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
    ];
    // The merged trade dates: the Sunday-17:00-to-trade-date-16:00 span CME
    // assigns to the day after a Monday or Thursday holiday, whose `-1` queue is
    // the operator's holiday `16:00` rather than the family's weekday `16:45`.
    static MERGED: [ExceptionBlock; 4] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
        ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
    ];
    // Juneteenth 2025 falls on the Thursday, so the merged span opens on a
    // Wednesday and its `-2` queue is the ordinary weekday `16:45`.
    static MERGED_AFTER_WEEKDAY: [ExceptionBlock; 4] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
        ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 16 * 3_600),
    ];
    // Thanksgiving 2025: the merged span opens Wednesday evening, this trade
    // date's own close is the day-after-Thanksgiving 13:45, and it is the one
    // date in the window where the operator prints a second Pre-Open —
    // `07:00 preopen; 07:30 open` — so matching runs in two pieces with a
    // 30-minute queue between them. Carried in `extended`, that queue answered
    // `is_open = true`.
    static MERGED_EARLY_CLOSE: [ExceptionBlock; 7] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
        ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
        ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
        ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
        ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 13 * 3_600 + 45 * 60),
    ];
    // The 2026 and 2027 Thanksgiving Fridays publish the 13:45 close alone.
    static MERGED_EARLY_CLOSE_LATER: [ExceptionBlock; 4] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
        ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 13 * 3_600 + 45 * 60),
    ];
    let early = |hour: u32, minute: u32| HolidayKind::EarlyClose {
        close_ssm: hour * 3_600 + minute * 60,
    };
    let blocks = || HolidayKind::ReplacementBlocks(&BLOCKS);
    let merged = || HolidayKind::ReplacementBlocks(&MERGED);
    let merged_early = || HolidayKind::ReplacementBlocks(&MERGED_EARLY_CLOSE_LATER);
    vec![
        (day(2025, 1, 1), HolidayKind::Closed),
        (day(2025, 1, 21), merged()),
        (day(2025, 2, 18), merged()),
        (day(2025, 4, 18), HolidayKind::Closed),
        (day(2025, 5, 27), merged()),
        (
            day(2025, 6, 20),
            HolidayKind::ReplacementBlocks(&MERGED_AFTER_WEEKDAY),
        ),
        (day(2025, 7, 4), early(12, 0)),
        (day(2025, 9, 2), merged()),
        (
            day(2025, 11, 28),
            HolidayKind::ReplacementBlocks(&MERGED_EARLY_CLOSE),
        ),
        (day(2025, 11, 29), HolidayKind::Closed),
        (day(2025, 12, 24), early(12, 45)),
        (day(2025, 12, 25), HolidayKind::Closed),
        (day(2026, 1, 1), HolidayKind::Closed),
        (day(2026, 1, 20), merged()),
        (day(2026, 2, 17), merged()),
        (day(2026, 4, 3), early(10, 15)),
        (day(2026, 5, 26), merged()),
        (day(2026, 6, 19), early(12, 0)),
        (day(2026, 6, 22), blocks()),
        (day(2026, 7, 3), early(12, 0)),
        (day(2026, 7, 6), blocks()),
        (day(2026, 9, 8), merged()),
        (day(2026, 11, 27), merged_early()),
        (day(2026, 12, 24), early(12, 45)),
        (day(2026, 12, 25), HolidayKind::Closed),
        (day(2027, 1, 1), HolidayKind::Closed),
        (day(2027, 1, 19), merged()),
        (day(2027, 2, 16), merged()),
        (day(2027, 3, 26), HolidayKind::Closed),
        (day(2027, 6, 1), merged()),
        (day(2027, 6, 18), early(12, 0)),
        (day(2027, 6, 21), blocks()),
        (day(2027, 7, 6), merged()),
        (day(2027, 9, 7), merged()),
        (day(2027, 11, 26), merged_early()),
        (day(2027, 12, 24), HolidayKind::Closed),
    ]
}

// ---------------------------------------------------------------------------
// Case 1 — a closed day.
// ---------------------------------------------------------------------------

/// Christmas 2026 falls on a Friday, so the closure removes the family's whole
/// civil day: the leg that fed it opened Thursday evening and there is no
/// Friday-evening leg on this grid to survive it.
///
/// The row's own reading survives; the trade-date consequence is `None`, not an
/// error. `trade_date` resolves the instant's containing session and only then
/// judges the date, and no session contains a Christmas-Day instant on this
/// grid — an absent session, which LAW-COVERAGE keeps distinct from missing
/// evidence. The family declared a whole-domain `#93` phase gap until its
/// merged trade dates shipped as rows; that declaration was what turned this
/// absence into a refusal, and it is gone with the gap it described.
#[test]
fn christmas_2026_closes_the_whole_trade_date_and_its_previous_evening() {
    let calendar = fx();
    let holiday = calendar
        .holiday_on(day(2026, 12, 25))
        .expect("2026-12-25 ships a row");
    assert_eq!(holiday.kind(), HolidayKind::Closed);
    assert_eq!(holiday.tier(), EvidenceTier::T2);
    assert_eq!(holiday.document_id(), "CME-SVC-2026-12-24");

    assert!(
        calendar
            .is_closed_trade_date(day(2026, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_closed_all_day_on(day(2026, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    for probe in [(2, 0, 0), (10, 0, 0), (20, 0, 0)] {
        let instant = ct((2026, 12, 25), probe);
        assert!(
            !calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "Christmas Day must be closed at {instant}"
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            None,
            "no session contains {instant}, so it carries no trade date"
        );
    }

    // The previous evening's normal 17:00 CT open fed trade date 2026-12-25, so
    // it is gone with it.
    assert!(
        !calendar
            .is_open(ct((2026, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // From the eve's morning the next open is Sunday's reopen, two civil days
    // past the holiday.
    assert_eq!(
        calendar
            .next_session_open_after(ct((2026, 12, 24), (8, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 12, 27), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// Cases 2 and 3 — an early close, on each side of its cutoff.
// ---------------------------------------------------------------------------

/// The day after Thanksgiving 2025 closes at 13:45 CT. Thanksgiving itself
/// publishes no final close, so the span that ends here opened on Wednesday
/// evening and the whole of it carries trade date 2025-11-28 — which is why the
/// row is a replacement rather than the bare `EarlyClose` it used to be, and the
/// 13:45 is now the last block's end rather than the whole row's kind.
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "midnight is written in the table fence's own `h * 3_600 + m * 60` grammar, \
              because the fence rejects a bare zero"
)]
#[test]
fn the_2025_thanksgiving_friday_closes_at_1345_ct() {
    static EXPECTED: [ExceptionBlock; 7] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
        ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
        ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
        ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
        ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 13 * 3_600 + 45 * 60),
    ];
    let calendar = fx();
    assert_eq!(
        calendar
            .holiday_on(day(2025, 11, 28))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::ReplacementBlocks(&EXPECTED))
    );

    // Before the cutoff.
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), (13, 44, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    // At the cutoff: closes are end-exclusive.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (13, 45, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // And after it, for the rest of the civil day.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (15, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // 2025-11-28 is the one trade date in the window where the operator prints
    // a second Pre-Open (`07:00 preopen; 07:30 open`), so matching runs in two
    // pieces and the afternoon piece is the one the 13:45 close ends.
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 11, 28), (7, 30, 0)),
            ct((2025, 11, 28), (13, 45, 0))
        ))
    );
    // Before local midnight the same trading day is still in its holiday-evening
    // piece, which opens at 17:00 CT; after midnight the piece runs from the
    // civil day's start to the queue.
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 11, 27), (20, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 11, 27), (17, 0, 0)),
            ct((2025, 11, 28), (0, 0, 0))
        ))
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 11, 28), (6, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2025, 11, 28), (0, 0, 0)), ct((2025, 11, 28), (7, 0, 0))))
    );
    // The queue itself matches nothing.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (7, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), (7, 45, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 11, 28), (13, 45, 0)))
    );

    // Thanksgiving Thursday itself ships no row for this family — CME moved its
    // trade date, not its matching phases — so the Wednesday-evening leg that
    // feeds trade date 2025-11-27 is untouched.
    assert!(
        calendar
            .is_open(ct((2025, 11, 27), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

/// Good Friday 2026 is the exception CME's own page flags: this family traded
/// that morning and stopped at 10:15 CT, where the full Globex closure of Good
/// Friday 2025 and 2027 removes the day outright.
#[test]
fn good_friday_2026_closes_at_1015_ct_while_2025_and_2027_are_shut() {
    let calendar = fx();
    assert!(
        calendar
            .is_open(ct((2026, 4, 3), (10, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 4, 3), (10, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .candle_end(ct((2026, 4, 3), (6, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 4, 3), (10, 15, 0)))
    );

    for closed in [day(2025, 4, 18), day(2027, 3, 26)] {
        assert!(
            calendar
                .is_closed_trade_date(closed, SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
        assert!(
            calendar
                .is_closed_all_day_on(closed, SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
    }
}

// ---------------------------------------------------------------------------
// Case 4 — a late open. This family ships none, and that is fenced.
// ---------------------------------------------------------------------------

/// CME never reopens standard-grid FX later than its normal 17:00 CT in this
/// window, so no row is a late open and the post-closure reopen is the ordinary
/// one. Asserting every shipped kind is what keeps that true: a late open added
/// without a test fails here.
#[test]
fn the_family_ships_no_late_open_and_reopens_at_the_normal_1700_ct() {
    let calendar = fx();
    // Walk the whole coverage window, so a row on a date the fence does not
    // name fails here too: the shipped rows are exactly the handwritten list.
    let coverage = calendar
        .holiday_coverage()
        .expect("this family ships a table");
    let mut walked = Vec::new();
    // The handwritten fence below is the 2025-2027 block; the 2010-2012 rows
    // are fenced in this file's `era_` tests.
    let mut date = day(2025, 1, 1);
    let modern_last = day(2027, 12, 31);
    let _ = coverage.last();
    while date <= modern_last {
        if let Some(row) = calendar.holiday_on(date) {
            walked.push((date, row.kind()));
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }
    assert_eq!(
        walked,
        shipped_rows(),
        "the table's rows over its whole window are exactly the handwritten fence"
    );
    for (date, kind) in shipped_rows() {
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} must ship a row"));
        assert_eq!(row.kind(), kind, "{date} ships the wrong kind");
        assert!(
            matches!(
                row.kind(),
                HolidayKind::Closed
                    | HolidayKind::EarlyClose { .. }
                    | HolidayKind::ReplacementBlocks(_)
            ),
            "{date}: this family ships closures, early closes and the three \
             Saturday-session block sets only"
        );
        assert_eq!(row.tier(), EvidenceTier::T2, "{date} must be sourced at T2");
    }

    // Christmas Day 2025 is closed, and the evening of the holiday reopens for
    // the next trade date at the normal hour, not later.
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (16, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 12, 25), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The same after the Good Friday 2027 closure: Sunday's normal reopen.
    assert_eq!(
        calendar
            .next_session_open_after(ct((2027, 3, 25), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2027, 3, 28), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// Case 5 — a wrap removed by a closure.
// ---------------------------------------------------------------------------

/// Christmas Eve 2025 closes at 12:45 CT and Christmas Day is closed, so the
/// evening leg that would have carried trade date 2025-12-25 is deleted by the
/// closure rather than clipped by the early close.
#[test]
fn the_christmas_2025_closure_removes_the_eves_evening_wrap() {
    let calendar = fx();
    assert!(
        calendar
            .is_open(ct((2025, 12, 24), (12, 44, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (12, 45, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The eve's own trading day opened Tuesday evening and ends at the cutoff.
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 12, 24), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 12, 23), (17, 0, 0)),
            ct((2025, 12, 24), (12, 45, 0))
        ))
    );
    // No session at 17:30 CT on the eve: that leg's trade date is closed.
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // So the next open is Christmas evening, for trade date 2025-12-26.
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 12, 24), (12, 50, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 12, 25), (17, 0, 0)))
    );
    // Christmas Day is not closed *all day* — the next trade date's session
    // opens inside it. `is_closed_trade_date` is the holiday question.
    assert!(
        calendar
            .is_closed_trade_date(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// Case 6 — the trade-date consequence.
// ---------------------------------------------------------------------------

/// A shortened day keeps its own trade date; the evening leg that opens on a
/// closed holiday carries the *post*-holiday date.
#[test]
fn the_holiday_rows_keep_the_trade_dates_the_operator_prints() {
    let calendar = fx();
    // Inside the shortened Thanksgiving-Friday day.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    // The eve's evening open, on the closed holiday itself.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );
    // Juneteenth 2026: the operator prints the Friday noon early close against
    // this row's trade date, so the Thursday-evening leg that ends there belongs
    // to Monday 2026-06-22 rather than to the holiday Friday.
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 18), (20, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 6, 22))
    );
    assert!(
        calendar
            .is_open(ct((2026, 6, 19), (11, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 6, 19), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// Case 7 — both edges of the coverage window.
// ---------------------------------------------------------------------------

/// Inside the window a date with no row is audited normal; outside it the table
/// has no answer at all, and a known CME holiday one day — or one year — beyond
/// the edge is not applied.
///
/// The probes below now carry a refusal rather than a schedule. Christmas 2024
/// is a real CME closure one day below the floor — a row the module shipped
/// until Stage 5 removed the pre-floor eras — and 2028-01-03 is past the last
/// audited window, so the identity refuses each instead of extending its table.
/// The detached calendar still states the ordinary normal week there, which is
/// exactly the answer the identity does *not* borrow.
#[test]
fn the_table_answers_inside_its_window_and_nowhere_else() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a coverage window");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2025, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    // Christmas 2024 is a real CME closure one day below the window. The table
    // does not silently extend to it, and the contract refuses the date: it is
    // below the permanent floor, so there is no sourced normal week either.
    assert_eq!(calendar.holiday_on(day(2024, 12, 25)), None);
    let bare = calendar.without_holidays();
    assert_refused(
        calendar.is_open(ct((2024, 12, 25), (10, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(bare.is_open(ct((2024, 12, 25), (10, 0, 0))), BELOW_FLOOR);
    // And the first day above the window is refused as outside the covered
    // ranges rather than answered from the normal week the table never audited.
    let above = ct((2028, 1, 3), (10, 0, 0));
    assert_refused(calendar.is_open(above), OUTSIDE_COVERAGE);
    assert!(
        bare.is_open(above)
            .expect("the detached calendar states its normal week above the window"),
        "the normal week is the answer the identity refuses to borrow"
    );
}

// ---------------------------------------------------------------------------
// `without_holidays` restores the normal-week answer.
// ---------------------------------------------------------------------------

/// Detaching the table gives back exactly the pre-table calendar, and the
/// difference it makes on a holiday week is the rows themselves.
#[test]
fn without_holidays_restores_the_normal_week() {
    let calendar = fx();
    let bare = calendar.without_holidays();
    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2026, 12, 25)), None);

    // The normal week has a Thursday-evening leg into Christmas Friday 2026 and
    // a full Thursday day session; the table removes both.
    assert!(
        bare.is_open(ct((2026, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2026, 12, 24), (15, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2026, 12, 24), (15, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        bare.candle_end(ct((2026, 12, 24), (9, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2026, 12, 24), (16, 0, 0)))
    );

    // Away from the rows the two agree instant for instant. 2026-10-18 ..
    // 2026-10-24 is CME's own reference week for this family.
    //
    // All three answer everywhere in the week. `trade_date` is the one that can
    // be `None`: where no session contains the instant it reports the absence,
    // and the identity and the bare snapshot report the same one — which is
    // itself the "the table changes nothing here" claim. The identity declared
    // a whole-domain `#93` phase gap until this change; that declaration was
    // what turned these absences into refusals, and it went with the merged
    // trade dates it withheld.
    let mut instant = ct((2026, 10, 18), (0, 0, 0));
    let end = ct((2026, 10, 25), (0, 0, 0));
    while instant < end {
        assert_eq!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            bare.is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "is_open diverged at {instant} on an ordinary week"
        );
        if bare
            .is_open(instant)
            .expect("the detached calendar answers its own normal week")
        {
            assert_eq!(
                calendar
                    .trade_date(instant)
                    .expect("a session holds this instant, so the identity answers it"),
                bare.trade_date(instant)
                    .expect("a session holds this instant, so the snapshot answers it"),
                "trade_date diverged at {instant} on an ordinary week"
            );
        } else {
            assert_eq!(
                calendar
                    .trade_date(instant)
                    .expect("the coverage contract must answer a covered date"),
                bare.trade_date(instant)
                    .expect("the detached calendar answers its own normal week"),
                "trade_date diverged at {instant} on an ordinary week"
            );
        }
        assert_eq!(
            calendar
                .session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            bare.session_bounds(instant)
                .expect("the coverage contract must answer a covered date"),
            "session_bounds diverged at {instant} on an ordinary week"
        );
        instant += TimeDelta::minutes(37);
    }
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
/// 2010-04-02 Good Friday early close, the 2013-12-25 and 2024-12-25 closures,
/// the 2019-06-19 `Unsourced` row and the 2022-11-25 and 2024-12-24 early
/// closes. `holiday_on` answers `None` for all of them (the table has no
/// answer below the floor), the coverage no longer contains any of them, and
/// the date-aware query returns the explicit `BeforeSupportFloor` error
/// (LAW-COVERAGE), never a closure or a normal week read from the removed rows.
#[test]
fn pre_floor_rows_refuse_instead_of_answering() {
    let calendar = fx();

    for (year, month, date) in [
        (2010, 4, 2),
        (2013, 12, 25),
        (2016, 3, 25),
        (2019, 6, 19),
        (2022, 11, 25),
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
        assert_refused(
            calendar.is_closed_trade_date(removed, SessionKind::Both),
            BELOW_FLOOR,
        );
        assert_refused(
            calendar.is_open(ct((year, month, date), (10, 0, 0))),
            BELOW_FLOOR,
        );
    }

    // The evening leg that opens on 2024-12-31 belongs to trade date
    // 2025-01-01, whose own `Closed` row the module still ships — the retained
    // row carries the whole trading day including that wrap, so no pre-floor
    // row is load-bearing at or after the floor. The 2025-01-01 answers
    // themselves are fenced by the shipped-rows sweep and the case tests above.
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
// carrying the following Monday. Every expectation is read from the service
// window the row cites: the Saturday `05:00 open; 17:00 closed`, the Sunday
// `16:00 preopen; 17:00 open` and the following `16:00 closed`, all against the
// one trade date. The row states that whole day, because a replacement replaces
// the complete trade date.
// ---------------------------------------------------------------------------

/// A venue-local calendar date stated as `(year, month, day)`.
type Ymd = (i32, u32, u32);

/// The three trade dates, with the Saturday each session opens on.
const SATURDAY_TRADE_DATES: [(Ymd, Ymd); 3] = [
    ((2026, 6, 22), (2026, 6, 20)),
    ((2026, 7, 6), (2026, 7, 4)),
    ((2027, 6, 21), (2027, 6, 19)),
];

#[test]
fn a_saturday_session_row_states_the_whole_trade_date() {
    let calendar = fx();
    for (trade_date, saturday) in SATURDAY_TRADE_DATES {
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

        // The Saturday session, with an end-exclusive close.
        let saturday_open = ct(saturday, (5, 0, 0));
        let saturday_close = ct(saturday, (17, 0, 0));
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

        // The ordinary Sunday-Monday session survives the row, and the evening
        // open belongs to the same trade date.
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
            "{trade_date:?}: the day session was deleted by the row"
        );
        assert_eq!(
            calendar
                .trade_date(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some(day(trade_date.0, trade_date.1, trade_date.2))
        );
        assert!(
            !calendar
                .is_open(ct(trade_date, (16, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the final close is end-exclusive"
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

/// The replacement-block set the row keyed to `trade_date` declares.
///
/// The queue's instants are reachable through this one public accessor only:
/// `session_state`, `is_order_entry_only` and `is_accepting_orders` refuse the
/// two 2026 Sunday dates with `OutsideCoveredRange`, because the declared
/// phase-level gap for CME's Sunday quarter-hour (#79) is bounded to the era
/// before this family's 2026-08-22 knowledge-bound row, so no policy answer
/// reports the queue on those dates at all. `holiday_on` is not date-aware, so
/// it states the row the module ships on every one of the three dates.
#[expect(
    clippy::panic,
    reason = "a fixture row that is missing or of the wrong kind must fail loudly, \
              naming the value it found"
)]
fn declared_blocks(trade_date: Ymd) -> Vec<(ExceptionBlockKind, i8, u32, u32)> {
    let day = day(trade_date.0, trade_date.1, trade_date.2);
    let blocks = match fx().holiday_on(day).map(Holiday::kind) {
        Some(HolidayKind::ReplacementBlocks(blocks)) => blocks,
        other => panic!("{trade_date:?} must carry a replacement row, got {other:?}"),
    };
    blocks
        .iter()
        .map(|block| {
            (
                block.kind(),
                block.open_day_offset(),
                block.open_ssm(),
                block.close_ssm(),
            )
        })
        .collect()
}

/// The row's own bounds, the Sunday Pre-Open queue's interval, and the Sunday
/// evening open, are the instants the operator publishes for the date.
///
/// This is the fence an independent review found missing: the dedicated test
/// above probes the Saturday session and the existence of the Sunday-Monday
/// envelope, but never the queue or the envelope's own opening instant, so
/// moving either one changed no answer any test read.
#[test]
fn a_saturday_session_row_states_its_queue_and_evening_open() {
    for (trade_date, saturday) in SATURDAY_TRADE_DATES {
        let saturday_open = ct(saturday, (5, 0, 0));
        let saturday_close = ct(saturday, (17, 0, 0));
        let sunday = (saturday.0, saturday.1, saturday.2 + 1);
        let evening_open = ct(sunday, (17, 0, 0));
        let final_close = ct(trade_date, (16, 0, 0));

        assert_eq!(
            fx().session_bounds(ct(saturday, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((saturday_open, saturday_close)),
            "{trade_date:?}: the Saturday session's own bounds"
        );
        assert_eq!(
            fx().next_session_open_after(saturday_close)
                .expect("2026 and 2027 are covered dates"),
            Some(evening_open),
            "{trade_date:?}: the next open after the Saturday close is the Sunday \
             17:00 CT session the row states"
        );
        assert_eq!(
            fx().session_bounds(ct(sunday, (16, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((evening_open, final_close)),
            "{trade_date:?}: the Sunday-Monday session's own bounds"
        );
        assert_eq!(
            fx().session_bounds(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((evening_open, final_close)),
            "{trade_date:?}: the same session, probed on its closing day"
        );
        assert!(
            fx().is_open(evening_open)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Sunday 17:00 CT open must be inside the session"
        );
        assert!(
            !fx()
                .is_open(saturday_close)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Saturday close is end-exclusive"
        );

        // The row's own block set, with the queue's interval read off it: the
        // Thursday 16:45-17:00 CT Pre-Open queue at offset -4 and the session it
        // opens into, which ends at the Friday noon early close; the Saturday
        // session at offset -2; the 16:00-17:00 CT Pre-Open queue the operator
        // publishes for these dates at offset -1; and the ordinary
        // Sunday-17:00-to-Monday-16:00 session at offset -1.
        let expected = [
            (
                ExceptionBlockKind::OrderEntry,
                -4,
                16 * 3_600 + 45 * 60,
                17 * 3_600,
            ),
            (ExceptionBlockKind::Extended, -4, 17 * 3_600, 12 * 3_600),
            (ExceptionBlockKind::Extended, -2, 5 * 3_600, 17 * 3_600),
            (ExceptionBlockKind::OrderEntry, -1, 16 * 3_600, 17 * 3_600),
            (ExceptionBlockKind::Extended, -1, 17 * 3_600, 16 * 3_600),
        ];
        assert_eq!(
            declared_blocks(trade_date),
            expected.to_vec(),
            "{trade_date:?}: the row's own blocks, and the queue's 16:00-17:00 CT interval"
        );
        assert!(
            declared_blocks(trade_date)
                .windows(2)
                .all(|pair| (pair[0].1, pair[0].2) <= (pair[1].1, pair[1].2)),
            "{trade_date:?}: the row's blocks are ordered by opening day then open time"
        );
    }
}

// ---------------------------------------------------------------------------
// The 2025 merged trade dates.
// ---------------------------------------------------------------------------

/// A merged-date fixture: the eve the span opens on, the holiday whose own trade
/// date disappears, and the trade date the operator assigns the span to.
type MergedDates = ((i32, u32, u32), (i32, u32, u32), (i32, u32, u32));

/// The six 2025 trade dates whose span CME merges with the session before the
/// holiday: the holiday publishes no final close of its own, so the evening it
/// would have closed is printed against the next business day instead.
///
/// Probes are the instants the rows cite, and the expectation is the operator's
/// own `tradingDate` on those events — the eve's evening open and the holiday's
/// own evening open both belong to the trade date that follows. The day *after*
/// each merged date is probed too, because a replacement states the complete
/// trade date and must not have swallowed the ordinary week that resumes there.
#[test]
fn the_2025_merged_trade_dates_carry_the_operator_label() {
    let calendar = fx();
    // (eve of the merged span, holiday, merged trade date)
    let cases: [MergedDates; 6] = [
        ((2025, 1, 19), (2025, 1, 20), (2025, 1, 21)),
        ((2025, 2, 16), (2025, 2, 17), (2025, 2, 18)),
        ((2025, 5, 25), (2025, 5, 26), (2025, 5, 27)),
        ((2025, 6, 18), (2025, 6, 19), (2025, 6, 20)),
        ((2025, 8, 31), (2025, 9, 1), (2025, 9, 2)),
        ((2025, 11, 26), (2025, 11, 27), (2025, 11, 28)),
    ];
    for (eve, holiday, trade_date) in cases {
        for probe in [eve, holiday] {
            assert_eq!(
                calendar
                    .trade_date(ct(probe, (18, 0, 0)))
                    .expect("the coverage contract must answer a covered date"),
                Some(day(trade_date.0, trade_date.1, trade_date.2)),
                "{probe:?} 18:00 CT is inside the merged span and must carry \
                 the operator's trade date"
            );
            assert!(
                calendar
                    .is_open(ct(probe, (18, 0, 0)))
                    .expect("the coverage contract must answer a covered date"),
                "{probe:?} 18:00 CT is traded: the merge relabels the span, it \
                 does not delete it"
            );
        }
        // The holiday's own morning is inside the span the operator ran.
        assert!(
            calendar
                .is_open(ct(holiday, (10, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{holiday:?} morning is inside the merged span"
        );
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
    let calendar = fx();
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
