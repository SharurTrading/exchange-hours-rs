// SPDX-License-Identifier: MIT-0

//! `globex_cryptocurrency` holiday-table contracts, over the public surface.
//!
//! Every probe below is stated in America/Chicago wall clock — the zone CME
//! publishes in and the zone the rows are keyed in — and converted here, so a
//! daylight-saving slip cannot hide behind a hard-coded UTC literal.
//!
//! The family spans both holiday regimes the design memo names, and the two
//! answer differently on purpose:
//!
//! - the five-day 17:00-16:00 CT era closes over the weekend, so a `Closed`
//!   trade date deletes its whole trading day, wrap included;
//! - the 24/7 era from trade date 2026-05-30 assigns a block to the following
//!   open business date, so a `Closed` trade date is *skipped by that roll* and
//!   trading continues through the holiday under the next business date.
//!
//! Both are asserted. If the table and the business-date roll were wired up in
//! the wrong order the 24/7 family would go dark on every holiday, which is the
//! single highest-risk interaction in the whole feature.

use chrono::{DateTime, Datelike as _, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, ExceptionBlock, ExceptionBlockKind, ExchangeCalendar,
    Holiday, HolidayKind, MarketHoursKey, SUPPORT_FLOOR, SessionKind,
    calendar_for_market_hours_key,
};

/// The family under test, as a date-aware calendar.
fn crypto() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexCryptocurrency)
}

/// Asserts a date-aware query refused with `BeforeSupportFloor`, the verdict the
/// permanent 2025 floor gives a date below it (LAW-COVERAGE). The error must
/// name a venue-local day that really is below the floor, so a refusal for an
/// unrelated day or reason fails here, and an `Ok` answer fails the
/// `expect_err` above rather than being tolerated.
fn assert_before_floor<T: core::fmt::Debug>(result: Result<T, CalendarQueryError>, claim: &str) {
    let error = result.expect_err(claim);
    assert!(
        matches!(
            error,
            CalendarQueryError::BeforeSupportFloor { date, .. } if date < SUPPORT_FLOOR
        ),
        "{claim}: a date below the 2025 floor must refuse with BeforeSupportFloor; got {error}"
    );
}

/// Asserts a date-aware query refused with `OutsideCoveredRange`: the date's own
/// unsourced span, its missing holiday-layer answer, or a declared phase-level
/// gap (LAW-COVERAGE), on a date **at or above** the support floor.
///
/// The floor is checked first by every entry point, including the order-entry
/// scans: a pre-floor date reports `BeforeSupportFloor` and never the phase
/// verdict, because no range has been claimed below the floor for a phase gap to
/// be outside of. Use `assert_before_floor` for those probes.
fn assert_outside_range<T: core::fmt::Debug>(result: Result<T, CalendarQueryError>, claim: &str) {
    let error = result.expect_err(claim);
    assert!(
        matches!(error, CalendarQueryError::OutsideCoveredRange { .. }),
        "{claim}: the date is outside this identity's covered ranges, so the query must \
         refuse it with OutsideCoveredRange; got {error}"
    );
}

/// An America/Chicago wall clock, converted to the UTC the surface takes.
fn ct(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("fixture must name an unambiguous Central wall clock")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

fn kind_on(date: NaiveDate) -> Option<HolidayKind> {
    crypto().holiday_on(date).map(Holiday::kind)
}

// ---------------------------------------------------------------------------
// 1. A closed day.
// ---------------------------------------------------------------------------

/// Christmas 2025 falls in the five-day era, where a closed trade date removes
/// the complete trading day — including the Wednesday-evening block that would
/// have fed it.
///
/// The civil-day half of that claim still answers (`is_open` is false on the
/// whole day, and on the eve's leg). The trade-date half does not: resolving an
/// instant on the closed day reads a phase the family declares it cannot state,
/// so Stage 2B refuses it as `OutsideCoveredRange` rather than answering "no
/// trade date".
#[test]
fn a_closed_trade_date_has_no_session_and_no_trade_date() {
    let calendar = crypto();

    assert_eq!(kind_on(day(2025, 12, 25)), Some(HolidayKind::Closed));
    assert!(
        calendar
            .is_closed_trade_date(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );

    for probe in [
        ct(2025, 12, 25, 0, 30),
        ct(2025, 12, 25, 9, 0),
        ct(2025, 12, 25, 15, 0),
    ] {
        assert!(
            !calendar
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            "2025-12-25 must be shut at {probe}"
        );
        // The trade-date half of this test's name is no longer claimable:
        // resolving an instant on this closed day reads an order-entry phase
        // the family declares it cannot state (#93/#123), so the query refuses
        // the date rather than reporting "no trade date".
        assert_outside_range(
            calendar.trade_date(probe),
            "2025-12-25's trade date is refused, not stated as absent",
        );
    }

    // The eve's own 17:00 CT open would carry trade date 2025-12-25, so the
    // closure deletes it rather than leaving an orphaned evening leg. This
    // one still answers — the deletion is a fact about the shipped row, not
    // about the phase-gapped order-entry window.
    assert!(
        !calendar
            .is_open(ct(2025, 12, 24, 17, 30))
            .expect("the coverage contract must answer a covered date"),
        "the eve's evening leg belongs to the closed trade date"
    );
}

/// The five-day era's `[N3]` holidays are a trade-date merge, not a closure:
/// CME published a 16:00 CT pre-open in place of the 16:00 CT final close, with
/// no `[N6]` predecessor on the preceding evening, so matching ran from Sunday
/// 17:00 CT through to 16:00 CT as on a normal Monday and only the trade-date
/// label moved.
///
/// The holiday itself still carries no row — the operator assigns the span the
/// following business day's trade date — but the crate no longer answers its
/// own date for it: the merged span ships as a `ReplacementBlocks` row keyed to
/// 2025-01-21, so the Sunday evening, the holiday and the Tuesday close all
/// carry the operator's own label.
#[test]
fn a_trade_date_merge_keeps_the_sunday_evening_block() {
    let calendar = crypto();

    assert_eq!(kind_on(day(2025, 1, 20)), None);
    for probe in [
        ct(2025, 1, 19, 18, 0),
        ct(2025, 1, 20, 9, 0),
        ct(2025, 1, 20, 15, 0),
    ] {
        assert!(
            calendar
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            "2025-01-20 matched at {probe}"
        );
        assert_eq!(
            calendar
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
            Some(day(2025, 1, 21))
        );
    }
    assert!(
        !calendar
            .is_open(ct(2025, 1, 20, 16, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2025, 1, 20, 17, 30))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 1, 20, 17, 30))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 1, 21))
    );
}

