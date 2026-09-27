// SPDX-License-Identifier: MIT-0

//! The built-in holiday table of `globex_interest_rates`, 2025-2027.
//!
//! Every probe is stated in `America/Chicago` wall clock — the zone CME states
//! its trading hours in — and converted to UTC by [`ct`], because that is the
//! only form in which a reader can check a probe against the operator's own
//! printed instant. The family's grid in this window is one wrapping leg per
//! trade date, Sunday to Thursday 17:00 CT into a 16:00 CT close the next local
//! day, so a clip stated on a trade date lands on a session that opened the
//! previous evening and a closure deletes that evening leg.

use chrono::{DateTime, Datelike as _, Days, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, EvidenceTier, ExceptionBlock, ExceptionBlockKind,
    ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind, SessionState,
    calendar_for_market_hours_key,
};

/// The family under test, as a date-aware calendar.
fn rates() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexInterestRates)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// A venue-local Central-Time wall clock, as the operator prints it.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous CT instant")
        .with_timezone(&Utc)
}

/// Asserts that a query about a pre-floor probe states the floor refusal.
///
/// Stage 2B (#115) makes every identity-backed, date-aware query refuse a date
/// below the permanent 2025-01-01 floor. The 2009-2024 sweeps in this suite
/// therefore cannot read the answers they were written for: each probe below
/// states the refusal it now gets, while everything the static holiday table
/// still states — the row's date, kind, tier and citation, read through
/// `holiday_on`, which is not date-aware — is asserted unchanged beside it and
/// named in the test's own comment.
#[track_caller]
fn assert_below_floor<T>(result: Result<T, CalendarQueryError>, date: NaiveDate) {
    let refused = result.err();
    assert!(
        matches!(
            refused,
            Some(CalendarQueryError::BeforeSupportFloor { date: named, .. }) if named == date
        ),
        "{date} precedes the 2025-01-01 floor and its refusal must name it, got {refused:?}",
    );
}

/// Asserts that an in-domain query refuses a date no covered range holds.
///
/// Two shapes reach this: a date above the family's `2027-12-31` horizon, and
/// an at-or-after-floor date whose query has to read the withheld Sunday
/// quarter-hour the identity's `#79` declaration covers — `coverage()` reports
/// `OutsideCoveredRange` for that date either way. A pre-floor date never
/// reaches here: the floor governs the phase check first, so those probes state
/// [`assert_below_floor`]'s error instead.
#[track_caller]
fn assert_out_of_range<T>(result: Result<T, CalendarQueryError>, date: NaiveDate) {
    let refused = result.err();
    assert!(
        matches!(
            refused,
            Some(CalendarQueryError::OutsideCoveredRange { date: named, .. }) if named == date
        ),
        "{date} is outside this identity's covered ranges and its refusal must name it, got {refused:?}",
    );
}

/// 12:00 CT, the noon halt CME prints on the Monday and Thursday holidays.
const NOON: u32 = 12 * 3_600;

// ---------------------------------------------------------------------------
// 1. A closed day.
// ---------------------------------------------------------------------------

