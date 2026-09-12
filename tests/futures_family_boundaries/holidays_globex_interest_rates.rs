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

use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key,
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

    assert!(calendar.is_closed_trade_date(day(2025, 12, 25), SessionKind::Both));
    // Three probes inside the civil day, none of them open.
    assert!(!calendar.is_open(ct((2025, 12, 25), (0, 30, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 25), (8, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 25), (12, 0, 0))));

    // The next trade date's leg opens inside the holiday's civil day, so the
    // civil day is not wholly closed. `is_closed_trade_date` is the holiday
    // question; `is_closed_all_day_on` is not.
    assert!(calendar.is_open(ct((2025, 12, 25), (18, 0, 0))));
    assert!(!calendar.is_closed_all_day_on(day(2025, 12, 25), SessionKind::Both));
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
    assert!(calendar.is_open(ct((2027, 3, 25), (9, 0, 0))));
    // Its evening leg fed the closed Friday and is gone.
    assert!(!calendar.is_open(ct((2027, 3, 25), (18, 0, 0))));
    assert!(!calendar.is_open(ct((2027, 3, 26), (9, 0, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2027, 3, 25), (12, 0, 0))),
        Some(ct((2027, 3, 28), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// 2 and 3. An early close, on both sides of the cutoff.
// ---------------------------------------------------------------------------

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
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        }
    );
    assert_eq!(holiday.document_id(), "CME-SVC-2025-11-26");

    // The instant before the close, and the close itself: closes are
    // end-exclusive.
    assert!(calendar.is_open(ct((2025, 11, 28), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2025, 11, 28), (12, 15, 0))));
    // The remainder of the trading day is gone, not merely quiet.
    assert!(!calendar.is_open(ct((2025, 11, 28), (14, 0, 0))));

    assert_eq!(
        calendar.session_bounds(ct((2025, 11, 28), (10, 0, 0))),
        Some((
            ct((2025, 11, 27), (17, 0, 0)),
            ct((2025, 11, 28), (12, 15, 0))
        ))
    );
    assert_eq!(
        calendar.candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2025, 11, 28), (12, 15, 0)))
    );
    // Friday hands over to the weekend, so the next open is Sunday evening.
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 11, 28), (12, 20, 0))),
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
            calendar.is_open(cutoff - chrono::TimeDelta::seconds(1)),
            "{label}: the instant before the early close must still be open"
        );
        assert!(
            !calendar.is_open(cutoff),
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
    assert!(calendar.is_closed_trade_date(day(2025, 11, 29), SessionKind::Both));
    assert!(!calendar.is_open(ct((2025, 11, 29), (10, 0, 0))));
    assert!(
        !calendar
            .without_holidays()
            .is_open(ct((2025, 11, 29), (10, 0, 0))),
        "the normal week has no Saturday session, so the row deletes nothing"
    );
    // The Sunday-evening reopen is untouched.
    assert!(calendar.is_open(ct((2025, 11, 30), (17, 0, 0))));
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
    assert!(!calendar.is_open(ct((2025, 12, 25), (16, 59, 59))));
    assert!(calendar.is_open(ct((2025, 12, 25), (17, 0, 0))));
    assert_eq!(calendar.holiday_on(day(2025, 12, 26)), None);

    // New Year's Day 2026, the same shape on a Thursday.
    assert!(!calendar.is_open(ct((2026, 1, 1), (16, 59, 59))));
    assert!(calendar.is_open(ct((2026, 1, 1), (17, 0, 0))));
    assert_eq!(calendar.holiday_on(day(2026, 1, 2)), None);

    // Every trade date the table names resolves to a closure or an early
    // close; none of them is a late open of either branch.
    for date in [
        day(2025, 1, 1),
        day(2025, 1, 20),
        day(2025, 12, 25),
        day(2026, 4, 3),
        day(2026, 12, 25),
        day(2027, 6, 18),
        day(2027, 12, 24),
    ] {
        let kind = calendar
            .holiday_on(date)
            .map(Holiday::kind)
            .expect("each named date ships a row");
        assert!(
            matches!(kind, HolidayKind::Closed | HolidayKind::EarlyClose { .. }),
            "{date} is not one of the two kinds this family ships: {kind:?}"
        );
    }
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
    assert!(calendar.is_open(ct((2025, 12, 24), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2025, 12, 24), (12, 15, 0))));
    // No session at 17:30 CT on Christmas Eve: that leg's trade date is closed.
    assert!(!calendar.is_open(ct((2025, 12, 24), (17, 0, 0))));
    assert!(!calendar.is_open(ct((2025, 12, 24), (17, 30, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 12, 24), (12, 20, 0))),
        Some(ct((2025, 12, 25), (17, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// 6. The trade-date consequence.
// ---------------------------------------------------------------------------

/// A shortened day keeps its own trade date, a deleted leg has none, and the
/// leg opening on a closed date already belongs to the next trade date.
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
        calendar.trade_date(ct((2025, 11, 28), (10, 0, 0))),
        Some(day(2025, 11, 28))
    );
    assert_eq!(calendar.trade_date(ct((2025, 12, 24), (17, 30, 0))), None);
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), (18, 0, 0))),
        Some(day(2025, 12, 26))
    );
    // The noon halt of a Monday holiday shortens that Monday; the 17:00 CT
    // open on the same civil day belongs to the next trade date.
    assert_eq!(
        calendar.trade_date(ct((2025, 1, 20), (9, 0, 0))),
        Some(day(2025, 1, 20))
    );
    assert!(calendar.is_open(ct((2025, 1, 20), (18, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2025, 1, 20), (18, 0, 0))),
        Some(day(2025, 1, 21))
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 19), (10, 0, 0))),
        Some(day(2026, 6, 19))
    );
}