// ---------------------------------------------------------------------------
// 2 and 3. An early close, on each side of the cutoff.
// ---------------------------------------------------------------------------

/// Independence Day 2025: the session opened Thursday at 17:00 CT and the
/// early close is stated on Friday's trade date, so the clip lands on the
/// correct civil day.
#[test]
fn an_early_close_ends_a_day_that_opened_the_previous_evening() {
    let calendar = crypto();
    let cutoff = ct(2025, 7, 4, 12, 0);

    assert_eq!(
        kind_on(day(2025, 7, 4)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600
        })
    );

    assert!(
        calendar
            .is_open(cutoff - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );

    let (open, close) = calendar
        .session_bounds(ct(2025, 7, 4, 9, 0))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(open, ct(2025, 7, 3, 17, 0));
    assert_eq!(close, cutoff);
    assert_eq!(
        calendar
            .candle_end(ct(2025, 7, 4, 9, 0), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(cutoff)
    );

    // Nothing re-opens behind the cutoff on that civil day.
    assert!(
        !calendar
            .is_open(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2025, 7, 4, 18, 0))
            .expect("the coverage contract must answer a covered date")
    );
}

/// The day after Thanksgiving 2025 stops at 13:45 CT, inside a trade date the
/// Thursday holiday merged into it: the span opens on Wednesday evening, pauses
/// at 07:00 CT and reopens at 07:30 CT before the final close.
#[test]
fn an_early_close_stands_without_a_closed_neighbour() {
    let calendar = crypto();
    let cutoff = ct(2025, 11, 28, 13, 45);

    assert_eq!(kind_on(day(2025, 11, 27)), None);
    assert!(
        matches!(
            kind_on(day(2025, 11, 28)),
            Some(HolidayKind::ReplacementBlocks(_))
        ),
        "2025-11-28 states its complete trading day"
    );

    assert!(
        calendar
            .is_open(cutoff - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(cutoff)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(cutoff - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    // Thursday's 17:00 CT open feeds Friday's trade date, so the Thanksgiving
    // merge must not take it: it is the `-1` matching block's own opening.
    assert!(
        calendar
            .is_open(ct(2025, 11, 27, 18, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 11, 27, 18, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
}

// ---------------------------------------------------------------------------
// 4. A late open.
// ---------------------------------------------------------------------------

/// This family ships no late open, in either era, and that is a claim rather
/// than an omission.
///
/// CME never re-opens cryptocurrency later than its normal first open on a
/// holiday: what a holiday removes is the trade date, not the start of trading.
/// Asserting the absence keeps the claim honest — the moment a later wave keys
/// a `LateOpen` row here this fails, which is what forces the two-branch
/// cutoff test the design memo requires for a late open.
#[test]
fn the_family_keys_no_late_open_row() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a holiday table");

    let mut date = coverage.first();
    while date <= coverage.last() {
        if let Some(holiday) = calendar.holiday_on(date) {
            // `Unsourced` joined the vocabulary with the 2022-2024 wave: it
            // states that a date inside a window was audited and clips nothing.
            // `ReplacementBlocks` joined it for the merged trade dates and the
            // special sessions, which state a day's blocks rather than a scalar
            // boundary. The claim this test fences — no `LateOpen` anywhere — is
            // unchanged and is asserted again explicitly below.
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed
                        | HolidayKind::EarlyClose { .. }
                        | HolidayKind::ReplacementBlocks(_)
                        | HolidayKind::Unsourced
                ),
                "{date} ships {:?}; a late open needs its own two-branch cutoff test",
                holiday.kind()
            );
            assert!(
                !matches!(
                    holiday.kind(),
                    HolidayKind::LateOpen { .. } | HolidayKind::LateOpenAndEarlyClose { .. }
                ),
                "{date} ships {:?}; a late open needs its own two-branch cutoff test",
                holiday.kind()
            );
        }
        date = date
            .succ_opt()
            .expect("the coverage window is representable");
    }
}

// ---------------------------------------------------------------------------
// 5. A wrap removed by a closure.
// ---------------------------------------------------------------------------

/// Christmas Eve 2025 closes at 12:45 CT and does not re-open, because the
/// block that would have opened at 17:00 CT carries the closed Christmas trade
/// date. The next session is Christmas evening, feeding Boxing Day.
#[test]
fn a_closure_removes_the_previous_evenings_wrap() {
    let calendar = crypto();

    assert_eq!(
        kind_on(day(2025, 12, 24)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60
        })
    );

    assert!(
        calendar
            .is_open(ct(2025, 12, 24, 12, 44))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2025, 12, 24, 12, 45))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2025, 12, 24, 17, 30))
            .expect("the coverage contract must answer a covered date")
    );

    let (open, _close) = calendar
        .next_session_after(ct(2025, 12, 24, 12, 20))
        .expect("the coverage contract must answer a covered date")
        .expect("a covered date must resolve the queried value");
    assert_eq!(open, ct(2025, 12, 25, 17, 0));
}

