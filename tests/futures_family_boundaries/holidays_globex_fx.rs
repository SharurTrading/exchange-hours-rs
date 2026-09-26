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

use chrono::{
    DateTime, Datelike as _, Days, Duration, NaiveDate, TimeDelta, TimeZone as _, Utc, Weekday,
};
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
/// This file's `era_*` and `wave2_*` sweeps probe the 2010-2024 history, and
/// Stage 2B's identity-backed queries answer no date there: the floor is fixed
/// and never a rolling window, so the value the old assertions read is gone and
/// the refusal is what the contract states instead.
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
    // Thanksgiving 2025: the merged span opens Wednesday evening and this trade
    // date's own close is the day-after-Thanksgiving 13:45.
    static MERGED_EARLY_CLOSE: [ExceptionBlock; 4] = [
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
        (day(2026, 4, 3), early(10, 15)),
        (day(2026, 6, 19), early(12, 0)),
        (day(2026, 6, 22), blocks()),
        (day(2026, 7, 3), early(12, 0)),
        (day(2026, 7, 6), blocks()),
        (day(2026, 11, 27), early(13, 45)),
        (day(2026, 12, 24), early(12, 45)),
        (day(2026, 12, 25), HolidayKind::Closed),
        (day(2027, 1, 1), HolidayKind::Closed),
        (day(2027, 3, 26), HolidayKind::Closed),
        (day(2027, 6, 18), early(12, 0)),
        (day(2027, 6, 21), blocks()),
        (day(2027, 11, 26), early(13, 45)),
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
/// The row's own reading survives; what does not is the trade-date consequence.
/// `trade_date` resolves the instant's containing session *before* it judges the
/// date, and CME publishes holiday sessions this family's scalar vocabulary
/// cannot state, declared as the whole-domain `#93` phase gap — so an instant
/// with no containing session is refused rather than resolved through the
/// order-entry phase that would name the next trade date.
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
        assert_refused(calendar.trade_date(instant), OUTSIDE_COVERAGE);
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
#[test]
fn the_2025_thanksgiving_friday_closes_at_1345_ct() {
    static EXPECTED: [ExceptionBlock; 4] = [
        ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
        ExceptionBlock::extended(-2, 17 * 3_600, 16 * 3_600),
        ExceptionBlock::order_entry(-1, 16 * 3_600, 17 * 3_600),
        ExceptionBlock::extended(-1, 17 * 3_600, 13 * 3_600 + 45 * 60),
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
    // The trading day runs from Thursday's 17:00 CT open to the cutoff.
    assert_eq!(
        calendar
            .session_bounds(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 11, 27), (17, 0, 0)),
            ct((2025, 11, 28), (13, 45, 0))
        ))
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
/// Both probes below now carry a refusal rather than a schedule. Christmas 2009
/// precedes the permanent 2025-01-01 floor, and 2028-01-03 is past the last
/// audited window, so the identity refuses it instead of extending its table to
/// it. The detached calendar still states the ordinary normal week there, which
/// is exactly the answer the identity does *not* borrow.
#[test]
fn the_table_answers_inside_its_window_and_nowhere_else() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a coverage window");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2010, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    // Christmas 2009 is a real CME closure one year below the window. The table
    // does not silently extend to it, and the contract refuses the date: it is
    // below the permanent floor, so there is no sourced normal week either.
    assert_eq!(calendar.holiday_on(day(2009, 12, 25)), None);
    let bare = calendar.without_holidays();
    assert_refused(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(bare.is_open(ct((2009, 12, 25), (10, 0, 0))), BELOW_FLOOR);
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
    // `is_open` and `session_bounds` answer everywhere in the week and are
    // compared directly. `trade_date` does not: the identity's declared
    // whole-domain `#93` gap withholds the order-entry phase, so it answers only
    // where a session actually contains the instant and refuses the rest. The
    // detached calendar carries the same declaration, so both refuse the same
    // instants — which is itself the "the table changes nothing here" claim.
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
            assert_refused(calendar.trade_date(instant), OUTSIDE_COVERAGE);
            assert_refused(bare.trade_date(instant), OUTSIDE_COVERAGE);
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
// The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 15:15 CT, the era's Friday-holiday-eve final close.
const ERA_FRIDAY_EVE_CLOSE: u32 = 15 * 3_600 + 15 * 60;
/// 12:00 CT, the era's Monday/Thursday-holiday FX final close.
const ERA_NOON: u32 = 12 * 3_600;
/// 10:15 CT, the era's Good Friday FX final close.
const ERA_GOOD_FRIDAY_CLOSE: u32 = 10 * 3_600 + 15 * 60;
/// 05:00 CT, the era's post-holiday first open.
const ERA_REOPEN: u32 = 5 * 3_600;

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening: Martin Luther King Jr. Day 2010 stops at 12:00 CT and reopens the
/// same civil evening for the next trade date, and Good Friday 2010 stops at
/// 10:15 CT — the instant the 2026 table reuses for the same holiday.
///
/// The rows still read exactly as the module ships them. Every *query* below
/// probes a date below the permanent 2025-01-01 floor, so each states the
/// contract's refusal where it used to read the era's schedule (LAW-COVERAGE);
/// the schedule itself is no longer answerable at any resolution.
#[test]
fn era_early_closes_end_the_wrapped_trading_day_at_the_stated_instant() {
    let calendar = fx();

    // The MLK Friday eve, 2010-01-15, states the era's 15:15 CT early close.
    // It is asserted first because it is the row a mutation of any other
    // instant in this block would leave untouched: an unfenced row is one a
    // later edit can silently corrupt.
    let eve = calendar
        .holiday_on(day(2010, 1, 15))
        .expect("2010-01-15 ships a row");
    assert_eq!(
        eve.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_FRIDAY_EVE_CLOSE
        }
    );
    assert_eq!(eve.tier(), EvidenceTier::T1);
    assert_eq!(
        eve.document_id(),
        "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"
    );
    assert_refused(
        calendar.is_open(ct((2010, 1, 15), (15, 14, 59))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.is_open(ct((2010, 1, 15), (15, 15, 0))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.session_bounds(ct((2010, 1, 15), (9, 0, 0))),
        BELOW_FLOOR,
    );

    let mlk = calendar
        .holiday_on(day(2010, 1, 18))
        .expect("2010-01-18 ships a row");
    assert_eq!(
        mlk.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON
        }
    );
    assert_eq!(mlk.tier(), EvidenceTier::T1);
    assert_eq!(
        mlk.document_id(),
        "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z"
    );
    // The Sunday-evening leg that feeds this trade date is clipped, not
    // deleted: it still opens.
    assert_refused(calendar.is_open(ct((2010, 1, 17), (17, 0, 0))), BELOW_FLOOR);
    assert_refused(
        calendar.is_open(ct((2010, 1, 18), (11, 59, 59))),
        BELOW_FLOOR,
    );
    assert_refused(calendar.is_open(ct((2010, 1, 18), (12, 0, 0))), BELOW_FLOOR);
    assert_refused(
        calendar.session_bounds(ct((2010, 1, 18), (9, 0, 0))),
        BELOW_FLOOR,
    );
    // 17:00 CT the same civil evening opens the next trade date, which is
    // normal: this family publishes no holiday-evening row.
    assert_refused(
        calendar.trade_date(ct((2010, 1, 18), (18, 0, 0))),
        BELOW_FLOOR,
    );

    let good_friday = calendar
        .holiday_on(day(2010, 4, 2))
        .expect("2010-04-02 ships a row");
    assert_eq!(
        good_friday.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_GOOD_FRIDAY_CLOSE
        }
    );
    assert_refused(
        calendar.is_open(ct((2010, 4, 2), (10, 14, 59))),
        BELOW_FLOOR,
    );
    assert_refused(calendar.is_open(ct((2010, 4, 2), (10, 15, 0))), BELOW_FLOOR);
    assert_refused(
        calendar.session_bounds(ct((2010, 4, 2), (9, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.next_session_open_after(ct((2010, 4, 2), (10, 20, 0))),
        BELOW_FLOOR,
    );
}

/// Good Friday 2011 is a **full Globex closure**, unlike 2010 and 2012: CME's
/// 2011 sheet prints `CME Globex is closed` with no early close, so the row is
/// `Closed(2011-04-22)` and it removes the trading day that began Thursday
/// evening, exactly as the operator states.
///
/// The row is unchanged; the probes below are 2011 dates, so each carries the
/// contract's below-floor refusal rather than the closed/open answer the test
/// used to read.
#[test]
fn era_good_friday_2011_is_a_full_closure() {
    let calendar = fx();

    assert_eq!(
        calendar
            .holiday_on(day(2011, 4, 22))
            .expect("2011-04-22 ships a row")
            .kind(),
        HolidayKind::Closed
    );
    assert_refused(calendar.is_open(ct((2011, 4, 21), (17, 0, 0))), BELOW_FLOOR);
    assert_refused(calendar.is_open(ct((2011, 4, 22), (9, 0, 0))), BELOW_FLOOR);
    assert_refused(calendar.is_open(ct((2011, 4, 22), (15, 0, 0))), BELOW_FLOOR);
    // Nothing trades on the Friday evening either: CME prints the next open as
    // Sunday 2011-04-24 at 17:00 CT for trade date Monday 2011-04-25.
    assert_refused(calendar.is_open(ct((2011, 4, 22), (18, 0, 0))), BELOW_FLOOR);
    assert_refused(
        calendar.trade_date(ct((2011, 4, 24), (18, 0, 0))),
        BELOW_FLOOR,
    );
}

/// The era's late opens state 05:00 CT — earlier than the family's normal
/// 17:00 CT first open — so each cutoff lands on the trade date itself: the
/// leg that would have opened the Monday evening did not run, and trade date
/// 2011-12-27 begins on its own civil day.
///
/// The rows are unchanged; every probe is a 2011 instant, so each carries the
/// contract's below-floor refusal. The late-open shape itself — a session that
/// begins on the trade date's own civil day — is no longer observable, which is
/// why the row's kind and instant are the fence that remains.
#[test]
fn era_late_opens_land_on_the_trade_date_itself() {
    let calendar = fx();

    let holiday = calendar
        .holiday_on(day(2011, 12, 27))
        .expect("2011-12-27 ships a row");
    assert_eq!(
        holiday.kind(),
        HolidayKind::LateOpen {
            open_ssm: ERA_REOPEN
        }
    );
    assert_eq!(
        holiday.document_id(),
        "2011-christmas.pdf @2012-01-25T02:05:48Z"
    );
    assert_refused(
        calendar.is_open(ct((2011, 12, 26), (17, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.is_open(ct((2011, 12, 27), (4, 59, 59))),
        BELOW_FLOOR,
    );
    assert_refused(calendar.is_open(ct((2011, 12, 27), (5, 0, 0))), BELOW_FLOOR);
    assert_refused(
        calendar.session_bounds(ct((2011, 12, 27), (5, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.trade_date(ct((2011, 12, 27), (9, 0, 0))),
        BELOW_FLOOR,
    );
    // The next ordinary evening leg carries the next trade date.
    assert_refused(
        calendar.trade_date(ct((2011, 12, 27), (18, 0, 0))),
        BELOW_FLOOR,
    );

    // The era's three late opens are all the same 05:00 CT shape, so none can
    // take the preceding-local-date branch a 17:00 CT-or-later statement would.
    for (year, month, date) in [(2011, 12, 27), (2012, 1, 3), (2012, 12, 26)] {
        let row = calendar
            .holiday_on(day(year, month, date))
            .unwrap_or_else(|| panic!("{year}-{month:02}-{date:02} ships a row"));
        assert!(matches!(
            row.kind(),
            HolidayKind::LateOpen {
                open_ssm: ERA_REOPEN
            }
        ));
    }
}

/// A date inside the widened window with no row is audited normal: the crate
/// serves the profile's own 2010 week, and the detached calendar agrees, so
/// the answer comes from the row set and not from a profile change.
///
/// What survives is the pair of claims that never needed a schedule value: the
/// date ships no row, and the identity and the detached calendar refuse it for
/// the same reason — it precedes the permanent floor, so neither the table nor
/// its absence can change an answer on it.
#[test]
fn era_dates_without_rows_are_audited_normal() {
    let calendar = fx();
    let bare = calendar.without_holidays();

    for (date, previous_day, time) in [
        ((2010, 6, 15), (2010, 6, 14), (10, 0, 0)),
        ((2011, 3, 9), (2011, 3, 8), (9, 0, 0)),
        ((2012, 10, 10), (2012, 10, 9), (14, 0, 0)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?} must ship no row"
        );
        for probe in [(previous_day, (17, 0, 0)), (date, time)] {
            assert_refused(calendar.is_open(ct(probe.0, probe.1)), BELOW_FLOOR);
            assert_refused(bare.is_open(ct(probe.0, probe.1)), BELOW_FLOOR);
        }
    }
}

/// The widened window's edges answer as the module declares: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// not applied and the detached calendar agreeing.
///
/// The probe is a 2009 instant, so the contract refuses it below the floor; the
/// claim that survives is that the table carries no row for the date and does
/// not reach back to it.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = fx();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");

    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(!coverage.contains(day(2009, 12, 25)));

    assert_eq!(calendar.holiday_on(day(2009, 12, 25)), None);
    assert_refused(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(bare.is_open(ct((2009, 12, 25), (10, 0, 0))), BELOW_FLOOR);
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// The era's early closes clip the wrapped trading day at the printed instant,
/// and the three Independence Day eves CME prints no row for stay ordinary
/// trading days.
///
/// Every probe is a 2016-2018 instant, so each states the contract's below-floor
/// refusal. The rows and their instants are still read from `holiday_on` and the
/// cutoff arithmetic, which is what the sweep can still fence.
#[test]
fn wave2_early_closes_end_the_wrapped_trading_day_at_the_printed_instant() {
    let calendar = fx();

    for (date, previous_day, close_ssm) in [
        ((2016, 2, 15), (2016, 2, 14), 12 * 3_600),
        ((2017, 11, 23), (2017, 11, 22), 12 * 3_600),
        ((2016, 11, 25), (2016, 11, 24), 12 * 3_600 + 15 * 60),
        ((2018, 12, 24), (2018, 12, 23), 12 * 3_600 + 15 * 60),
    ] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose { close_ssm }),
            "{date:?}"
        );
        let cutoff = ct(date, (close_ssm / 3_600, (close_ssm % 3_600) / 60, 0));
        assert_refused(calendar.is_open(ct(previous_day, (17, 0, 0))), BELOW_FLOOR);
        assert_refused(
            calendar.is_open(cutoff - TimeDelta::seconds(1)),
            BELOW_FLOOR,
        );
        assert_refused(calendar.is_open(cutoff), BELOW_FLOOR);
        assert_refused(
            calendar.candle_end(ct(date, (9, 0, 0)), CalendarResolution::Daily),
            BELOW_FLOOR,
        );
    }

    // 2017-07-03 and 2018-07-03 are ordinary 16:00 CT days for this family.
    for date in [(2017, 7, 3), (2018, 7, 3)] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
        assert_refused(calendar.is_open(ct(date, (15, 30, 0))), BELOW_FLOOR);
    }
}

/// A closure removes its trade date and the prior-evening leg, and the era
/// ships no late open.
///
/// The row set still reads exactly as the module ships it. Every query below
/// probes a 2016-2018 instant and is refused below the floor, so the closure's
/// *schedule* consequence — which sessions disappear, and where the next one
/// opens — is no longer claimable; the era's closure list is what remains.
#[test]
fn wave2_closures_remove_the_trade_date_and_ship_no_late_open() {
    let calendar = fx();

    assert_eq!(
        calendar.holiday_on(day(2016, 3, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_refused(calendar.is_open(ct((2016, 3, 24), (18, 0, 0))), BELOW_FLOOR);
    assert_refused(calendar.is_open(ct((2016, 3, 25), (10, 0, 0))), BELOW_FLOOR);
    assert_refused(
        calendar.next_session_open_after(ct((2016, 3, 25), (10, 0, 0))),
        BELOW_FLOOR,
    );

    let mut closures = Vec::new();
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::Closed) => closures.push(date),
            Some(HolidayKind::EarlyClose { .. }) | None => {}
            Some(other) => panic!("{date}: the era ships no {other:?}"),
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        closures,
        [
            day(2016, 1, 1),
            day(2016, 3, 25),
            day(2016, 12, 26),
            day(2017, 1, 2),
            day(2017, 4, 14),
            day(2017, 12, 25),
            day(2018, 1, 1),
            day(2018, 3, 30),
            day(2018, 12, 25),
        ]
    );
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 12:15 CT, the era's Thanksgiving Friday close.
const ERA_QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;
/// 12:45 CT, the era's Christmas Eve 2024 close.
const ERA_TWELVE_FORTY_FIVE: u32 = 12 * 3_600 + 45 * 60;

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence: the sweep below walks the whole window
/// and compares against this list row for row, so a dropped, added or moved
/// row fails as loudly as a wrong instant. A sample would let a slipped close
/// move unnoticed on the dates nobody probed.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2022, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 16), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
        },
        EvidenceTier::T2,
    ),
    ((2024, 12, 25), HolidayKind::Closed, EvidenceTier::T2),
];

/// `ct` for a date the walk computed rather than spelled, so day arithmetic
/// cannot drift out of step with a hand-written tuple.
fn ct_on(date: NaiveDate, time: (u32, u32, u32)) -> DateTime<Utc> {
    ct((date.year(), date.month(), date.day()), time)
}

fn day_before(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// Whether this family's grid opens an evening leg on `day`.
///
/// The grid is one wrapping Sunday-through-Thursday 17:00 CT block, so Friday
/// and Saturday have no 17:00 CT occurrence: a 18:00 CT probe is held only when
/// its own day opens one, and a daytime probe only when the day before it does.
fn opens_evening(day: NaiveDate) -> bool {
    !matches!(day.weekday(), Weekday::Fri | Weekday::Sat)
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = fx();
    let mut index = 0_usize;
    let (mut quarter_past_noon, mut twelve_forty_five) = (0_usize, 0);
    let (mut closures, mut unsourced) = (0_usize, 0);
    let mut date = day(2022, 1, 1);
    while date <= day(2024, 12, 31) {
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
                    match close_ssm {
                        ERA_QUARTER_PAST_NOON => quarter_past_noon += 1,
                        ERA_TWELVE_FORTY_FIVE => twelve_forty_five += 1,
                        other => panic!("{date}: the era ships no {other} CT close"),
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date. Neither the
                    // clip nor the trade date is answerable any more: every
                    // instant here is a 2022-2024 one, below the permanent
                    // floor, so the contract refuses it.
                    assert_refused(
                        calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        BELOW_FLOOR,
                    );
                    assert_refused(
                        calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        BELOW_FLOOR,
                    );
                    // The everyday 17:00 CT leg holds this instant, so the
                    // refusal is the floor's rather than the phase gap's.
                    assert_refused(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                        BELOW_FLOOR,
                    );
                    // One second before the close is open; at it, closed.
                    assert_refused(
                        calendar.is_open(cutoff - TimeDelta::seconds(1)),
                        BELOW_FLOOR,
                    );
                    assert_refused(calendar.is_open(cutoff), BELOW_FLOOR);
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle.
                    assert_refused(calendar.session_bounds(ct_on(date, (9, 0, 0))), BELOW_FLOOR);
                    assert_refused(
                        calendar.candle_end(ct_on(date, (9, 0, 0)), CalendarResolution::Daily),
                        BELOW_FLOOR,
                    );
                    assert_refused(
                        calendar.trade_date(cutoff - TimeDelta::seconds(1)),
                        BELOW_FLOOR,
                    );
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
        (quarter_past_noon, twelve_forty_five, closures, unsourced),
        (3, 1, 7, 3),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
///
/// Every probe is a 2022-2024 instant, so the contract refuses it. The one
/// thing that changes the *verdict* is whether a session holds the instant:
/// `trade_date` resolves the containing session before it judges the date, so
/// the eve's ordinary 17:00 CT leg and the 17:00 CT reopen are refused below
/// the floor, while a probe on the deleted trade date's own civil day — where
/// no session ran — is refused by the declared phase-level gap instead. The
/// reopen instant is no longer readable from any query, so its fixture
/// arithmetic is gone with it.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = fx();
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(date.0, date.1, date.2);
        assert_refused(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            BELOW_FLOOR,
        );
        // The evening leg that would have carried this trade date is gone.
        assert_refused(
            calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
            BELOW_FLOOR,
        );
        assert_refused(
            calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
            BELOW_FLOOR,
        );
        // And so is the trade date's own civil day.
        assert_refused(calendar.is_open(ct_on(date, (9, 0, 0))), BELOW_FLOOR);
        assert_refused(calendar.is_open(ct_on(date, (15, 59, 0))), BELOW_FLOOR);
        // The walk falls through to the order-entry scan for this instant,
        // but the floor is checked first at every entry point, so the refusal
        // is the floor's: below 2025-01-01 no range is claimed for a phase gap
        // to be outside of.
        assert_refused(calendar.trade_date(ct_on(date, (10, 0, 0))), BELOW_FLOOR);
        assert_refused(
            calendar.next_session_open_after(ct_on(date, (10, 0, 0))),
            BELOW_FLOOR,
        );
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// Every query about an `Unsourced` date refuses exactly as the detached
/// calendar does: the row states that the date was audited, makes no scheduling
/// claim, and clips nothing.
///
/// The claim used to be observable as *answers* that matched the detached
/// calendar. Every probe here is a 2019-2023 instant, so the identity and the
/// detached snapshot now both refuse it below the permanent floor — and the
/// equality that survives is that neither verdict depends on the row.
///
/// `trade_date` is the one query whose verdict is not uniform, and it follows
/// the calendar rather than the row: it resolves the containing session before
/// it judges the date, so a probe a session holds is refused below the floor
/// while one no session holds is refused by the declared phase-level gap. On
/// this grid an opening day is Sunday through Thursday 17:00 CT and its session
/// runs to 16:00 CT the next local day, which is what `opens_evening` states.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = fx();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    // An `Unsourced` row closes nothing: the identity and the detached snapshot
    // refuse the date on the same terms, so the row is demonstrably not the
    // reason for either verdict.
    assert_refused(
        calendar.is_closed_trade_date(date, SessionKind::Both),
        BELOW_FLOOR,
    );
    assert_refused(
        detached.is_closed_trade_date(date, SessionKind::Both),
        BELOW_FLOOR,
    );

    for (probe, held) in [
        (
            ct_on(day_before(date), (18, 0, 0)),
            opens_evening(day_before(date)),
        ),
        (ct_on(date, (9, 0, 0)), opens_evening(day_before(date))),
        (ct_on(date, (15, 59, 0)), opens_evening(day_before(date))),
        (ct_on(date, (18, 0, 0)), opens_evening(date)),
    ] {
        // Every probe here is below the floor, and the floor is checked before
        // the phase declaration at every entry point, so `trade_date` refuses
        // with the floor's verdict whether or not a session holds the instant:
        // the earlier held/not-held split no longer selects a variant.
        let _ = held;
        assert_refused(calendar.is_open(probe), BELOW_FLOOR);
        assert_refused(detached.is_open(probe), BELOW_FLOOR);
        assert_refused(calendar.trade_date(probe), BELOW_FLOOR);
        assert_refused(detached.trade_date(probe), BELOW_FLOOR);
        assert_refused(calendar.session_bounds(probe), BELOW_FLOOR);
        assert_refused(detached.session_bounds(probe), BELOW_FLOOR);
        assert_refused(calendar.next_session_open_after(probe), BELOW_FLOOR);
        assert_refused(detached.next_session_open_after(probe), BELOW_FLOOR);
        assert_refused(
            calendar.candle_end(probe, CalendarResolution::Daily),
            BELOW_FLOOR,
        );
        assert_refused(
            detached.candle_end(probe, CalendarResolution::Daily),
            BELOW_FLOOR,
        );
    }
}

#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = fx();
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date.0, date.1, date.2);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_unsourced_changes_nothing(date, row, EvidenceTier::T2);
    }
}

/// The 2022-2024 window sits fifth in the declared coverage, its edges
/// answer, and the 2013-2015 interval below the 2016-2018 wave is a window of
/// its own since this wave shipped. (The 2019-2021 interval this test used to
/// fence became a window of its own when that wave shipped; the section below
/// fences it.)
#[test]
fn era_2022_2024_window_sits_fifth_and_the_2013_2015_era_is_audited() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2013, 1, 1), day(2015, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );
    assert!(coverage.contains(day(2022, 1, 1)));
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(coverage.contains(day(2021, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);

    // The 2013-2015 wave shipped after this test was written: 2015-12-24 is
    // its 12:15 CT early close and 2015-12-25 its Christmas closure.
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 24)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        })
    );
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    // The 2015 probes are below the permanent floor, so the two instants the
    // rows would have closed are refused rather than answered.
    assert_refused(
        calendar.is_open(ct((2015, 12, 24), (13, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.is_open(ct((2015, 12, 25), (10, 0, 0))),
        BELOW_FLOOR,
    );
    // Christmas 2015 is the 2013-2015 wave's own closure, and 2015-12-24 the
    // 12:15 CT early close it shipped with it.
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 24)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        })
    );
    assert_refused(
        calendar.is_open(ct((2015, 12, 25), (10, 0, 0))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.is_open(ct((2015, 12, 24), (13, 0, 0))),
        BELOW_FLOOR,
    );
}

/// The era's early closes are the family's own instants, not the equity
/// index's: 12:15 CT on the three Thanksgiving Fridays, and 12:45 CT on
/// Christmas Eve 2024 — and no other instant anywhere in the window.
///
/// The instants are still fenced through the rows themselves. The probes are
/// 2022-2024, so each states the contract's below-floor refusal rather than the
/// open/closed answer the test used to read.
#[test]
fn era_2022_2024_early_close_instants_are_the_familys_own() {
    let calendar = fx();

    for date in [(2022, 11, 25), (2023, 11, 24), (2024, 11, 29)] {
        assert_eq!(
            calendar
                .holiday_on(day(date.0, date.1, date.2))
                .map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: ERA_QUARTER_PAST_NOON
            }),
            "{date:?}"
        );
        assert_refused(calendar.is_open(ct(date, (12, 14, 59))), BELOW_FLOOR);
        assert_refused(calendar.is_open(ct(date, (12, 15, 0))), BELOW_FLOOR);
    }

    assert_eq!(
        calendar.holiday_on(day(2024, 12, 24)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE
        })
    );
    assert_refused(
        calendar.is_open(ct((2024, 12, 24), (12, 44, 59))),
        BELOW_FLOOR,
    );
    assert_refused(
        calendar.is_open(ct((2024, 12, 24), (12, 45, 0))),
        BELOW_FLOOR,
    );
    // The 16:00 CT final close the family prints on an ordinary day is not
    // reached on any of the four.
    for date in [
        (2022, 11, 25),
        (2023, 11, 24),
        (2024, 11, 29),
        (2024, 12, 24),
    ] {
        assert_refused(calendar.is_open(ct(date, (15, 0, 0))), BELOW_FLOOR);
    }
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
/// from or to, and a kind this family does not ship fails outright.
///
/// Every probe is a 2019-2021 instant and is refused below the permanent floor,
/// so the schedule consequence of each row — open one second before its printed
/// instant and closed at it, a closure taking the trade date with the leg that
/// opened it — is no longer claimable. What the sweep still fences, and what a
/// slipped close still fails on, is the row set: the kind and the instant
/// payload of every row, counted over the whole window.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = fx();
    let (mut noons, mut quarters, mut ten_fifteens) = (0_usize, 0, 0);
    let (mut closures, mut unsourced) = (0_usize, 0);
    let mut rows = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::EarlyClose { close_ssm } => {
                    match close_ssm {
                        ERA_NOON => noons += 1,
                        ERA_QUARTER_PAST_NOON => quarters += 1,
                        ERA_GOOD_FRIDAY_CLOSE => ten_fifteens += 1,
                        other => {
                            panic!("{date}: this family ships no {other}-second CT close here")
                        }
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date. Neither the
                    // clip nor the trade date is answerable any more: every
                    // instant here is a 2019-2021 one, below the permanent
                    // floor, so the contract refuses it.
                    assert_refused(
                        calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        BELOW_FLOOR,
                    );
                    assert_refused(
                        calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        BELOW_FLOOR,
                    );
                    // The everyday 17:00 CT leg holds this instant, so the
                    // refusal is the floor's rather than the phase gap's.
                    assert_refused(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                        BELOW_FLOOR,
                    );
                    // One second before the close is open; at it, closed.
                    assert_refused(
                        calendar.is_open(cutoff - TimeDelta::seconds(1)),
                        BELOW_FLOOR,
                    );
                    assert_refused(calendar.is_open(cutoff), BELOW_FLOOR);
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle. The probe sits just inside the
                    // session, so the 10:15 Good Friday close cannot make the
                    // query answer `None` instead.
                    let inside = cutoff - TimeDelta::minutes(1);
                    assert_refused(calendar.session_bounds(inside), BELOW_FLOOR);
                    assert_refused(
                        calendar.candle_end(inside, CalendarResolution::Daily),
                        BELOW_FLOOR,
                    );
                    assert_refused(
                        calendar.trade_date(cutoff - TimeDelta::seconds(1)),
                        BELOW_FLOOR,
                    );
                }
                HolidayKind::Closed => {
                    closures += 1;
                    assert_refused(
                        calendar.is_closed_trade_date(date, SessionKind::Both),
                        BELOW_FLOOR,
                    );
                    // The evening leg that would have carried this trade date
                    // is gone, and so is the trade date's own session.
                    assert_refused(
                        calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        BELOW_FLOOR,
                    );
                    assert_refused(
                        calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        BELOW_FLOOR,
                    );
                    assert_refused(calendar.is_open(ct_on(date, (9, 0, 0))), BELOW_FLOOR);
                    assert_refused(calendar.is_open(ct_on(date, (15, 59, 0))), BELOW_FLOOR);
                    // No session holds this instant — the closure deleted the
                    // trade date — so the walk falls through to the order-entry
                    // scan. The floor is checked first by every entry point, so
                    // the refusal is the floor's, not the declared phase gap's:
                    // below 2025-01-01 no range has been claimed for a phase gap
                    // to be outside of.
                    assert_refused(calendar.trade_date(ct_on(date, (10, 0, 0))), BELOW_FLOOR);
                }
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: this era ships no {other:?}"),
            }
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 35, "the era's rows");
    assert_eq!(
        (noons, quarters, ten_fifteens, closures, unsourced),
        (18, 5, 1, 8, 3),
        "the era's shape"
    );
}