/// Christmas 2025 is a full closure, so its whole trading day goes — including
/// the leg that opened at 17:00 CT on Christmas Eve — while the leg that opens
/// on Christmas evening, which belongs to trade date 2025-12-26, survives.
#[test]
fn a_closed_trade_date_removes_the_whole_christmas_trading_day() {
    let calendar = rates();
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
    // Three probes inside the civil day, none of them open.
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (0, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (8, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // The next trade date's leg opens inside the holiday's civil day, so the
    // civil day is not wholly closed. `is_closed_trade_date` is the holiday
    // question; `is_closed_all_day_on` is not.
    assert!(
        calendar
            .is_open(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
}

/// Good Friday 2027 closes a Friday, whose trading day is the only one the
/// week hands to the weekend: the Thursday-evening leg disappears and the next
/// open is the Sunday-evening one.
#[test]
fn a_closed_friday_hands_the_next_open_to_sunday_evening() {
    let calendar = rates();
    assert_eq!(
        calendar.holiday_on(day(2027, 3, 26)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );

    // Thursday 2027-03-25 trades its own day to the ordinary 16:00 CT close.
    assert!(
        calendar
            .is_open(ct((2027, 3, 25), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // Its evening leg fed the closed Friday and is gone.
    assert!(
        !calendar
            .is_open(ct((2027, 3, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2027, 3, 26), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2027, 3, 25), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2027, 3, 28), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// 2 and 3. An early close, on both sides of the cutoff.
// ---------------------------------------------------------------------------

/// Thanksgiving Day publishes no final close, so 2025-11-28 now owns the span
/// from Wednesday evening and ends at this family's own `12:15` CT close.
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "midnight is written in the table fence's own `h * 3_600 + m * 60` grammar, \
              because the fence rejects a bare zero"
)]
static EXPECTED_2025_11_28: [ExceptionBlock; 7] = [
    ExceptionBlock::order_entry(-2, 16 * 3_600 + 45 * 60, 17 * 3_600),
    ExceptionBlock::extended(-2, 17 * 3_600, 12 * 3_600),
    ExceptionBlock::order_entry(-1, 12 * 3_600, 17 * 3_600),
    ExceptionBlock::extended(-1, 17 * 3_600, 24 * 3_600),
    ExceptionBlock::extended(0, 0 * 3_600 + 0 * 60, 7 * 3_600),
    ExceptionBlock::order_entry(0, 7 * 3_600, 7 * 3_600 + 30 * 60),
    ExceptionBlock::extended(0, 7 * 3_600 + 30 * 60, 12 * 3_600 + 15 * 60),
];

/// The day after Thanksgiving 2025 closes at 12:15 CT, and the clip is stated
/// on the trade date, so it lands on a session that opened at 17:00 CT the
/// previous evening.
#[test]
fn the_day_after_thanksgiving_clips_a_session_opened_the_previous_evening() {
    let calendar = rates();
    let holiday = calendar
        .holiday_on(day(2025, 11, 28))
        .expect("2025-11-28 ships a row");
    assert_eq!(
        holiday.kind(),
        HolidayKind::ReplacementBlocks(&EXPECTED_2025_11_28)
    );
    assert_eq!(holiday.document_id(), "CME-SVC-2025-11-26");

    // The instant before the close, and the close itself: closes are
    // end-exclusive.
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
    // The remainder of the trading day is gone, not merely quiet.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (14, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    assert_eq!(
        calendar
            .session_bounds(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((
            ct((2025, 11, 28), (7, 30, 0)),
            ct((2025, 11, 28), (12, 15, 0))
        ))
    );
    assert_eq!(
        calendar
            .candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 11, 28), (12, 15, 0)))
    );
    // Friday hands over to the weekend, so the next open is Sunday evening.
    assert_eq!(
        calendar
            .next_session_open_after(ct((2025, 11, 28), (12, 20, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2025, 11, 30), (17, 0, 0)))
    );
}

/// Each distinct early-close instant the table carries is fenced on both sides.
///
/// A one-minute slip in any of the four would otherwise be invisible: the noon
/// halt of the Monday and Thursday holidays, the 12:15 CT half-days, Good
/// Friday 2026's 10:15 CT close — the day CME keeps this family trading for the
/// employment release — and Independence Day 2027's 13:30 CT halt, which is not
/// the noon default of the other Monday holidays.
#[test]
fn every_distinct_early_close_instant_is_fenced_on_both_sides() {
    let calendar = rates();
    let cutoffs = [
        (ct((2025, 1, 20), (12, 0, 0)), "MLK 2025, 12:00 CT"),
        (ct((2025, 11, 28), (12, 15, 0)), "Thanksgiving Friday 2025"),
        (ct((2026, 4, 3), (10, 15, 0)), "Good Friday 2026, 10:15 CT"),
        (ct((2027, 7, 5), (13, 30, 0)), "Independence 2027, 13:30 CT"),
    ];
    for (cutoff, label) in cutoffs {
        assert!(
            calendar
                .is_open(cutoff - chrono::TimeDelta::seconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{label}: the instant before the early close must still be open"
        );
        assert!(
            !calendar
                .is_open(cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{label}: the early close is end-exclusive"
        );
    }

    assert_eq!(
        calendar.holiday_on(day(2025, 1, 20)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose { close_ssm: NOON })
    );
    assert_eq!(
        calendar.holiday_on(day(2026, 4, 3)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 15 * 60
        })
    );
    assert_eq!(
        calendar.holiday_on(day(2027, 7, 5)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3_600 + 30 * 60
        })
    );
}

/// The Thanksgiving Saturday is a sourced closure that deletes nothing: the
/// family's normal week has no Saturday session either.
///
/// It ships so the venue tables, which are the date-by-date intersection of the
/// families routing to a venue, do not lose an audited closure this family
/// merely declined to state.
#[test]
fn the_thanksgiving_saturday_row_changes_no_answer() {
    let calendar = rates();
    assert_eq!(
        calendar.holiday_on(day(2025, 11, 29)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        calendar
            .is_closed_trade_date(day(2025, 11, 29), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 11, 29), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .without_holidays()
            .is_open(ct((2025, 11, 29), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "the normal week has no Saturday session, so the row deletes nothing"
    );
    // The Sunday-evening reopen is untouched.
    assert!(
        calendar
            .is_open(ct((2025, 11, 30), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// 4. A late open — which this family does not have, and must not acquire.
// ---------------------------------------------------------------------------

/// No 2025-2027 holiday moves this family's **first** open, so the table ships
/// no late open and every post-closure reopen is the ordinary 17:00 CT one.
///
/// The absence is the assertion: if a future row were mis-encoded as a late
/// open, or a reopen were shifted by the `late_open_ssm` branch that resolves
/// on the preceding local date, one of these boundaries would move by hours.
#[test]
fn no_late_open_ships_and_the_post_closure_reopen_is_the_normal_open() {
    let calendar = rates();
    // Christmas Day 2025 reopens for trade date 2025-12-26 at the normal hour.
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
    assert_eq!(calendar.holiday_on(day(2025, 12, 26)), None);

    // New Year's Day 2026, the same shape on a Thursday.
    assert!(
        !calendar
            .is_open(ct((2026, 1, 1), (16, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2026, 1, 1), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(calendar.holiday_on(day(2026, 1, 2)), None);

    // Over the audited window, every row is a closure, an early close, a late
    // open or an `Unsourced` statement. The retained window ships no late open
    // and no `Unsourced` row, and the counts pin its shape, so a row on a date
    // no test names fails here.
    let coverage = calendar
        .holiday_coverage()
        .expect("this family ships a table");
    let (mut closed, mut early, mut late, mut unsourced) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            // No row, or a complete-day replacement: neither is a boundary move,
            // so neither counts in the three columns below.
            None | Some(HolidayKind::ReplacementBlocks(_)) => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { .. }) => early += 1,
            Some(HolidayKind::LateOpen { .. }) => late += 1,
            Some(HolidayKind::Unsourced) => unsourced += 1,
            Some(other) => {
                panic!("{date} is not one of the kinds this family ships: {other:?}")
            }
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }
    assert_eq!(
        (closed, early, late, unsourced),
        (9, 24, 0, 0),
        "closed, early-close, late-open and unsourced rows over the retained window"
    );
}

// ---------------------------------------------------------------------------
// 5. A wrap removed by a closure.
// ---------------------------------------------------------------------------

/// The Christmas shape: the eve's own day is clipped at 12:15 CT, the evening
/// leg that would have fed Christmas is deleted rather than clipped, and the
/// next open is Christmas evening's 17:00 CT.
#[test]
fn a_closure_removes_the_prior_evening_wrap() {
    let calendar = rates();
    assert!(
        calendar
            .is_open(ct((2025, 12, 24), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2025, 12, 24), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // No session at 17:30 CT on Christmas Eve: that leg's trade date is closed.
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
        Some(ct((2025, 12, 25), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// 6. The trade-date consequence.
// ---------------------------------------------------------------------------

/// A shortened day keeps its own trade date, and the leg opening on a closed
/// date already belongs to the next trade date.
///
/// The deleted 2025-12-24 evening leg is **no longer readable as a trade-date
/// answer**: Stage 2B refuses the probe below, so the claim "a deleted leg has
/// none" survives only as the refusal this test now states. Which row deletes
/// which leg is still fenced by the table itself.
///
/// The 2026-06-19 probe is the interpretive step the evidence file records:
/// CME gives that Friday's 12:00 CT close the following Monday's trade date,
/// while this crate assigns a session the venue-local date of its final close
/// and this family has no following-business-day roll. The session is modelled
/// exactly as published; only the label differs.
#[test]
fn trade_dates_follow_the_shortened_and_the_deleted_days() {
    let calendar = rates();
    assert_eq!(
        calendar
            .trade_date(ct((2025, 11, 28), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    // 2025-12-24's deleted evening leg can no longer be read as "no trade
    // date": the instant is in no session, so the trade-date walk has to
    // consult the order-entry queue, and the identity's `#79` declaration
    // withholds that phase through 2026-08-21. The refusal is what this probe
    // states now; the deleted leg itself remains a fact of the shipped row.
    assert_out_of_range(
        calendar.trade_date(ct((2025, 12, 24), (17, 30, 0))),
        day(2025, 12, 24),
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 25), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26))
    );
    // The noon halt of a Monday holiday shortens that Monday, and the whole span
    // through it carries the *next* trade date: CME publishes no final close for
    // the holiday, so the operator labels it 2025-01-21.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 20), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 1, 21))
    );
    assert!(
        calendar
            .is_open(ct((2025, 1, 20), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2025, 1, 20), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 1, 21))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2026, 6, 19), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 6, 22))
    );
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// Inside the window a date with no row is audited normal; outside it the table
/// has no answer, and a holiday it would otherwise have carried is not applied.
///
/// Both outside probes are refused rather than answered — 2024-12-25 for
/// preceding the permanent 2025-01-01 floor, 2028-01-17 for lying above the
/// family's 2027-12-31 horizon — so the non-application is read from the
/// table's own silence, and the refusal is what each probe states.
#[test]
fn the_coverage_window_bounds_what_the_table_answers() {
    let calendar = rates();
    let coverage = calendar
        .holiday_coverage()
        .expect("this family ships a table");
    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2026, 7, 3)));

    // Inside, with no row: audited normal.
    assert_eq!(calendar.holiday_on(day(2026, 10, 22)), None);
    assert!(
        calendar
            .is_open(ct((2026, 10, 22), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // The pre-floor eras left with Stage 5 of the release plan (#117), so the
    // window opens at the permanent 2025-01-01 floor and the dates below it
    // have neither a row nor an audited normal week. Christmas Day 2024 is a
    // closure the module shipped until then.
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert!(!coverage.contains(day(2024, 12, 25)));
    assert_eq!(
        calendar.holiday_on(day(2024, 12, 25)).map(Holiday::kind),
        None
    );
    // Below the audited range *and* below the permanent 2025-01-01 floor: the
    // table's silence is static, the query is refused.
    assert_below_floor(
        calendar.is_open(ct((2024, 12, 25), (9, 0, 0))),
        day(2024, 12, 25),
    );

    // One day above the window, and a known CME holiday above it: the table
    // does not silently extend, and the query above the family's 2027-12-31
    // horizon is refused rather than answered as open.
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 17)), None);
    assert_out_of_range(
        calendar.is_open(ct((2028, 1, 17), (9, 0, 0))),
        day(2028, 1, 17),
    );

    // The last audited trade date is an ordinary Friday.
    assert_eq!(calendar.holiday_on(day(2027, 12, 31)), None);
    assert!(
        calendar
            .is_open(ct((2027, 12, 31), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// 8. `without_holidays` restores the normal-week answer.
// ---------------------------------------------------------------------------

/// The consumer's escape hatch detaches the table exactly, on every date the
/// table changes and on the accessors that report it.
#[test]
fn without_holidays_restores_the_normal_week_answer() {
    let calendar = rates();
    let bare = calendar.without_holidays();

    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day(2025, 12, 25)), None);
    assert_eq!(
        bare.market_hours_key(),
        Some(MarketHoursKey::GlobexInterestRates)
    );

    // A full closure: the normal week trades the whole day.
    assert!(
        !calendar
            .is_open(ct((2025, 12, 25), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2025, 12, 25), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The wrap the closure removed is back.
    assert!(
        bare.is_open(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // An early close: the normal Friday runs to its 16:00 CT close.
    assert!(
        !calendar
            .is_open(ct((2025, 11, 28), (14, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2025, 11, 28), (14, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        bare.trade_date(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 25))
    );
    // Away from the table's rows the two agree, which is what makes the
    // detach a control rather than a different calendar.
    assert_eq!(
        calendar
            .session_bounds(ct((2026, 10, 22), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        bare.session_bounds(ct((2026, 10, 22), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
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
/// 2010-04-02 Good Friday early close, the 2013-12-25 and 2024-12-25 closures,
/// the 2019-06-19 `Unsourced` row, the 2022-11-25 early close and the
/// 2012-01-03 late open. `holiday_on` answers `None` for all of them (the
/// table has no answer below the floor), the coverage no longer contains any
/// of them, and the date-aware query returns the explicit `BeforeSupportFloor`
/// error (LAW-COVERAGE), never a closure or a normal week read from the
/// removed rows.
#[test]
fn pre_floor_rows_refuse_instead_of_answering() {
    let calendar = rates();

    for (year, month, date) in [
        (2010, 4, 2),
        (2012, 1, 3),
        (2013, 12, 25),
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
        assert_below_floor(
            calendar.is_closed_trade_date(removed, SessionKind::Both),
            removed,
        );
        assert_below_floor(
            calendar.is_open(ct((year, month, date), (10, 0, 0))),
            removed,
        );
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
    let calendar = rates();
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
    let blocks = match rates().holiday_on(day).map(Holiday::kind) {
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
/// moving either one changed no answer any test read — moving the queue's open
/// or the envelope's open failed no test in this whole suite.
#[test]
fn a_saturday_session_row_states_its_queue_and_evening_open() {
    for (trade_date, saturday) in SATURDAY_TRADE_DATES {
        let saturday_open = ct(saturday, (5, 0, 0));
        let saturday_close = ct(saturday, (17, 0, 0));
        let sunday = (saturday.0, saturday.1, saturday.2 + 1);
        let evening_open = ct(sunday, (17, 0, 0));
        let final_close = ct(trade_date, (16, 0, 0));

        assert_eq!(
            rates()
                .session_bounds(ct(saturday, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((saturday_open, saturday_close)),
            "{trade_date:?}: the Saturday session's own bounds"
        );
        assert_eq!(
            rates()
                .next_session_open_after(saturday_close)
                .expect("2026 and 2027 are covered dates"),
            Some(evening_open),
            "{trade_date:?}: the next open after the Saturday close is the Sunday \
             17:00 CT session the row states"
        );
        assert_eq!(
            rates()
                .session_bounds(ct(sunday, (16, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((evening_open, final_close)),
            "{trade_date:?}: the Sunday-Monday session's own bounds"
        );
        assert_eq!(
            rates()
                .session_bounds(ct(trade_date, (10, 0, 0)))
                .expect("2026 and 2027 are covered dates"),
            Some((evening_open, final_close)),
            "{trade_date:?}: the same session, probed on its closing day"
        );
        assert!(
            rates()
                .is_open(evening_open)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Sunday 17:00 CT open must be inside the session"
        );
        assert!(
            !rates()
                .is_open(saturday_close)
                .expect("2026 and 2027 are covered dates"),
            "{trade_date:?}: the Saturday close is end-exclusive"
        );

        // The one phase-level probe this identity answers on a post-2026-08-22
        // date: the 2027-06-20 queue is served, so the state query reports the
        // order-entry phase there. The two 2026 dates refuse it by policy, which
        // is why the queue's interval is pinned by the accessor below rather
        // than by a query on every date.
        if sunday == (2027, 6, 20) {
            assert_eq!(
                rates()
                    .session_state(ct(sunday, (16, 0, 0)))
                    .expect("2027-06-20 is inside the served era"),
                SessionState::OrderEntry,
                "{trade_date:?}: 16:00 CT is the Pre-Open queue, where no trade matches"
            );
            assert!(
                rates()
                    .is_accepting_orders(ct(sunday, (16, 0, 0)))
                    .expect("2027-06-20 is inside the served era"),
                "{trade_date:?}: orders are accepted from the queue's 16:00 CT open"
            );
        }

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
    let calendar = rates();
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