// ---------------------------------------------------------------------------
// 6. The trade-date consequence, in both eras.
// ---------------------------------------------------------------------------

/// Five-day era: a shortened day keeps its own trade date, and the holiday
/// evening's block already belongs to the following date.
#[test]
fn the_five_day_era_keeps_the_holidays_own_trade_dates() {
    let calendar = crypto();

    assert_eq!(
        calendar
            .trade_date(ct(2025, 7, 4, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 7, 4))
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 12, 23, 18, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 24))
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 12, 25, 17, 30))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );
}

// ---------------------------------------------------------------------------
// 6b. The nine merged trade dates CME publishes for the five-day era.
// ---------------------------------------------------------------------------

/// One merged-date fixture: the span's opening day (`-2`), the holiday whose own
/// trade date disappears (`-1`), and the trade date the operator assigns it.
type MergedDates = ((i32, u32, u32), (i32, u32, u32), (i32, u32, u32));

/// The nine merged trade dates of the five-day era, with the span's eve and the
/// holiday between them.
///
/// The operator's own service prints every event of the span with the following
/// business day's `tradingDate`, so the crate answers that date and not the
/// holiday's. Eight spans open on a Sunday evening; the Juneteenth one opens on
/// a Wednesday, because CME observed that holiday on the Thursday.
const MERGED_TRADE_DATES: [MergedDates; 9] = [
    ((2025, 1, 19), (2025, 1, 20), (2025, 1, 21)),
    ((2025, 2, 16), (2025, 2, 17), (2025, 2, 18)),
    ((2025, 5, 25), (2025, 5, 26), (2025, 5, 27)),
    ((2025, 6, 18), (2025, 6, 19), (2025, 6, 20)),
    ((2025, 8, 31), (2025, 9, 1), (2025, 9, 2)),
    ((2025, 11, 26), (2025, 11, 27), (2025, 11, 28)),
    ((2026, 1, 18), (2026, 1, 19), (2026, 1, 20)),
    ((2026, 2, 15), (2026, 2, 16), (2026, 2, 17)),
    ((2026, 5, 24), (2026, 5, 25), (2026, 5, 26)),
];

/// The replacement-block set the row keyed to `trade_date` declares, as
/// `(kind, opening day offset, open, close)` tuples.
#[expect(
    clippy::panic,
    reason = "a fixture row that is missing or of the wrong kind must fail loudly, \
              naming the value it found"
)]
fn declared_blocks(trade_date: (i32, u32, u32)) -> Vec<(ExceptionBlockKind, i8, u32, u32)> {
    let date = day(trade_date.0, trade_date.1, trade_date.2);
    let blocks = match crypto().holiday_on(date).map(Holiday::kind) {
        Some(HolidayKind::ReplacementBlocks(blocks)) => blocks,
        other => panic!("{trade_date:?} must carry a replacement row, got {other:?}"),
    };
    blocks
        .iter()
        .map(|block: &ExceptionBlock| {
            (
                block.kind(),
                block.open_day_offset(),
                block.open_ssm(),
                block.close_ssm(),
            )
        })
        .collect()
}

/// Every one of the nine rows ships the operator's complete day, block for
/// block, and each block set ends at the row's own close.
///
/// The block tuples are written out rather than derived from the module: the
/// fence is compared against the captured service bytes, so a row whose queue,
/// opening instant or close moves fails here even though `is_open` on the
/// unmutated minutes would still agree.
#[test]
fn the_nine_merged_rows_state_the_operators_own_blocks() {
    const QUEUE_16: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::OrderEntry, -2, 16 * 3_600, 17 * 3_600);
    const SESSION_16: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::Extended, -2, 17 * 3_600, 16 * 3_600);
    const HOLIDAY_QUEUE: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::OrderEntry, -1, 16 * 3_600, 17 * 3_600);
    const HOLIDAY_SESSION: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::Extended, -1, 17 * 3_600, 16 * 3_600);
    const QUEUE_1645: (ExceptionBlockKind, i8, u32, u32) = (
        ExceptionBlockKind::OrderEntry,
        -2,
        16 * 3_600 + 45 * 60,
        17 * 3_600,
    );

    let mut checked = 0_usize;
    for (eve, _holiday, trade_date) in MERGED_TRADE_DATES {
        // The Juneteenth span opens on a Wednesday, whose queue the operator
        // prints at the family's ordinary weekday 16:45 CT; every other span
        // opens on the Sunday this family's week begins, at 16:00 CT.
        let weekday_open = eve == (2025, 6, 18);
        let expected = if trade_date == (2025, 11, 28) {
            // The finalised publication adds the 07:00 preopen; 07:30 open
            // pause, so this day is six blocks and its `-1` leg stops at 07:00.
            vec![
                QUEUE_1645,
                SESSION_16,
                HOLIDAY_QUEUE,
                (ExceptionBlockKind::Extended, -1, 17 * 3_600, 7 * 3_600),
                (
                    ExceptionBlockKind::OrderEntry,
                    0,
                    7 * 3_600,
                    7 * 3_600 + 30 * 60,
                ),
                (
                    ExceptionBlockKind::Extended,
                    0,
                    7 * 3_600 + 30 * 60,
                    13 * 3_600 + 45 * 60,
                ),
            ]
        } else if weekday_open {
            vec![QUEUE_1645, SESSION_16, HOLIDAY_QUEUE, HOLIDAY_SESSION]
        } else {
            vec![QUEUE_16, SESSION_16, HOLIDAY_QUEUE, HOLIDAY_SESSION]
        };
        assert_eq!(
            declared_blocks(trade_date),
            expected,
            "{trade_date:?}: the row must state the operator's own blocks"
        );
        assert!(
            declared_blocks(trade_date)
                .windows(2)
                .all(|pair| (pair[0].1, pair[0].2) <= (pair[1].1, pair[1].2)),
            "{trade_date:?}: the row's blocks are ordered by opening day then open time"
        );
        checked += 1;
    }
    assert_eq!(checked, 9, "the five-day era's merged trade dates");
}