// ---------------------------------------------------------------------------
// 7. Both edges of the coverage window.
// ---------------------------------------------------------------------------

/// Inside the window a date with no row is audited normal; outside it the table
/// has no answer, and a holiday it would otherwise have carried is not applied.
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
    assert!(calendar.is_open(ct((2026, 10, 22), (9, 0, 0))));

    // One day below the window, and a known CME closure below it: neither is
    // answered, and the normal week stands.
    assert_eq!(calendar.holiday_on(day(2024, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2024, 12, 25)), None);
    assert!(calendar.is_open(ct((2024, 12, 25), (9, 0, 0))));

    // One day above the window, and a known CME holiday above it: the table
    // does not silently extend.
    assert_eq!(calendar.holiday_on(day(2028, 1, 1)), None);
    assert_eq!(calendar.holiday_on(day(2028, 1, 17)), None);
    assert!(calendar.is_open(ct((2028, 1, 17), (9, 0, 0))));

    // The last audited trade date is an ordinary Friday.
    assert_eq!(calendar.holiday_on(day(2027, 12, 31)), None);
    assert!(calendar.is_open(ct((2027, 12, 31), (9, 0, 0))));
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
    assert!(!calendar.is_open(ct((2025, 12, 25), (9, 0, 0))));
    assert!(bare.is_open(ct((2025, 12, 25), (9, 0, 0))));
    // The wrap the closure removed is back.
    assert!(bare.is_open(ct((2025, 12, 24), (17, 30, 0))));
    // An early close: the normal Friday runs to its 16:00 CT close.
    assert!(!calendar.is_open(ct((2025, 11, 28), (14, 0, 0))));
    assert!(bare.is_open(ct((2025, 11, 28), (14, 0, 0))));
    assert_eq!(
        bare.trade_date(ct((2025, 12, 24), (17, 30, 0))),
        Some(day(2025, 12, 25))
    );
    // Away from the table's rows the two agree, which is what makes the
    // detach a control rather than a different calendar.
    assert_eq!(
        calendar.session_bounds(ct((2026, 10, 22), (9, 0, 0))),
        bare.session_bounds(ct((2026, 10, 22), (9, 0, 0)))
    );
}
