// SPDX-License-Identifier: MIT-0

//! `globex_energy`'s built-in holiday rows, 2025-2027, through the public
//! surface only.
//!
//! The family's trading day wraps: one occurrence opens 17:00 CT on the
//! previous local day and closes 16:00 CT on the trade date. That is what
//! makes these cases worth fencing — a row is stated on the **trade date**, so
//! an early close has to land on a session that opened the evening before, a
//! closure has to delete that evening leg rather than the holiday's own
//! daytime, and the eve's own trade date has to survive both.
//!
//! Every probe is stated in `America/Chicago` wall clock and converted, so a
//! DST slip in either direction fails rather than passing on a coincidence.
//! The seven cases of the design memo's §4.1 are covered in order, with the
//! late-open case answered by proving the family ships no late open in this
//! window rather than by inventing one.

use chrono::{DateTime, Datelike as _, Days, NaiveDate, TimeDelta, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, DateCoverage, EvidenceTier, ExceptionBlockKind,
    ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind,
    calendar_for_market_hours_key,
};

const KEY: MarketHoursKey = MarketHoursKey::GlobexEnergy;

/// A venue-local calendar date stated as `(year, month, day)`.
type Ymd = (i32, u32, u32);

fn calendar() -> ExchangeCalendar {
    calendar_for_market_hours_key(KEY)
}

/// A probe instant stated in the venue's own wall clock.
fn ct(date: Ymd, hour: u32, minute: u32) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, hour, minute, 0)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(date: Ymd) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn one_second_before(instant: DateTime<Utc>) -> DateTime<Utc> {
    instant - TimeDelta::seconds(1)
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

/// Case 1 — a closed day. Christmas 2025 falls on a Thursday, so its trade
/// date is removed together with the Wednesday-evening leg that fed it.
#[test]
fn christmas_2025_removes_the_whole_trade_date() {
    let calendar = calendar();
    let christmas = day((2025, 12, 25));

    assert!(
        calendar
            .is_closed_trade_date(christmas, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .holiday_on(christmas)
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );
    for (hour, minute) in [(0_u32, 30_u32), (10, 0), (15, 30)] {
        assert!(
            !calendar
                .is_open(ct((2025, 12, 25), hour, minute))
                .expect("the coverage contract must answer a covered date"),
            "the market is closed all of Christmas Day at {hour:02}:{minute:02} CT"
        );
    }
}

/// Case 5 — the wrap a closure removes, and where the next session actually
/// opens. Christmas Eve keeps its own trade date and closes early at 12:45 CT;
/// the 17:00 CT leg that would have opened trade date 2025-12-25 is gone, and
/// the next open is 17:00 CT on Christmas Day itself, for trade date 12-26.
#[test]
fn the_christmas_eve_evening_leg_is_removed_and_the_next_open_is_the_holiday_evening() {
    let calendar = calendar();

    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), 17, 30))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), 23, 0))
            .expect("the coverage contract must answer a covered date")
    );

    let (open, close) = calendar
        .next_session_after(ct((2025, 12, 24), 13, 0))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(open, ct((2025, 12, 25), 17, 0));
    assert_eq!(close, ct((2025, 12, 26), 16, 0));
}