/// The evening legs of a merged span carry the operator's trade date, not the
/// holiday's.
///
/// This is the acceptance question the captured bytes answer: on 2025-01-19 the
/// service prints `17:00 open` with `tradingDate 2025-01-21`, and the crate once
/// answered 2025-01-20 for it. Both the eve's evening open and the holiday's own
/// evening open are probed, on both sides of the merge.
#[test]
fn the_merged_evening_legs_carry_the_operators_trade_date() {
    let calendar = crypto();
    for (eve, holiday, trade_date) in MERGED_TRADE_DATES {
        let merged = day(trade_date.0, trade_date.1, trade_date.2);
        for probe in [eve, holiday] {
            let instant = ct(probe.0, probe.1, probe.2, 18, 0);
            assert!(
                calendar
                    .is_open(instant)
                    .expect("the coverage contract must answer a covered date"),
                "{probe:?} 18:00 CT is inside the merged span and is traded"
            );
            assert_eq!(
                calendar
                    .trade_date(instant)
                    .expect("the coverage contract must answer a covered date"),
                Some(merged),
                "{probe:?} 18:00 CT must carry the operator's own trade date"
            );
        }
        // The holiday's morning is inside the span the operator ran, under the
        // same label.
        assert_eq!(
            calendar
                .trade_date(ct(holiday.0, holiday.1, holiday.2, 10, 0))
                .expect("the coverage contract must answer a covered date"),
            Some(merged),
            "{holiday:?} 10:00 CT is inside the merged span"
        );
    }
}

/// The span's own close is the row's, and the last matching second before it
/// carries the row's trade date.
///
/// A merge relabels the span; it must not swallow the ordinary week that resumes
/// behind the close.
#[test]
fn the_merged_spans_close_on_their_own_trade_date() {
    let calendar = crypto();
    for (_eve, _holiday, trade_date) in MERGED_TRADE_DATES {
        let merged = day(trade_date.0, trade_date.1, trade_date.2);
        let (close_hour, close_minute) = if trade_date == (2025, 11, 28) {
            (13, 45)
        } else {
            (16, 0)
        };
        let close = ct(
            trade_date.0,
            trade_date.1,
            trade_date.2,
            close_hour,
            close_minute,
        );
        assert_eq!(
            calendar
                .trade_date(close - Duration::seconds(1))
                .expect("the coverage contract must answer a covered date"),
            Some(merged),
            "{trade_date:?}: the final matching second carries the row's trade date"
        );
        assert!(
            calendar
                .is_open(close - Duration::seconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{trade_date:?}: the final matching second is open"
        );
        assert!(
            !calendar
                .is_open(close)
                .expect("the coverage contract must answer a covered date"),
            "{trade_date:?}: the close is end-exclusive"
        );
    }
}

/// The day after Thanksgiving 2025 pauses between 07:00 and 07:30 CT.
///
/// The pre-holiday capture of the service prints only the 13:45 CT close, and a
/// four-block row built from it would keep serving 07:00-07:30 CT as matching.
/// The finalised publication adds `07:00 preopen; 07:30 open`, which is the
/// six-block row this test fences at both of the pause's edges.
#[test]
fn the_thanksgiving_merge_pauses_before_its_early_close() {
    let calendar = crypto();
    let trade_date = day(2025, 11, 28);

    assert!(
        matches!(kind_on(trade_date), Some(HolidayKind::ReplacementBlocks(_))),
        "2025-11-28 states its complete trading day, not a scalar early close"
    );

    // The `-1` session runs from the Wednesday evening leg to the pause.
    for (hour, minute) in [(0, 1), (6, 59)] {
        assert!(
            calendar
                .is_open(ct(2025, 11, 28, hour, minute))
                .expect("the coverage contract must answer a covered date"),
            "2025-11-28 {hour:02}:{minute:02} CT is inside the overnight session"
        );
    }
    for (hour, minute) in [(7, 0), (7, 15), (7, 29)] {
        assert!(
            !calendar
                .is_open(ct(2025, 11, 28, hour, minute))
                .expect("the coverage contract must answer a covered date"),
            "2025-11-28 {hour:02}:{minute:02} CT is inside the published pause"
        );
    }
    for (hour, minute) in [(7, 30), (9, 0), (13, 44)] {
        let instant = ct(2025, 11, 28, hour, minute);
        assert!(
            calendar
                .is_open(instant)
                .expect("the coverage contract must answer a covered date"),
            "2025-11-28 {hour:02}:{minute:02} CT is inside the final session"
        );
        assert_eq!(
            calendar
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(trade_date),
            "the reopened session still carries 2025-11-28"
        );
    }
    assert!(
        !calendar
            .is_open(ct(2025, 11, 28, 13, 45))
            .expect("the coverage contract must answer a covered date")
    );

    // The Wednesday-evening leg that opens the span, and its queue.
    assert!(
        calendar
            .is_open(ct(2025, 11, 26, 17, 1))
            .expect("the coverage contract must answer a covered date"),
        "the 2025-11-26 17:00 CT leg opens the merged span"
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 11, 26, 17, 1))
            .expect("the coverage contract must answer a covered date"),
        Some(trade_date)
    );
}

