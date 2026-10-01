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

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
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

/// Asserts that a query about a pre-floor probe states the floor refusal, and
/// that it names one of the two days the answer can depend on.
///
/// The crate reports the venue-local day it could not establish. For a probe
/// that falls in no session at all that is the probe's own civil day; for one
/// whose answer comes from a wrapping leg, or from the trade date an overlay
/// walk resolves, it is that other day. A site where both are possible states
/// both rather than guessing which path the walk took.
/// Asserts that an in-domain query refuses a date no covered range holds.
///
/// Two shapes reach this: a date above the family's `2027-12-31` horizon, and
/// an at-or-after-floor date whose query has to read the withheld Sunday
/// quarter-hour the identity's `#79` declaration covers — `coverage()` reports
/// `OutsideCoveredRange` for that date either way. A pre-floor date never
/// reaches here: the floor governs the phase check first, so those probes state
/// [`assert_below_floor`]'s error instead.
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

    // Over every audited window, every row is a closure, an early close, a
    // late open, a replacement or an `Unsourced` statement. Three late opens
    // come from 2010-2012 and three from 2013-2015; the counts pin the shape of
    // all six eras, so a row on a date no test names fails here. The
    // 2010-2012 wave's eighteen noon halts added eighteen early closes and
    // turned its three Thanksgiving Fridays into replacement rows, which is
    // the 153 -> 168 move.
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
        (47, 170, 6, 4),
        "closed, early-close, late-open and unsourced rows over all six windows"
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
    // 2025-12-24's evening leg is deleted by the closure row, so the instant
    // is in no session; the walk that finds neither a session nor a queue
    // answers absence, which is the sourced closure's own answer and never a
    // refusal (#172). The deleted leg itself remains a fact of the shipped row.
    assert_eq!(
        calendar
            .trade_date(ct((2025, 12, 24), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date"),
        None
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
/// Both outside probes are refused rather than answered — 2009-12-31 for
/// preceding the permanent 2025-01-01 floor, 2028-01-17 for lying above the
/// family's 2027-12-31 horizon — so the non-application is read from the
/// table's own silence, and the refusal is what each probe states.
#[test]
fn the_coverage_window_bounds_what_the_table_answers() {
    let calendar = rates();
    let coverage = calendar
        .holiday_coverage()
        .expect("this family ships a table");
    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2026, 7, 3)));

    // Inside, with no row: audited normal.
    assert_eq!(calendar.holiday_on(day(2026, 10, 22)), None);
    assert!(
        calendar
            .is_open(ct((2026, 10, 22), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // The 2013-2015 wave shipped after this test was written, so the dates it
    // used to probe for an unaudited gap now carry its shipped rows, and the
    // probe below the audited range is 2009-12-31.
    assert!(coverage.contains(day(2015, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2015, 12, 31)), None);
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        !calendar
            .is_open(ct((2015, 12, 25), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(!coverage.contains(day(2009, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2009, 12, 31)), None);
    // 2009-12-31 is below the permanent floor, so the refusal is the floor's:
    // what the table states about the date is the absent row above.
    assert_eq!(
        calendar.is_open(ct((2009, 12, 31), (9, 0, 0))),
        Err(CalendarQueryError::BeforeSupportFloor {
            source: calendar.source(),
            date: day(2009, 12, 31)
        })
    );
    assert!(coverage.contains(day(2020, 12, 25)));
    assert_eq!(
        calendar
            .holiday_on(day(2020, 12, 25))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(coverage.contains(day(2024, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(
        calendar
            .holiday_on(day(2024, 12, 25))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );

    // One day above the window, and a known CME holiday above it: the table
    // does not silently extend.
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 17)), None);
    // The probe above the window is refused: the table does not silently
    // extend, and the identity answers no holiday question there.
    assert!(
        calendar
            .is_open(ct((2028, 1, 17), (9, 0, 0)))
            .is_err_and(|error| matches!(error, CalendarQueryError::OutsideCoveredRange { .. })),
        "2028-01-17 is outside the audited window"
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
// The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 17:30 CT, the family's normal evening first open to 2012-04-30.
const ERA_EVENING_OPEN: u32 = 17 * 3_600 + 30 * 60;
/// 05:00 CT, the era's post-holiday first open.
const ERA_REOPEN: u32 = 5 * 3_600;

/// One expected block of a merged trade date: opening-day offset, kind, and
/// the venue-local seconds-since-midnight bounds.
type EraBlock = (i8, ExceptionBlockKind, u32, u32);

/// The merged-day blocks of the era's `17:30` CT clock: the Sunday Pre-Open
/// queue, the Sunday open into the Monday noon halt, the halt's order-entry
/// window to the sheet's `1730 CT` CBOT-financial resume, and the resume into
/// the Tuesday `16:00` CT close.
const ERA_BLOCKS_1730_MONDAY: [EraBlock; 4] = [
    (
        -2,
        ExceptionBlockKind::OrderEntry,
        16 * 3_600 + 15 * 60,
        17 * 3_600 + 30 * 60,
    ),
    (
        -2,
        ExceptionBlockKind::Extended,
        17 * 3_600 + 30 * 60,
        12 * 3_600,
    ),
    (
        -1,
        ExceptionBlockKind::OrderEntry,
        12 * 3_600,
        17 * 3_600 + 30 * 60,
    ),
    (
        -1,
        ExceptionBlockKind::Extended,
        17 * 3_600 + 30 * 60,
        16 * 3_600,
    ),
];

/// The 2010 merged Thanksgiving Friday, still on the `17:30` CT clock and
/// closing at the trade date's own `12:15` CT.
const ERA_BLOCKS_1730_THANKSGIVING: [EraBlock; 4] = [
    (
        -2,
        ExceptionBlockKind::OrderEntry,
        16 * 3_600 + 45 * 60,
        17 * 3_600 + 30 * 60,
    ),
    (
        -2,
        ExceptionBlockKind::Extended,
        17 * 3_600 + 30 * 60,
        12 * 3_600,
    ),
    (
        -1,
        ExceptionBlockKind::OrderEntry,
        12 * 3_600,
        17 * 3_600 + 30 * 60,
    ),
    (
        -1,
        ExceptionBlockKind::Extended,
        17 * 3_600 + 30 * 60,
        12 * 3_600 + 15 * 60,
    ),
];

/// The merged-day blocks of the unified `17:00` CT clock, closing at the
/// following trade date's ordinary `16:00` CT.
const ERA_BLOCKS_1700_MONDAY: [EraBlock; 4] = [
    (
        -2,
        ExceptionBlockKind::OrderEntry,
        16 * 3_600 + 15 * 60,
        17 * 3_600,
    ),
    (-2, ExceptionBlockKind::Extended, 17 * 3_600, 12 * 3_600),
    (-1, ExceptionBlockKind::OrderEntry, 12 * 3_600, 17 * 3_600),
    (-1, ExceptionBlockKind::Extended, 17 * 3_600, 16 * 3_600),
];

/// The 2012 Fourth-of-July merged Thursday: the `-2` day is the Tuesday, whose
/// queue is the weekday 16:45 Pre-Open the sheet's own note dates.
const ERA_BLOCKS_1700_JULY4: [EraBlock; 4] = [
    (
        -2,
        ExceptionBlockKind::OrderEntry,
        16 * 3_600 + 45 * 60,
        17 * 3_600,
    ),
    (-2, ExceptionBlockKind::Extended, 17 * 3_600, 12 * 3_600),
    (-1, ExceptionBlockKind::OrderEntry, 12 * 3_600, 17 * 3_600),
    (-1, ExceptionBlockKind::Extended, 17 * 3_600, 16 * 3_600),
];

/// The 2011 and 2012 merged Thanksgiving Friday on the unified `17:00` CT
/// clock, closing at the trade date's own `12:15` CT.
const ERA_BLOCKS_1700_THANKSGIVING: [EraBlock; 4] = [
    (
        -2,
        ExceptionBlockKind::OrderEntry,
        16 * 3_600 + 45 * 60,
        17 * 3_600,
    ),
    (-2, ExceptionBlockKind::Extended, 17 * 3_600, 12 * 3_600),
    (-1, ExceptionBlockKind::OrderEntry, 12 * 3_600, 17 * 3_600),
    (
        -1,
        ExceptionBlockKind::Extended,
        17 * 3_600,
        12 * 3_600 + 15 * 60,
    ),
];

/// One expected era row: the halt row of a noon-halt pair, the merged row of
/// a pair (with the block group it must state), or any other scalar row the
/// window ships, carried with its kind.
enum EraExpectedKind {
    Halt,
    Merged(&'static [EraBlock; 4]),
    Other(HolidayKind),
}

/// `EarlyClose { close_ssm }`, for the table below.
const fn ec(close_ssm: u32) -> HolidayKind {
    HolidayKind::EarlyClose { close_ssm }
}

/// Every row the 2010-2012 window ships, in table order.
///
/// The eighteen holiday dates whose sheet prints the `1200 CT` trading halt
/// and the eighteen following trade dates whose sheet opens the prior evening
/// **for their own trade date** are the issue's subject; the rest are the
/// wave's original rows, carried so the sweep can state "none added" over the
/// whole window. Each entry cites the sheet both rows of a pair read from,
/// because one document states the whole arrangement.
///
/// This is the era-wide instant fence the wave shipped without: the sweep
/// below walks the whole window and compares against this list row for row,
/// so a dropped, added or moved row fails as loudly as a wrong instant.
/// Reading the blocks back also fences the issue's own finding — the sheets
/// state the halt and the resume in session language, and the resume instant
/// is the family's own evening open (`1730` CBOT-financial through 2011-09-05,
/// `1700` unified from 2011-11-24), not the `1700` CME-products line alone.
const ERA_2010_2012_ROWS: &[((i32, u32, u32), EraExpectedKind, &str)] = &[
    (
        (2010, 1, 1),
        EraExpectedKind::Other(HolidayKind::Closed),
        "2010-new-years.pdf @2010-02-15T05:16:52Z",
    ),
    (
        (2010, 1, 15),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z",
    ),
    (
        (2010, 1, 18),
        EraExpectedKind::Halt,
        "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z",
    ),
    (
        (2010, 1, 19),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2010-martin-luther-king.pdf @2010-03-31T06:42:26Z",
    ),
    (
        (2010, 2, 12),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2010-presidents-day.pdf @2010-02-15T06:46:41Z",
    ),
    (
        (2010, 2, 15),
        EraExpectedKind::Halt,
        "2010-presidents-day.pdf @2010-02-15T06:46:41Z",
    ),
    (
        (2010, 2, 16),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2010-presidents-day.pdf @2010-02-15T06:46:41Z",
    ),
    (
        (2010, 4, 2),
        EraExpectedKind::Other(ec(10 * 3_600 + 15 * 60)),
        "2010-good-friday.pdf @2010-06-01T11:19:16Z",
    ),
    (
        (2010, 5, 28),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2010-memorial-day.pdf @2010-06-01T09:42:25Z",
    ),
    (
        (2010, 5, 31),
        EraExpectedKind::Halt,
        "2010-memorial-day.pdf @2010-06-01T09:42:25Z",
    ),
    (
        (2010, 6, 1),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2010-memorial-day.pdf @2010-06-01T09:42:25Z",
    ),
    (
        (2010, 7, 2),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2010-4th-of-july.pdf @2010-06-02T00:56:37Z",
    ),
    (
        (2010, 7, 5),
        EraExpectedKind::Halt,
        "2010-4th-of-july.pdf @2010-06-02T00:56:37Z",
    ),
    (
        (2010, 7, 6),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2010-4th-of-july.pdf @2010-06-02T00:56:37Z",
    ),
    (
        (2010, 9, 3),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2010-labor-day.pdf @2010-06-02T00:56:41Z",
    ),
    (
        (2010, 9, 6),
        EraExpectedKind::Halt,
        "2010-labor-day.pdf @2010-06-02T00:56:41Z",
    ),
    (
        (2010, 9, 7),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2010-labor-day.pdf @2010-06-02T00:56:41Z",
    ),
    (
        (2010, 10, 8),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2010-columbus-day.pdf @2010-08-21T13:31:22Z",
    ),
    (
        (2010, 11, 25),
        EraExpectedKind::Halt,
        "2010-thanksgiving.pdf @2010-11-22T09:40:12Z",
    ),
    (
        (2010, 11, 26),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_THANKSGIVING),
        "2010-thanksgiving.pdf @2010-11-22T09:40:12Z",
    ),
    (
        (2010, 12, 24),
        EraExpectedKind::Other(HolidayKind::Closed),
        "2010-christmas.pdf @2010-12-14T06:12:38Z",
    ),
    (
        (2010, 12, 31),
        EraExpectedKind::Other(ec(12 * 3_600 + 15 * 60)),
        "2011-new-years.pdf @2011-11-01T14:39:45Z",
    ),
    (
        (2011, 1, 14),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z",
    ),
    (
        (2011, 1, 17),
        EraExpectedKind::Halt,
        "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z",
    ),
    (
        (2011, 1, 18),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2011-martin-luther-king.pdf @2011-10-28T02:34:29Z",
    ),
    (
        (2011, 2, 18),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2011-presidents-day.pdf @2011-10-28T02:35:16Z",
    ),
    (
        (2011, 2, 21),
        EraExpectedKind::Halt,
        "2011-presidents-day.pdf @2011-10-28T02:35:16Z",
    ),
    (
        (2011, 2, 22),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2011-presidents-day.pdf @2011-10-28T02:35:16Z",
    ),
    (
        (2011, 4, 22),
        EraExpectedKind::Other(HolidayKind::Closed),
        "2011-good-friday.pdf @2011-10-28T02:37:07Z",
    ),
    (
        (2011, 5, 27),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2011-memorial-day.pdf @2013-09-30T10:56:52Z",
    ),
    (
        (2011, 5, 30),
        EraExpectedKind::Halt,
        "2011-memorial-day.pdf @2013-09-30T10:56:52Z",
    ),
    (
        (2011, 5, 31),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2011-memorial-day.pdf @2013-09-30T10:56:52Z",
    ),
    (
        (2011, 7, 1),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2011-4th-of-july.pdf @2011-11-01T14:40:54Z",
    ),
    (
        (2011, 7, 4),
        EraExpectedKind::Halt,
        "2011-4th-of-july.pdf @2011-11-01T14:40:54Z",
    ),
    (
        (2011, 7, 5),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2011-4th-of-july.pdf @2011-11-01T14:40:54Z",
    ),
    (
        (2011, 9, 2),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2011-labor-day.pdf @2011-11-01T14:43:45Z",
    ),
    (
        (2011, 9, 5),
        EraExpectedKind::Halt,
        "2011-labor-day.pdf @2011-11-01T14:43:45Z",
    ),
    (
        (2011, 9, 6),
        EraExpectedKind::Merged(&ERA_BLOCKS_1730_MONDAY),
        "2011-labor-day.pdf @2011-11-01T14:43:45Z",
    ),
    (
        (2011, 10, 7),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2011-columbus-day.pdf @2011-11-01T14:39:16Z",
    ),
    (
        (2011, 11, 24),
        EraExpectedKind::Halt,
        "2011-thanksgiving.pdf @2011-11-24T18:52:46Z",
    ),
    (
        (2011, 11, 25),
        EraExpectedKind::Merged(&ERA_BLOCKS_1700_THANKSGIVING),
        "2011-thanksgiving.pdf @2011-11-24T18:52:46Z",
    ),
    (
        (2011, 12, 26),
        EraExpectedKind::Other(HolidayKind::Closed),
        "2011-christmas.pdf @2012-01-25T02:05:48Z",
    ),
    (
        (2011, 12, 27),
        EraExpectedKind::Other(HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        }),
        "2011-christmas.pdf @2012-01-25T02:05:48Z",
    ),
    (
        (2012, 1, 2),
        EraExpectedKind::Other(HolidayKind::Closed),
        "2012-new-years.pdf @2012-01-25T02:54:30Z",
    ),
    (
        (2012, 1, 3),
        EraExpectedKind::Other(HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        }),
        "2012-new-years.pdf @2012-01-25T02:54:30Z",
    ),
    (
        (2012, 1, 13),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z",
    ),
    (
        (2012, 1, 16),
        EraExpectedKind::Halt,
        "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z",
    ),
    (
        (2012, 1, 17),
        EraExpectedKind::Merged(&ERA_BLOCKS_1700_MONDAY),
        "2012-martin-luther-king.pdf @2012-05-05T16:15:26Z",
    ),
    (
        (2012, 2, 17),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2012-presidents-day.pdf @2012-05-05T16:15:39Z",
    ),
    (
        (2012, 2, 20),
        EraExpectedKind::Halt,
        "2012-presidents-day.pdf @2012-05-05T16:15:39Z",
    ),
    (
        (2012, 2, 21),
        EraExpectedKind::Merged(&ERA_BLOCKS_1700_MONDAY),
        "2012-presidents-day.pdf @2012-05-05T16:15:39Z",
    ),
    (
        (2012, 4, 6),
        EraExpectedKind::Other(ec(10 * 3_600 + 15 * 60)),
        "2012-good-friday.pdf @2012-04-17T00:42:47Z",
    ),
    (
        (2012, 5, 25),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2012-memorial-day.pdf @2012-09-15T00:37:14Z",
    ),
    (
        (2012, 5, 28),
        EraExpectedKind::Halt,
        "2012-memorial-day.pdf @2012-09-15T00:37:14Z",
    ),
    (
        (2012, 5, 29),
        EraExpectedKind::Merged(&ERA_BLOCKS_1700_MONDAY),
        "2012-memorial-day.pdf @2012-09-15T00:37:14Z",
    ),
    (
        (2012, 7, 4),
        EraExpectedKind::Halt,
        "2012-4th-of-july.pdf @2012-09-15T00:39:23Z",
    ),
    (
        (2012, 7, 5),
        EraExpectedKind::Merged(&ERA_BLOCKS_1700_JULY4),
        "2012-4th-of-july.pdf @2012-09-15T00:39:23Z",
    ),
    (
        (2012, 8, 31),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2012-labor-day.pdf @2012-09-15T00:34:37Z",
    ),
    (
        (2012, 9, 3),
        EraExpectedKind::Halt,
        "2012-labor-day.pdf @2012-09-15T00:34:37Z",
    ),
    (
        (2012, 9, 4),
        EraExpectedKind::Merged(&ERA_BLOCKS_1700_MONDAY),
        "2012-labor-day.pdf @2012-09-15T00:34:37Z",
    ),
    (
        (2012, 10, 5),
        EraExpectedKind::Other(ec(15 * 3_600 + 15 * 60)),
        "2012-columbus-day.pdf @2012-09-15T00:15:14Z",
    ),
    (
        (2012, 11, 22),
        EraExpectedKind::Halt,
        "2012-thanksgiving.pdf @2013-01-27T22:39:01Z",
    ),
    (
        (2012, 11, 23),
        EraExpectedKind::Merged(&ERA_BLOCKS_1700_THANKSGIVING),
        "2012-thanksgiving.pdf @2013-01-27T22:39:01Z",
    ),
    (
        (2012, 12, 24),
        EraExpectedKind::Other(ec(12 * 3_600 + 15 * 60)),
        "2012-christmas.pdf @2013-04-14T19:40:27Z",
    ),
    (
        (2012, 12, 25),
        EraExpectedKind::Other(HolidayKind::Closed),
        "2012-christmas.pdf @2013-04-14T19:40:27Z",
    ),
    (
        (2012, 12, 26),
        EraExpectedKind::Other(HolidayKind::LateOpen {
            open_ssm: 5 * 3_600,
        }),
        "2012-christmas.pdf @2013-04-14T19:40:27Z",
    ),
];

/// The era-wide sweep of everything the 2010-2012 window ships — the wave's
/// original rows plus the eighteen noon halts and their eighteen merged trade
/// dates: every row's date, kind, instant, tier and citation, in table order.
///
/// The halt rows carry the scalar `12:00` CT close; the merged rows carry the
/// complete replacement day, whose blocks are compared block for block. Every
/// query about the era's answers is refused — the whole 2010-2012 window
/// precedes the permanent 2025-01-01 floor — so the rows' own facts are what
/// this fence states, and they are read from the static table the way a
/// caller without a date-aware query would read them.
#[test]
fn era_2010_2012_noon_halts_and_merged_trade_dates_are_the_audited_set() {
    let calendar = rates();
    let mut index = 0_usize;
    let mut date = day(2010, 1, 1);
    while date <= day(2012, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, document) = ERA_2010_2012_ROWS.get(index).unwrap_or_else(|| {
                panic!("{date}: a row ships in the 2010-2012 window that the audited set does not record")
            });
            assert_eq!(
                (date.year(), date.month(), date.day()),
                *expected,
                "the 2010-2012 rows must ship in order, with none added"
            );
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert_eq!(row.document_id(), *document, "{date}");
            match kind {
                EraExpectedKind::Halt => {
                    assert_eq!(
                        row.kind(),
                        HolidayKind::EarlyClose {
                            close_ssm: 12 * 3_600
                        },
                        "{date} must carry the sheet's 12:00 CT halt"
                    );
                }
                EraExpectedKind::Merged(group) => {
                    let HolidayKind::ReplacementBlocks(blocks) = row.kind() else {
                        panic!("{date} must carry a replacement row, not {:?}", row.kind());
                    };
                    assert_eq!(
                        blocks.len(),
                        group.len(),
                        "{date}: the merged day states {} blocks",
                        group.len()
                    );
                    for (block, (day_offset, block_kind, open_ssm, close_ssm)) in
                        blocks.iter().zip(group.iter())
                    {
                        assert_eq!(block.open_day_offset(), *day_offset, "{date}");
                        assert_eq!(block.kind(), *block_kind, "{date}");
                        assert_eq!(block.open_ssm(), *open_ssm, "{date}");
                        assert_eq!(block.close_ssm(), *close_ssm, "{date}");
                    }
                }
                EraExpectedKind::Other(expected_kind) => {
                    assert_eq!(row.kind(), *expected_kind, "{date}");
                }
            }
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2010_2012_ROWS.len(), "every recorded row ships");
}

/// The era's early closes clip a trading day that opened 17:30 CT the previous
/// evening, so each cutoff has to land on its own trade date and delete the
/// whole remaining session — including the 15:15 CT Martin Luther King Jr. Day
/// close, the 10:15 CT Good Friday close and the 12:15 CT New Year's Eve close.
///
/// The rows' kinds, instants and tiers are still read from the static table.
/// Every query about them is refused: the 2010-2012 era precedes the permanent
/// 2025-01-01 floor, so the clipped wrap, the end-exclusive cutoff, the trade
/// date's own bounds and the Sunday-evening re-open are no longer observable
/// through the identity calendar.
#[test]
fn era_early_closes_end_the_wrapped_trading_day_at_the_stated_instant() {
    let calendar = rates();

    for (date, previous_day, close_ssm) in [
        ((2010, 1, 15), (2010, 1, 14), 15 * 3_600 + 15 * 60),
        ((2010, 4, 2), (2010, 4, 1), 10 * 3_600 + 15 * 60),
        ((2010, 12, 31), (2010, 12, 30), 12 * 3_600 + 15 * 60),
    ] {
        let holiday = calendar
            .holiday_on(day(date.0, date.1, date.2))
            .unwrap_or_else(|| panic!("{date:?} ships a row"));
        assert_eq!(holiday.kind(), HolidayKind::EarlyClose { close_ssm });
        assert_eq!(holiday.tier(), EvidenceTier::T1);

        // The evening leg that feeds this trade date is clipped, not deleted.
        assert!(
            calendar
                .is_open(ct(previous_day, (17, 30, 0)))
                .expect("the coverage contract must answer a covered date")
        );
        // One second before the close, and at it: closes are end-exclusive.
        let before = ct(
            date,
            (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
        ) - chrono::TimeDelta::seconds(1);
        assert!(
            calendar
                .is_open(before)
                .expect("the coverage contract must answer a covered date"),
            "{date:?}: {close_ssm} is too early"
        );
        assert!(
            !calendar
                .is_open(ct(
                    date,
                    (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60)
                ))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}: {close_ssm} is not end-exclusive"
        );
        // The whole trading day ends there, and it is still the trade date's
        // own session that ends.
        assert_eq!(
            calendar
                .session_bounds(ct(date, (9, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some((
                ct(previous_day, (17, 30, 0)),
                ct(
                    date,
                    (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60)
                )
            ))
        );
    }
    // Sunday evening reopens each of the three cuts.
    assert_eq!(
        calendar
            .next_session_open_after(ct((2010, 1, 15), (16, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2010, 1, 17), (17, 30, 0)))
    );
}

/// Every late open the era ships states an instant *earlier* than the family's
/// normal 17:30 CT first open, so the cutoff lands on the trade date itself
/// rather than on the preceding local date.
///
/// 2012-01-03 states 05:00 CT: the 2012-01-02 evening leg did not run and the
/// trade date begins on its own civil day. 2010-04-01 states 17:00 CT — half
/// Every late open the era ships states an instant *earlier* than the family's
/// normal 17:30 CT first open, so the cutoff lands on the trade date itself:
/// the 2011-12-27, 2012-01-03 and 2012-12-26 rows all reopen at 05:00 CT after
/// a closure removed the prior-evening leg.
///
/// The two eves the retrieval also states a `1700 CT - Regular CME Globex
/// open` for — 2010-04-01 and 2012-04-05 — ship **no** row here: 17:00 is
/// *earlier* than this era's 17:30 first open, so a late-open row can only
/// move the open later, and the cutoff would land on the trade date itself
/// after that session's own 16:00 close and delete it. The dates are named as
/// gaps in the family's evidence file.
///
/// The rows themselves — that the three dates ship a late open at 05:00 CT and
/// that 2010-04-01 and 2012-04-05 ship none — are still read from the static
/// table. Where each stated instant lands is refused: the era precedes the
/// permanent 2025-01-01 floor, so neither the first open nor the trade date it
/// keys to is observable through the calendar.
#[test]
fn era_late_opens_land_on_the_trade_date_itself() {
    let calendar = rates();
    let era_late_opens = [(2011, 12, 27), (2012, 1, 3), (2012, 12, 26)];
    // 2010-04-01 and 2012-04-05 ship no row: the operator's `1700 CT - Regular
    // CME Globex open` there is the eve's own evening open for the *next*
    // trade date, not a delayed first open of this one, and a late-open row at
    // 17:00 would land after this trade date's 16:00 close and delete it. The
    // dates are named as gaps in the family's evidence file.
    for (year, month, date) in [(2010, 4, 1), (2012, 4, 5)] {
        assert_eq!(
            calendar.holiday_on(day(year, month, date)),
            None,
            "{year}-{month:02}-{date:02} ships no row"
        );
    }
    for (year, month, date) in era_late_opens {
        let row = calendar
            .holiday_on(day(year, month, date))
            .unwrap_or_else(|| panic!("{year}-{month:02}-{date:02} ships a row"));
        let open_ssm = match row.kind() {
            HolidayKind::LateOpen { open_ssm } => open_ssm,
            other => panic!("{year}-{month:02}-{date:02} ships {other:?}, not a late open"),
        };
        assert!(
            open_ssm < ERA_EVENING_OPEN,
            "{year}-{month:02}-{date:02}: {open_ssm} is not earlier than the era's 17:30 CT open"
        );
    }

    // 2011-01-02 is a Sunday and this family derives no Sunday trade date, so
    // that row's clip is never consulted: the Monday trade date still opens at
    // the ordinary 17:30 CT on the Sunday evening.
    assert!(
        calendar
            .is_open(ct((2011, 1, 2), (17, 30, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct((2011, 1, 2), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2011, 1, 3))
    );

    let holiday = calendar
        .holiday_on(day(2012, 1, 3))
        .expect("2012-01-03 ships a row");
    assert_eq!(
        holiday.kind(),
        HolidayKind::LateOpen {
            open_ssm: ERA_REOPEN
        }
    );
    assert_eq!(
        holiday.document_id(),
        "2012-new-years.pdf @2012-01-25T02:54:30Z"
    );
    // The Monday-evening leg did not run; the stated 05:00 CT instant is the
    // first open, on the trade date itself.
    assert!(
        !calendar
            .is_open(ct((2012, 1, 2), (17, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2012, 1, 3), (4, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct((2012, 1, 3), (5, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_bounds(ct((2012, 1, 3), (5, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some((ct((2012, 1, 3), (5, 0, 0)), ct((2012, 1, 3), (16, 0, 0))))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2012, 1, 3), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2012, 1, 3))
    );
}

/// The widened window's edges answer as the module declares: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// not applied.
///
/// The 2009 probe also precedes the permanent 2025-01-01 floor, so neither
/// calendar answers it: the "detached calendar agrees" half of the old claim is
/// no longer observable, and both calendars now state the same refusal instead.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = rates();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_interest_rates ships a table");

    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(!coverage.contains(day(2009, 12, 25)));

    assert_eq!(calendar.holiday_on(day(2009, 12, 25)), None);
    assert_below_floor(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        day(2009, 12, 25),
    );
    assert_below_floor(
        bare.is_open(ct((2009, 12, 25), (10, 0, 0))),
        day(2009, 12, 25),
    );
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening, and the operator's own 17:00 CT evening leg begins the next trade
/// date untouched.
///
/// The rows' kinds and instants are still read from the static table below.
/// Every query about them is refused: the whole 2016-2018 wave precedes the
/// permanent 2025-01-01 floor, so the clip, the end-exclusive cutoff and the
/// daily candle are no longer observable through the identity calendar.
#[test]
fn wave2_early_closes_end_the_wrapped_trading_day_at_the_printed_instant() {
    let calendar = rates();

    for (date, previous_day, close_ssm) in [
        ((2016, 1, 18), (2016, 1, 17), NOON),
        ((2017, 11, 23), (2017, 11, 22), NOON),
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
        assert!(
            calendar
                .is_open(ct(previous_day, (17, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
        assert!(
            calendar
                .is_open(cutoff - Duration::seconds(1))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}: the second before the close is still open"
        );
        assert!(
            !calendar
                .is_open(cutoff)
                .expect("the coverage contract must answer a covered date"),
            "{date:?}: end-exclusive"
        );
        assert_eq!(
            calendar
                .candle_end(ct(date, (9, 0, 0)), CalendarResolution::Daily)
                .expect("the coverage contract must answer a covered date"),
            Some(cutoff),
            "{date:?}"
        );
        assert_eq!(
            calendar
                .trade_date(ct(date, (9, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some(day(date.0, date.1, date.2))
        );
    }

    // Independence Day 2017 and 2018 are ordinary trading days for this
    // family: CME prints the normal 16:00 CT close, so no row ships.
    for date in [(2017, 7, 3), (2018, 7, 3)] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
        assert!(
            calendar
                .is_open(ct(date, (15, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date:?}"
        );
    }
    // Independence Day itself is a noon cut, not a closure.
    assert_eq!(
        calendar.holiday_on(day(2017, 7, 4)).map(Holiday::kind),
        Some(HolidayKind::EarlyClose { close_ssm: NOON })
    );
    // And every closure in the era is exactly these nine dates.
    let mut closures = Vec::new();
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        if calendar.holiday_on(date).map(Holiday::kind) == Some(HolidayKind::Closed) {
            closures.push(date);
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

/// A closure removes its trade date and the leg that opened the previous
/// evening; the family ships no late open in the era, so every re-open CME
/// states is the ordinary 17:00 CT one.
///
/// The closure row is still read from the static table. Every query about it is
/// refused, because the 2016-2018 wave precedes the permanent 2025-01-01 floor,
/// so the deleted trade date, the deleted evening leg and the named 17:00 CT
/// re-open are no longer observable through the identity calendar.
#[test]
fn wave2_closures_remove_the_trade_date_and_ship_no_late_open() {
    let calendar = rates();

    assert_eq!(
        calendar.holiday_on(day(2016, 3, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        calendar
            .is_closed_trade_date(day(2016, 3, 25), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2016, 3, 24), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2016, 3, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct((2016, 3, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(ct((2016, 3, 27), (17, 0, 0)))
    );
    assert_eq!(
        calendar
            .trade_date(ct((2016, 3, 27), (18, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2016, 3, 28))
    );

    // The era ships two shapes only, and no late open.
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        if let Some(holiday) = calendar.holiday_on(date) {
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed | HolidayKind::EarlyClose { .. }
                ),
                "{date}: the era ships closures and early closes only, not {:?}",
                holiday.kind()
            );
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
}

/// The era's own window edges, and the two unaudited intervals either side.
#[test]
fn wave2_window_edges_and_unaudited_neighbours_answer_as_declared() {
    let calendar = rates();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");

    assert!(coverage.contains(day(2016, 1, 1)));
    assert!(coverage.contains(day(2018, 12, 31)));
    assert!(coverage.contains(day(2015, 12, 31)));
    // Every interval below the 2016-2018 wave is a window of its own: the
    // 2013-2015 and 2019-2021 waves both shipped after this test was written.
    assert!(coverage.contains(day(2019, 1, 1)));
    assert_eq!(
        calendar.holiday_on(day(2013, 6, 14)),
        None,
        "an ordinary Friday inside the new window is audited normal"
    );
    // Christmas 2015 is the 2013-2015 wave's own closure.
    assert_eq!(
        calendar.holiday_on(day(2015, 12, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        !calendar
            .is_open(ct((2015, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        bare.is_open(ct((2015, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date"),
        "detaching the table restores the normal Thursday"
    );
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 12:00 CT, the era's Monday and Thursday holiday close.
const ERA_NOON: u32 = 12 * 3_600;
/// 12:15 CT, the era's Thanksgiving Friday and Christmas Eve close.
const ERA_QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence: the sweep below walks the whole window
/// and compares against this list row for row, so a dropped, added or moved
/// row fails as loudly as a wrong instant. A sample would let a slipped close
/// move unnoticed on the dates nobody probed.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    (
        (2022, 1, 17),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 2, 21),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 5, 30),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 6, 20),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 9, 5),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
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
    (
        (2023, 2, 20),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 4, 7),
        HolidayKind::EarlyClose {
            close_ssm: 10 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 5, 29),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 6, 19),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 9, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 23),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2024, 1, 15),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 2, 19),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T2,
    ),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 6, 19),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T2,
    ),
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
            close_ssm: ERA_QUARTER_PAST_NOON,
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

/// The civil day after `date`, which is the trade date an evening leg carries.
fn day_after(date: NaiveDate) -> NaiveDate {
    date.checked_add_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// The ordinary 17:00 CT evening open that follows a closed trade date: this
/// civil date's own leg when the week has one, otherwise the Sunday evening
/// that opens the next week — the grid has no Friday-evening occurrence.
fn era_reopen_after_closure(date: NaiveDate) -> DateTime<Utc> {
    let reopen = if date.weekday() == Weekday::Fri {
        date.checked_add_days(Days::new(2))
            .expect("the era is far from the representable bound")
    } else {
        date
    };
    ct_on(reopen, (17, 0, 0))
}

/// The era-wide sweep: every shipped date's kind, instant and tier, and the
/// refusal each probe of it states.
///
/// The rows themselves are still read from the static table, so a dropped,
/// added or moved row fails as loudly as before. The instant-level half of the
/// sweep is now a refusal fence: every one of these dates precedes the
/// permanent 2025-01-01 floor, so the crate states no open, no bounds and no
/// trade date for them.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = rates();
    let mut index = 0_usize;
    let (mut noons, mut quarter_past_noon, mut ten_fifteens) = (0_usize, 0, 0);
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
                        ERA_NOON => noons += 1,
                        ERA_QUARTER_PAST_NOON => quarter_past_noon += 1,
                        ERA_TEN_FIFTEEN => ten_fifteens += 1,
                        other => panic!("{date}: the era ships no {other} CT close"),
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(
                        calendar
                            .is_open(ct_on(day_before(date), (17, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        calendar
                            .is_open(ct_on(day_before(date), (19, 30, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert_eq!(
                        calendar
                            .trade_date(ct_on(day_before(date), (18, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(
                        calendar
                            .is_open(cutoff - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !calendar
                            .is_open(cutoff)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}: end-exclusive"
                    );
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle.
                    assert_eq!(
                        calendar
                            .session_bounds(ct_on(date, (9, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        Some((ct_on(day_before(date), (17, 0, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar
                            .candle_end(ct_on(date, (9, 0, 0)), CalendarResolution::Daily)
                            .expect("the coverage contract must answer a covered date"),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar
                            .trade_date(cutoff - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        Some(date),
                        "{date}"
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
        (noons, quarter_past_noon, ten_fifteens, closures, unsourced),
        (20, 4, 1, 7, 1),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
///
/// What is still asserted is the set of dates the table closes and, for each,
/// the refusal every query about it states: the era precedes the permanent
/// 2025-01-01 floor, so the deleted leg, the deleted civil day and the named
/// re-open are no longer observable through the identity calendar.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = rates();
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(date.0, date.1, date.2);
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("the coverage contract must answer a covered date"),
            "{date}"
        );
        // The evening leg that would have carried this trade date is gone.
        assert!(
            !calendar
                .is_open(ct_on(day_before(date), (17, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date}"
        );
        assert!(
            !calendar
                .is_open(ct_on(day_before(date), (19, 30, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date}"
        );
        // And so is the trade date's own civil day.
        assert!(
            !calendar
                .is_open(ct_on(date, (9, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date}"
        );
        assert!(
            !calendar
                .is_open(ct_on(date, (15, 59, 0)))
                .expect("the coverage contract must answer a covered date"),
            "{date}"
        );
        // No session holds this instant and the closure deleted the trade date,
        // so the walk finds neither a session nor a queue: absence, never a
        // refusal.
        assert_eq!(
            calendar
                .trade_date(ct_on(date, (10, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            None,
            "{date}"
        );

        let reopen = era_reopen_after_closure(date);
        assert_eq!(
            calendar
                .next_session_open_after(ct_on(date, (10, 0, 0)))
                .expect("the coverage contract must answer a covered date"),
            Some(reopen),
            "{date}: the next session is the ordinary evening open"
        );
        if reopen == ct_on(date, (17, 0, 0)) {
            assert_eq!(
                calendar
                    .trade_date(reopen)
                    .expect("the coverage contract must answer a covered date"),
                Some(day_after(date)),
                "{date}"
            );
        }
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// Every query about an `Unsourced` date refuses, and the row's own statement
/// is unchanged.
///
/// The claim this helper used to make — that such a row clips nothing, so the
/// attached and the detached calendar answer every probe identically — is no
/// longer observable: both calendars refuse every probe, on every date the
/// table withholds. What survives is the pair of refusals, plus the row's kind
/// and tier, which are static.
///
/// Which day a refusal names is the entry point's business, not the row's:
/// `is_open`, `session_bounds`, `next_session_open_after`, `candle_end` and the
/// two closed-day questions name the probe's own day, while the trade-date walk
/// names the trade date it resolved. Every one of these pre-floor probes is
/// refused with the permanent 2025-01-01 floor error, including the ones that
/// fall in **no session at all** on this grid (Friday evening or Saturday) and
/// so send the walk to the order-entry queue the floor governs first.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = rates();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");

    // The withheld row contributes no clip, so the detached normal week keeps
    // the ordinary answer; the attached identity refuses everything it reads
    // the withheld date through, never as a claimed closure.
    assert!(
        detached
            .is_closed_trade_date(date, SessionKind::Both)
            .is_ok(),
        "{date}: the detached grid must answer"
    );
    assert!(
        detached
            .is_closed_all_day_on(date, SessionKind::Both)
            .is_ok(),
        "{date}: the detached grid must answer"
    );
    // 2021-06-19 is a Saturday, which the grid has no session on whether or
    // not a row exists, so the date-keyed queries answer there without ever
    // reading the withheld row; a trading day's derivation reads it and
    // refuses as `UnresolvedGap` — a refused day is never reported closed.
    if matches!(date.weekday(), Weekday::Sat | Weekday::Sun) {
        // A withheld weekend is still withheld (issue #107's explicit day
        // gate): the closure question is asked *about* the date, so the
        // attached identity refuses it rather than letting the normal week
        // describe a day its row withholds — an unknown date is never a
        // market closure. The detached grid keeps its NormalWeekOnly
        // relaxation and answers from the sourced week, whose weekends
        // carry no session for this family.
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
            "{date}: a withheld date is refused, never reported closed"
        );
        assert!(
            detached
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("the detached grid keeps its normal-week relaxation"),
            "{date}"
        );
    } else {
        assert_eq!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            Err(CalendarQueryError::UnresolvedGap {
                source: calendar.source(),
                date
            })
        );
        assert_eq!(
            calendar.is_closed_all_day_on(date, SessionKind::Both),
            Err(CalendarQueryError::UnresolvedGap {
                source: calendar.source(),
                date
            })
        );
    }

    // Whether a probe instant lies inside the wrapping session that opened
    // 17:00 CT the previous evening: that evening leg runs Sunday to Thursday,
    // and the leg it opens runs to 16:00 CT on the following local day, so the
    // 18:00 probes are inside it on Sunday-Thursday and the daytime probes on
    // Monday-Friday.
    let inside_wrapping_session = |probe_day: NaiveDate, evening: bool| {
        if evening {
            !matches!(probe_day.weekday(), Weekday::Fri | Weekday::Sat)
        } else {
            !matches!(probe_day.weekday(), Weekday::Sat | Weekday::Sun)
        }
    };

    for (probe, probe_day, evening) in [
        (ct_on(day_before(date), (18, 0, 0)), day_before(date), true),
        (ct_on(date, (9, 0, 0)), date, false),
        (ct_on(date, (15, 59, 0)), date, false),
        (ct_on(date, (18, 0, 0)), date, true),
    ] {
        // The detached grid is the ordinary week: `is_open` reads the wrap
        // arithmetic, the row having clipped nothing there. The attached
        // identity refuses every probe whose derivation reads the withheld
        // date — the on-date probes name it directly, and a held wrap is dated
        // by it — and a refused probe is never read as a claimed closure. A
        // probe off the date that holds no session anywhere answers, exactly
        // as the detached grid does. The bounded next-open walk always reads
        // forward across the withheld date, so it refuses on every probe.
        let held = inside_wrapping_session(probe_day, evening);
        let detached_open = detached
            .is_open(probe)
            .expect("the coverage contract must answer a covered date");
        assert_eq!(
            detached_open, held,
            "{probe}: the detached grid is the ordinary week"
        );
        let probe_day_local = probe.with_timezone(&US::Central).date_naive();
        if probe_day_local == date || detached_open {
            assert_eq!(
                calendar.is_open(probe),
                Err(CalendarQueryError::UnresolvedGap {
                    source: calendar.source(),
                    date
                })
            );
        } else {
            assert_eq!(
                calendar
                    .is_open(probe)
                    .expect("the coverage contract must answer a covered date"),
                detached_open,
                "{probe}: the row changes no answer off the withheld date"
            );
        }
        assert!(
            calendar
                .session_bounds(probe)
                .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
            "{probe}: the bounds must not claim a session across the withheld date"
        );
        assert!(
            calendar
                .next_session_open_after(probe)
                .is_err_and(|error| matches!(
                    error,
                    CalendarQueryError::UnresolvedGap { .. }
                        | CalendarQueryError::OutsideCoveredRange { .. }
                )),
            "{probe}: the walk must not claim a session across the withheld date"
        );
    }
}

#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = rates();
    let date = day(2023, 1, 16);
    let row = calendar
        .holiday_on(date)
        .unwrap_or_else(|| panic!("{date} ships a row"));
    assert_unsourced_changes_nothing(date, row, EvidenceTier::T2);
}

/// The two 2023 holidays the unsuffixed operator summary sheets settled are
/// sourced early closes at the sheet's own instants — 12:00 CT on Presidents
/// Day and 10:15 CT on Good Friday — cited to the sheet the family's own
/// evidence table resolves, at T1, the analogue of 2022-02-21 and 2021-04-02.
#[test]
fn era_2022_2023_sheet_sourced_early_closes() {
    let calendar = rates();
    for (date, close_ssm, document) in [
        (
            (2023, 2, 20),
            12 * 3_600,
            "files/presidents-day.pdf @2023-03-29T11:57:47Z",
        ),
        (
            (2023, 4, 7),
            10 * 3_600 + 15 * 60,
            "files/good-friday.pdf @2024-07-08T16:00:09Z",
        ),
    ] {
        let date = day(date.0, date.1, date.2);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_eq!(row.kind(), HolidayKind::EarlyClose { close_ssm }, "{date}");
        assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
        assert_eq!(row.document_id(), document, "{date}");
    }
}

/// The 2022-2024 window sits fifth in the declared coverage, its edges
/// answer, and the 2013-2015 interval below the 2016-2018 wave is a window of
/// its own since this wave shipped. (The 2019-2021 interval this test used to
/// fence became a window of its own when that wave shipped; the section below
/// fences it.)
#[test]
fn era_2022_2024_window_sits_fifth_and_the_2013_2015_era_is_audited() {
    let calendar = rates();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_interest_rates ships a table");

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

    // The 2013-2015 wave shipped after this test was written, so the dates it
    // used to probe for an unaudited gap now carry its shipped rows.
    assert_eq!(
        calendar.holiday_on(day(2013, 6, 14)),
        None,
        "an ordinary Friday inside the new window is audited normal"
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
    assert!(
        !calendar
            .is_open(ct((2015, 12, 25), (10, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct((2015, 12, 24), (13, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

/// The era's closes are the financial families' 12:00 CT on the Monday and
/// Thursday holidays and 12:15 CT on the Thanksgiving Fridays and the two
/// Christmas Eves — checked against the energy venue, which keeps trading to
/// 13:30 CT on the same holiday, so a copied instant fails here.
///
/// The cross-venue comparison itself is no longer observable: Stage 2B refuses
/// every 2022-2024 probe on **both** calendars, because the era precedes the
/// permanent 2025-01-01 floor. What the pair still states is that refusal, and
/// the rows' own instants remain fenced by the era's table.
#[test]
fn era_2022_2024_close_instants_are_the_financial_ones_not_the_energy_venue() {
    let rates = rates();
    let energy = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);

    // 2022-01-17: rates stop at 12:00 CT; energy trades on to 13:30 CT.
    assert!(
        rates
            .is_open(ct((2022, 1, 17), (11, 59, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !rates
            .is_open(ct((2022, 1, 17), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        energy
            .is_open(ct((2022, 1, 17), (12, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // 2022-11-25: the Thanksgiving Friday closes at 12:15 CT here, 12:45 CT
    // there.
    assert!(
        rates
            .is_open(ct((2022, 11, 25), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !rates
            .is_open(ct((2022, 11, 25), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        energy
            .is_open(ct((2022, 11, 25), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !energy
            .is_open(ct((2022, 11, 25), (12, 45, 0)))
            .expect("the coverage contract must answer a covered date")
    );

    // 2024-12-24: 12:15 CT here, 12:45 CT there.
    assert!(
        rates
            .is_open(ct((2024, 12, 24), (12, 14, 59)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !rates
            .is_open(ct((2024, 12, 24), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        energy
            .is_open(ct((2024, 12, 24), (12, 15, 0)))
            .expect("the coverage contract must answer a covered date")
    );
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// 10:15 CT, the one Good Friday 2021 close this era prints.
const ERA_TEN_FIFTEEN: u32 = 10 * 3_600 + 15 * 60;

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, and the refusal each probe of it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. The
/// instant-level half is now a refusal fence: the whole era precedes the
/// permanent 2025-01-01 floor, so the printed close, the clipped wrap and the
/// trade date each row carried are no longer observable through the calendar.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = rates();
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
                        ERA_TEN_FIFTEEN => ten_fifteens += 1,
                        other => {
                            panic!("{date}: this family ships no {other}-second CT close here")
                        }
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(
                        calendar
                            .is_open(ct_on(day_before(date), (17, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        calendar
                            .is_open(ct_on(day_before(date), (19, 30, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert_eq!(
                        calendar
                            .trade_date(ct_on(day_before(date), (18, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(
                        calendar
                            .is_open(cutoff - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !calendar
                            .is_open(cutoff)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}: end-exclusive"
                    );
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle. The probe sits just inside the
                    // session, so the 10:15 Good Friday close cannot make the
                    // query answer `None` instead.
                    let inside = cutoff - Duration::minutes(1);
                    assert_eq!(
                        calendar
                            .session_bounds(inside)
                            .expect("the coverage contract must answer a covered date"),
                        Some((ct_on(day_before(date), (17, 0, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar
                            .candle_end(inside, CalendarResolution::Daily)
                            .expect("the coverage contract must answer a covered date"),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar
                            .trade_date(cutoff - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        Some(date),
                        "{date}"
                    );
                }
                HolidayKind::Closed => {
                    closures += 1;
                    assert!(
                        calendar
                            .is_closed_trade_date(date, SessionKind::Both)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    // The evening leg that would have carried this trade date
                    // is gone, and so is the trade date's own session.
                    assert!(
                        !calendar
                            .is_open(ct_on(day_before(date), (17, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !calendar
                            .is_open(ct_on(day_before(date), (19, 30, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !calendar
                            .is_open(ct_on(date, (9, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !calendar
                            .is_open(ct_on(date, (15, 59, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    // Absence, not a refusal: the closure deleted the trade
                    // date and no session or queue holds the instant.
                    assert_eq!(
                        calendar
                            .trade_date(ct_on(date, (10, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        None,
                        "{date}"
                    );
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
    let calendar = rates();
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
    let calendar = rates();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_interest_rates ships a table");
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
    assert!(
        calendar
            .is_open(ct((2021, 12, 31), (9, 0, 0)))
            .expect("the coverage contract must answer a covered date")
    );
    // The neighbouring dates, which other waves audit, carry no row here.
    assert_eq!(calendar.holiday_on(day(2018, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = rates();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_interest_rates ships a table");

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
    let calendar = rates();
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
/// module, and the refusal each probe of it states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the side of the instant it moved from,
/// and a kind this family does not ship fails outright. The count tuple is the
/// era's shape as the block records it; the handwritten table below pins the
/// date set the counts cannot see. The instant-level half is now a refusal
/// fence: the whole era precedes the permanent 2025-01-01 floor, so the printed
/// open or close and the trade date each row carried are no longer observable
/// through the calendar.
#[test]
fn era_2013_2015_sweeps_every_shipped_row_kind_and_instant() {
    let venue = rates();
    let (mut closed, mut early, mut late, mut both) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = day(2013, 1, 1);
    while date <= day(2015, 12, 31) {
        if let Some(row) = venue.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::Closed => {
                    closed += 1;
                    assert!(
                        venue
                            .is_closed_trade_date(date, SessionKind::Both)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !venue
                            .is_open(ct_on(day_before(date), (17, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}: the eve leg is gone"
                    );
                }
                HolidayKind::EarlyClose { close_ssm } => {
                    early += 1;
                    let (h, m, s) = (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let cutoff = ct_on(date, (h, m, s));
                    assert!(
                        venue
                            .is_open(cutoff - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !venue
                            .is_open(cutoff)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}: end-exclusive"
                    );
                    assert_eq!(
                        venue
                            .trade_date(cutoff - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        Some(date),
                        "{date}"
                    );
                    assert_eq!(
                        venue
                            .candle_end(cutoff - Duration::minutes(1), CalendarResolution::Daily)
                            .expect("the coverage contract must answer a covered date"),
                        Some(cutoff),
                        "{date}"
                    );
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late += 1;
                    let (h, m, s) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let open = ct_on(date, (h, m, s));
                    assert!(
                        !venue
                            .is_open(open - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        venue
                            .is_open(open)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}: matching starts at the printed instant"
                    );
                    assert_eq!(
                        venue
                            .trade_date(open + Duration::hours(1))
                            .expect("the coverage contract must answer a covered date"),
                        Some(date),
                        "{date}: keyed to its own trade date"
                    );
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
                    assert!(
                        !venue
                            .is_open(open - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        venue
                            .is_open(open)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        venue
                            .is_open(cutoff - Duration::seconds(1))
                            .expect("the coverage contract must answer a covered date"),
                        "{date}"
                    );
                    assert!(
                        !venue
                            .is_open(cutoff)
                            .expect("the coverage contract must answer a covered date"),
                        "{date}: end-exclusive"
                    );
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
    let venue = rates();
    let coverage = venue
        .holiday_coverage()
        .expect("globex_interest_rates ships a table");
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
    let venue = rates();
    let coverage = venue
        .holiday_coverage()
        .expect("globex_interest_rates ships a table");

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
    let venue = rates();
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