/// Cases 2 and 3 — the instant before an early close and the instant at it.
/// The day after Thanksgiving 2025 closes 13:45 CT; the session it ends opened
/// at 17:00 CT the evening before, on a trade date that is itself an early
/// close at 13:30 CT and does not clip this one.
#[test]
fn the_2025_black_friday_early_close_is_end_exclusive_on_its_own_trade_date() {
    let calendar = calendar();
    let cutoff = ct((2025, 11, 28), 13, 45);

    assert!(
        calendar
            .is_open(one_second_before(cutoff))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );

    // This is the one trade date in the window where the operator prints a
    // second Pre-Open — `07:00 preopen; 07:30 open` — so matching runs in two
    // pieces and the afternoon piece is the one the 13:45 close ends.
    let (open, close) = calendar
        .session_bounds(ct((2025, 11, 28), 10, 0))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(open, ct((2025, 11, 28), 7, 30));
    assert_eq!(close, cutoff);
    // The piece before the queue is the midnight-to-queue run; the piece that
    // opened at the holiday's 17:00 CT the previous evening is the one before
    // that, reached by probing before local midnight.
    let (pre_midnight_open, pre_midnight_close) = calendar
        .session_bounds(ct((2025, 11, 27), 20, 0))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(pre_midnight_open, ct((2025, 11, 27), 17, 0));
    assert_eq!(pre_midnight_close, ct((2025, 11, 28), 0, 0));
    let (tail_open, tail_close) = calendar
        .session_bounds(ct((2025, 11, 28), 6, 0))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(tail_open, ct((2025, 11, 28), 0, 0));
    assert_eq!(tail_close, ct((2025, 11, 28), 7, 0));
    // The queue itself matches nothing.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), 7, 15))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), 7, 45))
            .expect("the coverage contract must answer a covered date")
    );

    assert_eq!(
        calendar
            .candle_end(ct((2025, 11, 28), 10, 0), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(cutoff)
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), 15, 0))
            .expect("the coverage contract must answer a covered date")
    );
}

/// The Thanksgiving pre-open shape CME publishes instead of a close: matching
/// stops at 13:30 CT and the ordinary 17:00 CT open still starts the next
/// trade date, so one civil day carries the end of one trading day and the
/// start of the next.
#[test]
fn the_2025_thanksgiving_early_close_stops_matching_at_13_30_and_reopens_at_17_00() {
    let calendar = calendar();
    let cutoff = ct((2025, 11, 27), 13, 30);

    assert!(
        calendar
            .is_open(one_second_before(cutoff))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 27), 16, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2025, 11, 27), 17, 0))
            .expect("the coverage contract must answer a covered date")
    );
}

/// The Friday holidays of 2026 and 2027, where CME prints the early close on the
/// Friday but dates its session to the following Monday. The row is keyed to the
/// trade date the operator prints, so the Thursday-evening leg the close ends is
/// kept and resolves to that Monday.
#[test]
fn juneteenth_2026_clips_the_friday_and_keys_the_thursday_leg_to_the_monday() {
    let calendar = calendar();
    let cutoff = ct((2026, 6, 19), 12, 0);

    assert!(
        calendar
            .is_open(ct((2026, 6, 18), 17, 30))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(one_second_before(cutoff))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2026, 6, 19), 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2026, 6, 18), 17, 0), cutoff))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 18), 18, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2026, 6, 22)))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 19), 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2026, 6, 22)))
    );
}

/// Case 4 — a late open. This family has none in the published window: CME
/// moves the pre-open on a holiday but never the 17:00 CT open itself, so
/// every deviation is a close or a closure. The absent case is fenced rather
/// than skipped, because a row of the wrong kind would otherwise ship unseen.
#[test]
fn no_late_open_row_ships_in_the_published_window() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a holiday table");

    let mut date = coverage.first();
    while date <= coverage.last() {
        if let Some(holiday) = calendar.holiday_on(date) {
            // `Unsourced` joined the vocabulary with the 2022-2024 wave: it
            // states that a date inside a window was audited, clips nothing,
            // and is not a late open. `ReplacementBlocks` joined it in Stage 4:
            // it states a complete trading day, which is a closure of a
            // different shape rather than a boundary move. The fence this test
            // draws is unchanged — no row states a later first open.
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed
                        | HolidayKind::EarlyClose { .. }
                        | HolidayKind::ReplacementBlocks(_)
                        | HolidayKind::Unsourced
                ),
                "globex_energy ships no late open: {date} is {:?}",
                holiday.kind()
            );
            assert!(
                !matches!(
                    holiday.kind(),
                    HolidayKind::LateOpen { .. } | HolidayKind::LateOpenAndEarlyClose { .. }
                ),
                "globex_energy ships no late open: {date}"
            );
        }
        date = date
            .succ_opt()
            .expect("the coverage window ends well inside the calendar range");
    }
}