/// The 2025 rows keep a strictly ascending, duplicate-free table, and the nine
/// merged dates are rows on it.
///
/// `holidays!` proves the order at build time; this reads it back through the
/// public surface, because the change replaced an existing 2025-11-28 row and a
/// second row on that date would have made one of the two unreachable.
#[test]
fn the_merged_rows_join_a_strictly_ascending_table() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a holiday table");

    let mut previous: Option<NaiveDate> = None;
    let mut date = coverage.first();
    while date <= coverage.last() {
        if calendar.holiday_on(date).is_some() {
            assert!(
                previous.is_none_or(|earlier| earlier < date),
                "{date} repeats or precedes the row before it"
            );
            previous = Some(date);
        }
        date = date.succ_opt().expect("the window is representable");
    }

    for (_eve, holiday, trade_date) in MERGED_TRADE_DATES {
        let merged = day(trade_date.0, trade_date.1, trade_date.2);
        assert!(
            matches!(kind_on(merged), Some(HolidayKind::ReplacementBlocks(_))),
            "{merged} ships the merged span"
        );
        assert_eq!(
            kind_on(day(holiday.0, holiday.1, holiday.2)),
            None,
            "{holiday:?} carries no row of its own: the operator gives it no trade date"
        );
    }
}

/// 24/7 era: the row deletes no trading. It makes the business-date roll skip
/// the holiday, so `is_open` stays true across the whole civil day while the
/// trade date moves to the next open business date — a Monday holiday to the
/// Tuesday, a Thursday holiday to the Friday, and a Friday holiday across the
/// weekend to the Monday.
#[test]
fn the_twenty_four_seven_era_rolls_the_trade_date_without_deleting_a_day() {
    let calendar = crypto();

    for (probe, rolled_to) in [
        // Labor Day 2026, a Monday.
        (ct(2026, 9, 7, 9, 0), day(2026, 9, 8)),
        // Thanksgiving 2026, a Thursday.
        (ct(2026, 11, 26, 9, 0), day(2026, 11, 27)),
        // Christmas 2026, a Friday: the roll crosses the weekend.
        (ct(2026, 12, 25, 9, 0), day(2026, 12, 28)),
        // Globex's closed Friday before Christmas 2027.
        (ct(2027, 12, 24, 9, 0), day(2027, 12, 27)),
    ] {
        assert!(
            calendar
                .is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            "24/7 cryptocurrency must keep trading at {probe}"
        );
        assert_eq!(
            calendar
                .trade_date(probe)
                .expect("the coverage contract must answer a covered date"),
            Some(rolled_to),
            "the business-date roll must skip the closed trade date at {probe}"
        );
    }

    for closed in [
        day(2026, 9, 7),
        day(2026, 11, 26),
        day(2026, 12, 25),
        day(2027, 12, 24),
    ] {
        assert_eq!(kind_on(closed), Some(HolidayKind::Closed));
        assert!(
            calendar
                .is_closed_trade_date(closed, SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
        // The charter's own surprise: a full closure is not a closed civil day
        // for this family, because trading never stops.
        assert!(
            !calendar
                .is_closed_all_day_on(closed, SessionKind::Both)
                .expect("the coverage contract must answer a covered date")
        );
    }

    // The Thursday before the closed 2027-12-24 settles its own trade date.
    assert_eq!(
        calendar
            .trade_date(ct(2027, 12, 23, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2027, 12, 23))
    );
}

// ---------------------------------------------------------------------------
// 6c. The eight 24/7-era merged trade dates CME publishes.
// ---------------------------------------------------------------------------

/// One 24/7-era merged-date fixture: the holiday whose own trade date
/// disappears, the trade date the operator assigns its merged span, and the
/// pre-holiday weekday that span opens on.
type Merged24x7Dates = ((i32, u32, u32), (i32, u32, u32), (i32, u32, u32));

/// The eight 24/7-era merged trade dates, with the holiday between them and the
/// weekday the merged span opens on.
///
/// Six Monday holidays roll the weekend block forward, so their spans open on
/// the **pre-holiday Friday**; the two Thursday holidays open on the Wednesday.
const MERGED_24X7_TRADE_DATES: [Merged24x7Dates; 8] = [
    ((2026, 9, 4), (2026, 9, 7), (2026, 9, 8)),
    ((2026, 11, 25), (2026, 11, 26), (2026, 11, 27)),
    ((2027, 1, 15), (2027, 1, 18), (2027, 1, 19)),
    ((2027, 2, 12), (2027, 2, 15), (2027, 2, 16)),
    ((2027, 5, 28), (2027, 5, 31), (2027, 6, 1)),
    ((2027, 7, 2), (2027, 7, 5), (2027, 7, 6)),
    ((2027, 9, 3), (2027, 9, 6), (2027, 9, 7)),
    ((2027, 11, 24), (2027, 11, 25), (2027, 11, 26)),
];

/// Every one of the eight rows ships the complete merged day, block for block,
/// in two shapes.
///
/// A Monday holiday's merged day opens on the pre-holiday **Friday** 16:02 CT
/// and runs to the Tuesday 16:00 CT close, so its blocks sit at offsets `-4`
/// through `0`; a Thursday holiday's opens on the **Wednesday** 16:02 CT and
/// runs to the Friday 16:00 CT close, at `-2` through `0`. The tuples are
/// written out rather than derived from the module: they are compared against
/// the captured service bytes, so a row whose queue, opening instant or close
/// moves fails here even though `is_open` on the unmutated minutes would still
/// agree.
#[test]
fn the_eight_24x7_merged_rows_state_the_operators_own_blocks() {
    const MIDNIGHT: u32 = 0;
    const SATURDAY_QUEUE: (ExceptionBlockKind, i8, u32, u32) = (
        ExceptionBlockKind::OrderEntry,
        -3,
        3 * 3_600 + 45 * 60,
        4 * 3_600,
    );
    const SATURDAY_SESSION: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::Extended, -3, 4 * 3_600, 24 * 3_600);
    const SUNDAY_SESSION: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::Extended, -2, MIDNIGHT, 24 * 3_600);
    const HOLIDAY_SESSION: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::Extended, -1, MIDNIGHT, 16 * 3_600 + 60);
    const HOLIDAY_QUEUE: (ExceptionBlockKind, i8, u32, u32) = (
        ExceptionBlockKind::OrderEntry,
        -1,
        16 * 3_600 + 60,
        16 * 3_600 + 120,
    );
    const HOLIDAY_EVENING: (ExceptionBlockKind, i8, u32, u32) = (
        ExceptionBlockKind::Extended,
        -1,
        16 * 3_600 + 120,
        24 * 3_600,
    );
    const TRADE_DATE_SESSION: (ExceptionBlockKind, i8, u32, u32) =
        (ExceptionBlockKind::Extended, 0, MIDNIGHT, 16 * 3_600);
    // The merged day's own opening blocks: the pre-holiday Friday's queue and
    // evening session for a Monday holiday, the Wednesday's for a Thursday one.
    let opening = |offset: i8| {
        [
            (
                ExceptionBlockKind::OrderEntry,
                offset,
                16 * 3_600 + 60,
                16 * 3_600 + 120,
            ),
            (
                ExceptionBlockKind::Extended,
                offset,
                16 * 3_600 + 120,
                24 * 3_600,
            ),
        ]
    };

    let mut checked = 0_usize;
    for (eve, holiday, trade_date) in MERGED_24X7_TRADE_DATES {
        let monday_holiday = day(holiday.0, holiday.1, holiday.2).weekday() == Weekday::Mon;
        let mut expected = Vec::new();
        if monday_holiday {
            // Friday 16:01/16:02 and Friday evening, then the Saturday
            // maintenance window with its queue and the Sunday session.
            expected.extend(opening(-4));
            expected.push((ExceptionBlockKind::Extended, -3, MIDNIGHT, 2 * 3_600));
            expected.push(SATURDAY_QUEUE);
            expected.push(SATURDAY_SESSION);
            expected.push(SUNDAY_SESSION);
        } else {
            expected.extend(opening(-2));
        }
        expected.push(HOLIDAY_SESSION);
        expected.push(HOLIDAY_QUEUE);
        expected.push(HOLIDAY_EVENING);
        expected.push(TRADE_DATE_SESSION);

        assert_eq!(
            declared_blocks(trade_date),
            expected,
            "{trade_date:?} (holiday {holiday:?}, opening {eve:?}): the row must state \
             the operator's own complete merged day"
        );
        assert!(
            declared_blocks(trade_date)
                .windows(2)
                .all(|pair| (pair[0].1, pair[0].2) <= (pair[1].1, pair[1].2)),
            "{trade_date:?}: the row's blocks are ordered by opening day then open time"
        );
        checked += 1;
    }
    assert_eq!(checked, 8, "the 24/7 era's merged trade dates");
}