/// Every `Unsourced` row the era ships changes no answer: the row states that
/// the date was audited, makes no scheduling claim, and clips nothing.
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = fx();
    let mut checked = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
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
#[test]
fn era_2019_2021_window_edges_answer_as_the_module_declares() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");
    let era = (day(2019, 1, 1), day(2021, 12, 31));

    assert!(
        coverage.windows().contains(&era),
        "the 2019-2021 window is declared as a window of its own"
    );
    let inside = |date: NaiveDate| era.0 <= date && date <= era.1;
    assert!(inside(day(2019, 1, 1)));
    assert!(inside(day(2021, 12, 31)));
    assert!(
        !inside(day(2018, 12, 31)),
        "2018-12-31 belongs to the 2016-2018 wave, not to this era"
    );
    assert!(
        !inside(day(2022, 1, 1)),
        "2022-01-01 belongs to the 2022-2024 wave, not to this era"
    );

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is an ordinary Friday this table audited.
    assert_eq!(
        calendar.holiday_on(day(2019, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(calendar.holiday_on(day(2021, 12, 31)), None);
    // The era's last day is a 2021 date, so the contract refuses it below the
    // floor; what the table states about it is the absent row above.
    assert_refused(calendar.is_open(ct((2021, 12, 31), (9, 0, 0))), BELOW_FLOOR);
    // The neighbouring dates, which other waves audit, carry no row here.
    assert_eq!(calendar.holiday_on(day(2018, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = fx();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_fx ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2013, 1, 1), day(2015, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );

    // Every shipped row of the era is inside the era's own window: the walk
    // reads the module, and the declared window is what must contain it.
    let mut rows = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
        if calendar.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 35, "the era's rows");
    for probe in [day(2018, 12, 31), day(2022, 1, 1)] {
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
        (2019, 1, 21),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 2, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2019, 4, 19), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2019, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2019, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2019, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2019, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2020, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 1, 20),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 2, 17),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2020, 4, 10), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2020, 5, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2020, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2020, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 9, 7),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2020, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2021, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2021, 1, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 2, 15),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 4, 2),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 5, 31),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2021, 6, 19), HolidayKind::Unsourced, EvidenceTier::T1),
    (
        (2021, 7, 5),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 9, 6),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2021, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2021, 12, 24), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2019_2021_rows_are_the_audited_date_kind_and_tier_set() {
    let calendar = fx();
    let mut index = 0_usize;
    let mut date = day(2019, 1, 1);
    while date <= day(2021, 12, 31) {
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
/// date set the counts cannot see.
///
/// Every probe is a 2013-2015 instant and is refused below the permanent floor,
/// so the schedule each row states is no longer readable; the row's kind and
/// its own instant payload remain the fence.
#[test]
fn era_2013_2015_sweeps_every_shipped_row_kind_and_instant() {
    let venue = fx();
    let (mut closed, mut early, mut late, mut both) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = day(2013, 1, 1);
    while date <= day(2015, 12, 31) {
        if let Some(row) = venue.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::Closed => {
                    closed += 1;
                    assert_refused(
                        venue.is_closed_trade_date(date, SessionKind::Both),
                        BELOW_FLOOR,
                    );
                    assert_refused(
                        venue.is_open(ct_on(day_before(date), (17, 0, 0))),
                        BELOW_FLOOR,
                    );
                }
                HolidayKind::EarlyClose { close_ssm } => {
                    early += 1;
                    let (h, m, s) = (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let cutoff = ct_on(date, (h, m, s));
                    assert_refused(venue.is_open(cutoff - Duration::seconds(1)), BELOW_FLOOR);
                    assert_refused(venue.is_open(cutoff), BELOW_FLOOR);
                    // The clipped leg still opens 17:00 CT the evening before,
                    // so a session holds this instant and the refusal is the
                    // floor's rather than the phase gap's.
                    assert_refused(venue.trade_date(cutoff - Duration::seconds(1)), BELOW_FLOOR);
                    assert_refused(
                        venue.candle_end(cutoff - Duration::minutes(1), CalendarResolution::Daily),
                        BELOW_FLOOR,
                    );
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late += 1;
                    let (h, m, s) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let open = ct_on(date, (h, m, s));
                    assert_refused(venue.is_open(open - Duration::seconds(1)), BELOW_FLOOR);
                    assert_refused(venue.is_open(open), BELOW_FLOOR);
                    // The late open lands on the trade date's own civil day and
                    // matching starts there, so a session holds this instant.
                    assert_refused(venue.trade_date(open + Duration::hours(1)), BELOW_FLOOR);
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
                    assert_refused(venue.is_open(open - Duration::seconds(1)), BELOW_FLOOR);
                    assert_refused(venue.is_open(open), BELOW_FLOOR);
                    assert_refused(venue.is_open(cutoff - Duration::seconds(1)), BELOW_FLOOR);
                    assert_refused(venue.is_open(cutoff), BELOW_FLOOR);
                }
                other => panic!("{date}: this era ships no {other:?}"),
            }
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        (closed, early, late, both),
        (8, 36, 3, 0),
        "the era's shape"
    );
}

/// The era is its own declared window: 2013-01-01 is inside it and 2012-12-31
/// and 2016-01-01 belong to the waves either side and lie outside it.
#[test]
fn era_2013_2015_window_edges_answer_as_the_module_declares() {
    let venue = fx();
    let coverage = venue.holiday_coverage().expect("globex_fx ships a table");
    let era = (day(2013, 1, 1), day(2015, 12, 31));

    assert!(
        coverage.windows().contains(&era),
        "the 2013-2015 window is declared as a window of its own"
    );
    assert!(coverage.contains(era.0));
    assert!(coverage.contains(era.1));

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is the last date the window declares.
    assert_eq!(
        venue.holiday_on(day(2013, 1, 1)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        coverage.contains(day(2015, 12, 31)),
        "the era's last day is inside the declared window"
    );
    // The neighbouring dates are their own eras' business: 2012-12-31 is
    // audited normal by the wave below, 2016-01-01 opens the next declared
    // window, and neither is this era.
    assert_eq!(venue.holiday_on(day(2012, 12, 31)), None);
    assert!(
        coverage.contains(day(2016, 1, 1)),
        "2016-01-01 is the next declared window's first day"
    );
    // A date below the January-2010 floor is outside every window, so this
    // table has no answer for it at all.
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert_eq!(venue.holiday_on(day(2009, 12, 31)), None);
}

/// The family's coverage names its windows in order, and the 2013-2015 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2013_2015_window_is_declared_in_order_and_bounds_every_row() {
    let venue = fx();
    let coverage = venue.holiday_coverage().expect("globex_fx ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
            (day(2013, 1, 1), day(2015, 12, 31)),
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2019, 1, 1), day(2021, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );

    let mut rows = 0_usize;
    let mut date = day(2013, 1, 1);
    while date <= day(2015, 12, 31) {
        if venue.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 47, "the era's rows");
    assert_eq!(venue.holiday_on(day(2012, 12, 31)), None);
    assert!(!coverage.contains(day(2028, 1, 1)));
    assert_eq!(venue.holiday_on(day(2028, 1, 1)), None);
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
            open_ssm: 5 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 1, 18),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 1, 21),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 2, 15),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 2, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2013, 3, 29), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 5, 24),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 8, 30),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2013, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2014, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 1, 17),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 1, 20),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 2, 14),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 2, 17),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    ((2014, 4, 18), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 5, 23),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 5, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 8, 29),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 9, 1),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2015, 1, 16),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 1, 19),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 2, 13),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 2, 16),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 4, 3),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 5, 22),
        HolidayKind::EarlyClose {
            close_ssm: 15 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 5, 25),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 9, 7),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2015, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2013_2015_rows_are_the_audited_date_kind_and_tier_set() {
    let venue = fx();
    let mut index = 0_usize;
    let mut date = day(2013, 1, 1);
    while date <= day(2015, 12, 31) {
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