/// Case 6 — the trade-date consequence. A shortened day keeps its own trade
/// date, the evening leg of an early-close day already carries the next one,
/// and the evening of a closed day carries the post-holiday date.
///
/// The three answers inside a session survive. The fourth probe — Christmas Day
/// at 10:00 CT, which no session holds — does not: `trade_date` resolves the
/// containing session before it judges the date, and CME's Sunday 16:00-16:15 CT
/// quarter-hour is withheld as the declared `#79` phase-level gap over the whole
/// era before 2026-08-22, so the query is refused rather than resolved through
/// the order-entry phase that would name the next trade date.
#[test]
fn trade_dates_follow_the_rows_rather_than_the_civil_day() {
    let calendar = calendar();

    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), 12, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 27), 18, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 25), 18, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 12, 26)))
    );
    assert_refused(
        calendar.trade_date(ct((2025, 12, 25), 10, 0)),
        OUTSIDE_COVERAGE,
    );
}

/// Case 7 — both edges of the coverage window. Inside it a date with no row is
/// audited normal; outside it the table has no answer at all and must not
/// silently extend to a holiday it never audited.
///
/// Christmas 2028 is past the last audited window, so the identity refuses it
/// rather than extending its table to it. The detached calendar is compared on
/// its own terms below: it claims only the normal week, and the date is above
/// its sourced start, so it still answers where the identity does not.
#[test]
fn the_coverage_window_bounds_every_answer_the_table_gives() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a holiday table");

    assert_eq!(coverage.first(), day((2025, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(coverage.contains(day((2025, 1, 1))));
    assert!(coverage.contains(day((2027, 12, 31))));

    assert_eq!(
        calendar
            .holiday_on(day((2025, 1, 1)))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(calendar.holiday_on(day((2024, 12, 31))).is_none());
    assert!(calendar.holiday_on(day((2028, 1, 1))).is_none());
    // The pre-floor eras left with Stage 5 of the release plan (#117), so
    // 2024-12-31's `None` is "no answer" rather than "audited normal": the
    // window opens at the permanent 2025-01-01 floor and reaches no earlier.
    assert!(!coverage.contains(day((2024, 12, 31))));

    // The probe outside the audited window is Christmas 2028, which the
    // family's table ends before: it must answer exactly as the normal week
    // does. The window's every date is at or after the floor, so the removed
    // pre-floor rows cannot reach an answer the table still gives.
    assert!(!coverage.contains(day((2028, 12, 25))));
    let probe = ct((2028, 12, 25), 10, 0);
    assert!(calendar.holiday_on(probe.date_naive()).is_none());
    assert_refused(calendar.is_open(probe), OUTSIDE_COVERAGE);
    assert!(
        calendar
            .without_holidays()
            .is_open(probe)
            .expect("the detached calendar states its normal week above the window"),
        "the normal week is the answer the identity refuses to borrow"
    );
}

/// `without_holidays` is the exact undo: it restores the normal-week answer on
/// a holiday week and changes nothing on an ordinary one.
#[test]
fn without_holidays_restores_the_normal_week_answer() {
    let calendar = calendar();
    let detached = calendar.without_holidays();

    assert!(detached.holiday_on(day((2025, 12, 25))).is_none());
    assert!(detached.holiday_coverage().is_none());
    assert!(
        detached
            .is_open(ct((2025, 12, 25), 10, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        detached
            .is_open(ct((2025, 11, 28), 15, 0))
            .expect("the coverage contract must answer a covered date")
    );

    // A dense grid over the 2025 Christmas week: the two answers differ only
    // where a row says so, and an ordinary week later agrees at every probe.
    let mut differences = 0_u32;
    for offset in 0..7_u64 {
        let date = day((2025, 12, 22))
            .checked_add_days(Days::new(offset))
            .expect("the fixture week is inside the calendar range");
        for hour in 0..24_u32 {
            let probe = US::Central
                .from_local_datetime(
                    &date
                        .and_hms_opt(hour, 0, 0)
                        .expect("a whole hour is a valid local time"),
                )
                .single()
                .expect("Central has no ambiguous hour in the fixture week")
                .with_timezone(&Utc);
            if calendar
                .is_open(probe)
                .expect("the coverage contract must answer a covered date")
                != detached
                    .is_open(probe)
                    .expect("the coverage contract must answer a covered date")
            {
                differences = differences.saturating_add(1);
            }
        }
    }
    assert!(
        differences > 0,
        "the Christmas week must differ once the table is attached"
    );

    for offset in 0..7_u64 {
        let date = day((2026, 10, 19))
            .checked_add_days(Days::new(offset))
            .expect("the reference week is inside the calendar range");
        for hour in 0..24_u32 {
            let probe = US::Central
                .from_local_datetime(
                    &date
                        .and_hms_opt(hour, 0, 0)
                        .expect("a whole hour is a valid local time"),
                )
                .single()
                .expect("Central has no ambiguous hour in the reference week")
                .with_timezone(&Utc);
            assert_eq!(
                calendar
                    .is_open(probe)
                    .expect("the coverage contract must answer a covered date"),
                detached
                    .is_open(probe)
                    .expect("the coverage contract must answer a covered date"),
                "an audited-normal week must answer identically at {probe}"
            );
        }
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
/// 2010-04-02 Good Friday closure, the 2013-12-25 and 2024-12-25 closures, the
/// 2019-06-19 `Unsourced` row, the 2022-06-20 Juneteenth 13:30 CT early close
/// and the 2024-12-24 early close. `holiday_on` answers `None` for all of them
/// (the table has no answer below the floor), the coverage no longer contains
/// any of them, and the date-aware query returns the explicit
/// `BeforeSupportFloor` error (LAW-COVERAGE), never a closure or a normal week
/// read from the removed rows.
#[test]
fn pre_floor_rows_refuse_instead_of_answering() {
    let calendar = calendar();

    for (year, month, date) in [
        (2010, 4, 2),
        (2013, 12, 25),
        (2019, 6, 19),
        (2022, 6, 20),
        (2024, 12, 24),
        (2024, 12, 25),
    ] {
        let removed = day((year, month, date));
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
            calendar.is_open(ct((year, month, date), 10, 0)),
            BELOW_FLOOR,
        );
    }

    // The evening leg that opens on 2024-12-31 belongs to trade date
    // 2025-01-01, whose own `Closed` row the module still ships — the retained
    // row carries the whole trading day including that wrap, so no pre-floor
    // row is load-bearing at or after the floor.
    assert_eq!(
        calendar.holiday_on(day((2024, 12, 31))),
        None,
        "2024-12-31 ships no row: its evening leg is trade date 2025-01-01's"
    );
    assert_eq!(
        calendar.holiday_on(day((2025, 1, 1))).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "trade date 2025-01-01 keeps its own closure, wrap included"
    );
}

// ---------------------------------------------------------------------------
// The Saturday-session trade dates (Stage 4, #116)
//
// CME states a Saturday session on three trade dates in this window, each
// carrying the following Monday. Every expectation below is read from the
// service window the row cites, not from the module's own tables: the operator
// prints the Saturday `05:00 open; 17:00 closed`, the Sunday
// `16:00 preopen; 17:00 open` and the following `16:00 closed`, all against the
// one trade date. The row states that whole day in blocks, because a
// replacement replaces the complete trade date.
// ---------------------------------------------------------------------------

/// The three trade dates CME states a Saturday session for, with the Saturday
/// each session opens on.
const SATURDAY_SESSION_TRADE_DATES: [(Ymd, Ymd); 3] = [
    ((2026, 6, 22), (2026, 6, 20)),
    ((2026, 7, 6), (2026, 7, 4)),
    ((2027, 6, 21), (2027, 6, 19)),
];

#[test]
fn a_saturday_session_row_states_the_complete_trade_date() {
    let calendar = calendar();
    for (trade_date, saturday) in SATURDAY_SESSION_TRADE_DATES {
        assert_eq!(
            day(saturday).weekday(),
            Weekday::Sat,
            "{saturday:?} must be the Saturday the session opens on"
        );
        assert_eq!(day(trade_date).weekday(), Weekday::Mon, "{trade_date:?}");
        assert!(
            matches!(
                calendar.holiday_on(day(trade_date)).map(Holiday::kind),
                Some(HolidayKind::ReplacementBlocks(_))
            ),
            "{trade_date:?} must carry a replacement row"
        );

        // The Saturday session itself: open on its first instant, open through
        // its last, and closed at its end-exclusive close.
        let saturday_open = ct(saturday, 5, 0);
        let saturday_close = ct(saturday, 17, 0);
        assert!(
            calendar
                .is_open(saturday_open)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Saturday session is not open at {saturday_open}"
        );
        assert!(
            calendar
                .is_open(one_second_before(saturday_close))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Saturday session closed early"
        );
        assert!(
            !calendar
                .is_open(saturday_close)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Saturday close is end-exclusive"
        );
        assert_eq!(
            calendar
                .session_bounds(ct(saturday, 10, 0))
                .expect("2026 and 2027 are covered dates"),
            Some((saturday_open, saturday_close)),
            "{trade_date:?}: the Saturday session's own bounds"
        );

        // Every instant of the day belongs to the following Monday, including
        // the Saturday block and the ordinary Sunday-Monday session.
        for (when, hour) in [
            (saturday, 5_u32),
            (saturday, 12),
            ((saturday.0, saturday.1, saturday.2 + 1), 18),
            (trade_date, 10),
        ] {
            assert_eq!(
                calendar
                    .trade_date(ct(when, hour, 0))
                    .expect("2026 and 2027 are covered dates"),
                Some(day(trade_date)),
                "{trade_date:?}: {when:?} {hour}:00 must belong to the trade date"
            );
        }

        // The ordinary Sunday-Monday session belongs to the same trade date and
        // is not deleted by the row: it opens 17:00 CT the evening before and
        // closes 16:00 CT on the trade date.
        let session_open = ct((saturday.0, saturday.1, saturday.2 + 1), 17, 0);
        let session_close = ct(trade_date, 16, 0);
        assert!(
            calendar
                .is_open(ct(trade_date, 10, 0))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Sunday-Monday session was deleted by the row"
        );
        assert_eq!(
            calendar
                .session_bounds(ct(trade_date, 10, 0))
                .expect("2026 and 2027 are covered dates"),
            Some((session_open, session_close)),
            "{trade_date:?}: the Sunday-Monday session's bounds"
        );
        assert!(
            !calendar
                .is_open(session_close)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the session close is end-exclusive"
        );

        // The gap between the Saturday session and the Sunday one is a pause on
        // a closed market, not a session of its own.
        for hour in [17_u32, 20, 23] {
            assert!(
                !calendar
                    .is_open(ct(saturday, hour, 0))
                    .expect("2026 and 2027 are covered dates"),
                "{trade_date:?}: {saturday:?} {hour}:00 falls between the blocks \
                 and must not be open"
            );
        }
    }
}

#[test]
fn a_saturday_session_row_keeps_the_ordinary_sunday_queue() {
    let calendar = calendar();
    for (trade_date, saturday) in SATURDAY_SESSION_TRADE_DATES {
        let sunday = (saturday.0, saturday.1, saturday.2 + 1);
        // CME prints `16:00 preopen; 17:00 open` for these Sundays: the queue
        // runs from 16:00 CT and no trade matches before the 17:00 open.
        // The queue is stated by the row. Whether the order-entry **gate** will
        // answer it is #132's subject: `contains_order_entry` asks
        // `require_phase_coverage` about the local day the queue opens on, a
        // Sunday, rather than about the trade date the queue belongs to, and
        // whether that day is inside the published range depends on which
        // revision is in force — so 2026-06-21 is refused here and 2027-06-20 is
        // answered. The defect is pre-existing and independent of any block row
        // (`is_open` answers the same instant on unmodified `main`), and #132
        // owns its fix. Either verdict is accepted, so this test does not pin a
        // defect as correct; what it does pin is that no **trade** prints in the
        // queue and that a refusal is the coverage refusal rather than a wrong
        // answer.
        let queue_probe = ct(sunday, 16, 30);
        match calendar.is_order_entry_only(queue_probe) {
            Ok(order_entry_only) => assert!(
                order_entry_only,
                "{trade_date:?}: {queue_probe} is inside the stated queue but not \
                 order-entry-only"
            ),
            Err(error) => assert!(
                matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
                "{trade_date:?}: the queue was refused for the wrong reason: {error:?}"
            ),
        }
        assert!(
            !calendar
                .is_open(queue_probe)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: no trade may print in the Sunday queue at {queue_probe}"
        );
        // The queue runs to the 17:00 CT open, and the session is open from it.
        assert!(
            !calendar
                .is_open(ct(sunday, 16, 59))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the queue must not print a trade"
        );
        assert!(
            calendar
                .is_open(ct(sunday, 17, 0))
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Sunday session opens at 17:00 CT"
        );

        // The row's instants are the ones the operator prints, not the module's
        // own: the block states a 16:00 CT queue, so the family's separately
        // sourced 16:15 or 16:45 variant is not what answers here.
        assert_eq!(
            calendar
                .holiday_on(day(trade_date))
                .map(Holiday::document_id),
            Some(match trade_date {
                (2026, 6, 22) => "CME-SVC-2026-06-18",
                (2026, 7, 6) => "CME-SVC-2026-07-03",
                _ => "CME-SVC-2027-06-17",
            }),
            "{trade_date:?}: the row cites the window its instants were read from"
        );
        assert_eq!(
            calendar.holiday_on(day(trade_date)).map(Holiday::tier),
            Some(EvidenceTier::T2),
            "{trade_date:?}: the row's tier"
        );
    }
}

/// A Saturday-session row's Sunday legs must be sourced from a window that
/// actually prints them.
///
/// Two of these rows span **two** windows: `CME-SVC-2026-06-18` and
/// `CME-SVC-2027-06-17` each stop at their own Saturday and print no Sunday
/// entry at all, so the Sunday Pre-Open and the Sunday-Monday session come from
/// the window that starts on that Sunday, `CME-SVC-2026-06-21` and
/// `CME-SVC-2027-06-20`. The third row's Saturday window, `CME-SVC-2026-07-03`,
/// happens to run through its Sunday and does print the legs, so it needs only
/// the one.
///
/// This fence exists because an independent review found the two-window rows
/// claiming a Sunday pair their cited artifact does not contain. It pins the
/// distinction the per-row citation check cannot see: a row may name a document
/// that resolves to real bytes and still quote from a window those bytes are
/// not.
#[test]
fn a_saturday_rows_sunday_legs_are_sourced_from_a_window_that_prints_them() {
    // (trade date, Saturday window, the window that actually prints the Sunday legs)
    for (trade_date, saturday_window, sunday_window) in [
        ("2026-06-22", "CME-SVC-2026-06-18", "CME-SVC-2026-06-21"),
        ("2026-07-06", "CME-SVC-2026-07-03", "CME-SVC-2026-07-03"),
        ("2027-06-21", "CME-SVC-2027-06-17", "CME-SVC-2027-06-20"),
    ] {
        let evidence = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/evidence/globex_energy.md"),
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
            row.contains(&format!("`{saturday_window}`")),
            "{trade_date}: the row must cite the window that carries the Saturday session"
        );
        assert!(
            row.contains(&format!("`{sunday_window}`")),
            "{trade_date}: the row must name {sunday_window}, the window that prints \
             the Sunday legs"
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
    let calendar = calendar();
    let holiday = calendar
        .holiday_on(day((2025, 11, 28)))
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
            .is_open(ct((2025, 11, 28), 7, 15))
            .expect("the coverage contract must answer a covered date"),
        "07:15 CT is the operator's Pre-Open and must not report matching"
    );
    assert!(
        calendar
            .is_open(ct((2025, 11, 28), 7, 45))
            .expect("the coverage contract must answer a covered date"),
        "07:45 CT is inside continuous trading and must report matching"
    );
    // The trade date does not move: the day still carries 2025-11-28.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), 6, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), 10, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day((2025, 11, 28)))
    );

    // The 2026 and 2027 Thanksgiving Fridays publish the final close alone, so
    // the same morning must still report matching there.
    for (year, month, date) in [(2026, 11, 27), (2027, 11, 26)] {
        assert!(
            calendar
                .is_open(ct((year, month, date), 7, 15))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} publishes no Pre-Open and must stay open at 07:15 CT"
        );
    }
}