/// The 24/7-era holiday's own 16:00-16:01 CT minute is traded, because CME
/// prints no `closed` event at 16:00 on those dates.
///
/// This is the acceptance question the captured bytes answer. The operator's
/// service prints `16:01 preopen; 16:02 open` and no `16:00 closed` on the eight
/// 24/7-era Monday and Thursday holidays, while printing the close on every
/// ordinary weekday of the reference week and on all seven of the era's Friday
/// holidays — so matching ran across the minute the ordinary week stops at, and
/// `is_open` must answer true there. The 16:01-16:02 CT Pre-Open stays
/// `order_entry`: it accepts orders and is not a session. The following trade
/// date keeps the ordinary window unchanged, so the fix widens by one minute and
/// no more.
#[test]
fn the_24x7_holidays_sixteen_hundred_minute_is_traded() {
    let calendar = crypto();
    let mut checked = 0_usize;

    for (_eve, holiday, trade_date) in MERGED_24X7_TRADE_DATES {
        let holiday_day = day(holiday.0, holiday.1, holiday.2);
        let trade_day = day(trade_date.0, trade_date.1, trade_date.2);
        assert_eq!(
            kind_on(holiday_day),
            Some(HolidayKind::Closed),
            "{holiday_day}: the holiday itself still keys the Closed row the roll reads"
        );
        match crypto().holiday_on(trade_day).map(Holiday::kind) {
            Some(HolidayKind::ReplacementBlocks(_)) => {}
            other => panic!("{trade_day} must carry a replacement row, got {other:?}"),
        }

        let sixteen = ct(holiday.0, holiday.1, holiday.2, 16, 0);
        // The whole minute the ordinary week breaks at, both ends included.
        for probe in [sixteen, sixteen + Duration::seconds(59)] {
            assert!(
                calendar
                    .is_open(probe)
                    .expect("the coverage contract must answer a covered date"),
                "the operator printed no 16:00 closed on {holiday_day}, so matching runs \
                 at {probe}"
            );
            assert_eq!(
                calendar
                    .trade_date(probe)
                    .expect("the coverage contract must answer a covered date"),
                Some(trade_day),
                "the minute carries the merged span's trade date at {probe}"
            );
        }

        // The Pre-Open queue is the operator's `16:01 preopen`: order entry, no
        // matching.
        let queue = ct(holiday.0, holiday.1, holiday.2, 16, 1);
        assert!(
            !calendar
                .is_open(queue)
                .expect("the coverage contract must answer a covered date"),
            "the 16:01-16:02 CT Pre-Open is an order-entry window at {queue}"
        );
        assert!(
            calendar
                .is_accepting_orders(queue)
                .expect("the coverage contract must answer a covered date"),
            "the Pre-Open accepts orders at {queue}"
        );
        assert!(
            calendar
                .is_open(ct(holiday.0, holiday.1, holiday.2, 16, 2))
                .expect("the coverage contract must answer a covered date"),
            "matching resumes at the operator's 16:02 open on {holiday_day}"
        );

        // Unchanged: the trade date's own 16:00 CT maintenance window, and the
        // instant before it.
        assert!(
            calendar
                .is_open(ct(trade_date.0, trade_date.1, trade_date.2, 15, 59))
                .expect("the coverage contract must answer a covered date"),
            "{trade_day} trades up to its own 16:00 CT close"
        );
        let close = ct(trade_date.0, trade_date.1, trade_date.2, 16, 0);
        assert!(
            !calendar
                .is_open(close)
                .expect("the coverage contract must answer a covered date"),
            "the trade date's own 16:00 CT final close is end-exclusive at {close}"
        );
        checked += 1;
    }
    assert_eq!(checked, 8, "the 24/7 era's merged trade dates");
}

/// The instants around the fix that must **not** move.
///
/// An ordinary 24/7 weekday keeps the 16:00-16:02 CT window: no matching from
/// 16:00, the queue at 16:01, matching from 16:02. A 24/7-era **Friday**
/// holiday keeps its printed `16:00 closed` and so keeps the same window. And
/// the Saturday maintenance window keeps its queue. If the merged rows' blocks
/// were stated a minute wide, this is the fence that fails.
#[test]
fn the_ordinary_24x7_windows_are_unchanged() {
    let calendar = crypto();

    for (label, y, m, d) in [
        ("an ordinary Tuesday", 2026, 10, 20),
        ("Christmas Day 2026, a Friday holiday", 2026, 12, 25),
    ] {
        assert!(
            !calendar
                .is_open(ct(y, m, d, 16, 0))
                .expect("the coverage contract must answer a covered date"),
            "{label}: the operator prints a 16:00 closed, so matching stops"
        );
        assert!(
            !calendar
                .is_open(ct(y, m, d, 16, 0) + Duration::seconds(59))
                .expect("the coverage contract must answer a covered date"),
            "{label}: the 16:00-16:01 CT minute stays closed"
        );
        assert!(
            calendar
                .is_accepting_orders(ct(y, m, d, 16, 1))
                .expect("the coverage contract must answer a covered date"),
            "{label}: the 16:01-16:02 CT Pre-Open still accepts orders"
        );
        assert!(
            !calendar
                .is_open(ct(y, m, d, 16, 1))
                .expect("the coverage contract must answer a covered date"),
            "{label}: the Pre-Open is never a session"
        );
        assert!(
            calendar
                .is_open(ct(y, m, d, 16, 2))
                .expect("the coverage contract must answer a covered date"),
            "{label}: matching resumes at 16:02 CT"
        );
    }

    // The era's Friday holiday settles no trade date of its own: every event it
    // prints carries the following Monday, and the crate rolls it there.
    assert_eq!(
        calendar
            .trade_date(ct(2026, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 28))
    );

    // The Saturday maintenance window and its queue are untouched.
    assert!(
        !calendar
            .is_open(ct(2026, 6, 6, 2, 0))
            .expect("the coverage contract must answer a covered date"),
        "the Saturday 02:00-04:00 CT maintenance window is unchanged"
    );
    assert!(
        calendar
            .is_accepting_orders(ct(2026, 6, 6, 3, 45))
            .expect("the coverage contract must answer a covered date"),
        "the Saturday 03:45-04:00 CT Pre-Open is unchanged"
    );
    assert!(
        !calendar
            .is_open(ct(2026, 6, 6, 3, 45))
            .expect("the coverage contract must answer a covered date"),
        "the Saturday Pre-Open is never a session"
    );
    assert!(
        calendar
            .is_open(ct(2026, 6, 6, 4, 0))
            .expect("the coverage contract must answer a covered date"),
        "Saturday matching resumes at 04:00 CT"
    );
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// Inside the window a date with no row is audited normal; outside it the table
/// has no answer at all, and must not silently extend to a neighbouring year's
/// holidays.
///
/// The date-aware side of that edge is a refusal, not an answer: 2024-12-25
/// precedes the 2025 floor and 2028-01-01 lies outside every audited window, so
/// each earns its own variant — and the detached snapshot answers the normal
/// week only for the post-floor one.
#[test]
fn the_coverage_window_has_two_hard_edges() {
    let calendar = crypto();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a holiday table");

    // The window opens at the permanent 2025-01-01 floor: the pre-floor eras
    // left with Stage 5 of the release plan (#117), so the outer edges are
    // what this fence pins.
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2025, 1, 1)));
    assert!(coverage.contains(day(2027, 12, 31)));
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert_eq!(kind_on(day(2025, 1, 1)), Some(HolidayKind::Closed));
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    // Below the floor the `None` is silence, not an audited normal date —
    // and Christmas 2024, a row the module shipped until Stage 5, is gone.
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert_eq!(kind_on(day(2024, 12, 25)), None);

    // Christmas 2018 and Christmas 2024 are CME closures below the window that
    // no retained wave audits. What is gone is the old reading that the
    // identity then answers the pure normal-week result: the date precedes the
    // 2025 floor, so the identity and the detached snapshot both refuse it, and
    // the identity refuses the trade-date question with the same verdict.
    let bare = calendar.without_holidays();
    for probe in [ct(2024, 12, 25, 9, 0), ct(2024, 12, 25, 18, 0)] {
        assert_before_floor(
            calendar.is_open(probe),
            "a pre-floor date is refused, not answered from the normal week",
        );
        assert_before_floor(
            calendar.trade_date(probe),
            "a pre-floor trade date is refused",
        );
        assert_before_floor(
            bare.is_open(probe),
            "detaching the table does not lift the floor",
        );
    }
    assert_eq!(calendar.holiday_on(day(2024, 12, 25)), None);

    // One day past the window the table is equally silent, but 2028-01-01 is
    // above the floor: the identity refuses the date its holiday layer has no
    // answer for, while the detached snapshot — which claims no coverage —
    // still answers the normal week.
    for probe in [ct(2028, 1, 1, 9, 0), ct(2028, 1, 1, 20, 0)] {
        assert_outside_range(
            calendar.is_open(probe),
            "2028-01-01 is outside every audited window",
        );
        assert!(
            bare.is_open(probe)
                .expect("a detached snapshot claims no coverage"),
            "{probe}: the normal week trades once the table is detached"
        );
    }
}

// ---------------------------------------------------------------------------
// `without_holidays` restores the pre-table answer.
// ---------------------------------------------------------------------------

/// The consumer's escape hatch detaches the table outright: no rows, no
/// coverage, and every query answers as it did before the family shipped one.
#[test]
fn without_holidays_restores_the_normal_week_answer() {
    let bare = crypto().without_holidays();

    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2025, 12, 25)), None);
    assert_eq!(bare.holiday_on(day(2026, 12, 25)), None);

    // Five-day era: the deleted Christmas day comes back.
    assert!(
        !crypto()
            .is_open(ct(2025, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct(2025, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        bare.trade_date(ct(2025, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 25))
    );

    // Five-day era: the clipped Independence Day runs to its normal 16:00 CT.
    assert!(
        !crypto()
            .is_open(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        bare.session_bounds(ct(2025, 7, 4, 13, 0))
            .expect("the coverage contract must answer a covered date")
            .map(|(_open, close)| close),
        Some(ct(2025, 7, 4, 16, 0))
    );

    // 24/7 era: the business-date roll stops rolling.
    assert_eq!(
        crypto()
            .trade_date(ct(2026, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 28))
    );
    assert_eq!(
        bare.trade_date(ct(2026, 12, 25, 9, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 12, 25))
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
/// 2019-01-01 closure, the 2019-06-19 `Unsourced` row, the 2021-11-26 early
/// close and the 2022-04-15, 2023-12-25 and 2024-12-25 closures. `holiday_on`
/// answers `None` for all of them (the table has no answer below the floor),
/// the coverage no longer contains any of them, and the date-aware query
/// returns the explicit `BeforeSupportFloor` error (LAW-COVERAGE), never a
/// closure or a normal week read from the removed rows.
#[test]
fn pre_floor_rows_refuse_instead_of_answering() {
    let calendar = crypto();

    for (year, month, date) in [
        (2019, 1, 1),
        (2019, 6, 19),
        (2021, 11, 26),
        (2022, 4, 15),
        (2023, 12, 25),
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
        assert_before_floor(
            calendar.is_closed_trade_date(removed, SessionKind::Both),
            "the removed row's trade date is below the floor",
        );
        assert_before_floor(
            calendar.is_open(ct(year, month, date, 10, 0)),
            "the removed row's own civil day is below the floor",
        );
    }

    // The evening leg that opens on 2024-12-31 belongs to trade date
    // 2025-01-01, whose own `Closed` row the module still ships — in the
    // five-day era that row deletes the complete trading day of 2025-01-01
    // including that wrap, so no pre-floor row is load-bearing at or after the
    // floor. The 24/7-era `ReplacementBlocks` rows keyed to 2026-2027 merged
    // trade dates reach back only over evenings inside the retained window.
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
