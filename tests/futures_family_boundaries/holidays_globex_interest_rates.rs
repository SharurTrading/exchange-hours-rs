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

use chrono::{DateTime, Days, NaiveDate, TimeZone as _, Utc};
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

    // Over the whole coverage window, every row is a closure or an early
    // close and none is a late open of either branch; the counts pin the
    // table's shape, so a row on a date no test names fails here.
    let coverage = calendar
        .holiday_coverage()
        .expect("this family ships a table");
    let (mut closed, mut early, mut late) = (0_usize, 0_usize, 0_usize);
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { .. }) => early += 1,
            Some(HolidayKind::LateOpen { .. }) => late += 1,
            Some(other) => {
                panic!("{date} is not one of the kinds this family ships: {other:?}")
            }
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }
    assert_eq!(
        (closed, early, late),
        (9, 51, 3),
        "closed, early-close and late-open rows over the whole 2010-2027 window"
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
    assert_eq!(coverage.first(), day(2010, 1, 1));
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

// ---------------------------------------------------------------------------
// 9. The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 17:30 CT, the family's normal evening first open to 2012-04-30.
const ERA_EVENING_OPEN: u32 = 17 * 3_600 + 30 * 60;
/// 05:00 CT, the era's post-holiday first open.
const ERA_REOPEN: u32 = 5 * 3_600;

/// The era's early closes clip a trading day that opened 17:30 CT the previous
/// evening, so each cutoff has to land on its own trade date and delete the
/// whole remaining session — including the 15:15 CT Martin Luther King Jr. Day
/// close, the 10:15 CT Good Friday close and the 12:15 CT New Year's Eve close.
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
        assert!(calendar.is_open(ct(previous_day, (17, 30, 0))));
        // One second before the close, and at it: closes are end-exclusive.
        let before = ct(
            date,
            (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
        ) - chrono::TimeDelta::seconds(1);
        assert!(
            calendar.is_open(before),
            "{date:?}: {close_ssm} is too early"
        );
        assert!(
            !calendar.is_open(ct(
                date,
                (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60)
            )),
            "{date:?}: {close_ssm} is not end-exclusive"
        );
        // The whole trading day ends there, and it is still the trade date's
        // own session that ends.
        assert_eq!(
            calendar.session_bounds(ct(date, (9, 0, 0))),
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
        calendar.next_session_open_after(ct((2010, 1, 15), (16, 0, 0))),
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
    assert!(calendar.is_open(ct((2011, 1, 2), (17, 30, 0))));
    assert_eq!(
        calendar.trade_date(ct((2011, 1, 2), (18, 0, 0))),
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
    assert!(!calendar.is_open(ct((2012, 1, 2), (17, 0, 0))));
    assert!(!calendar.is_open(ct((2012, 1, 3), (4, 59, 59))));
    assert!(calendar.is_open(ct((2012, 1, 3), (5, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2012, 1, 3), (5, 0, 0))),
        Some((ct((2012, 1, 3), (5, 0, 0)), ct((2012, 1, 3), (16, 0, 0))))
    );
    assert_eq!(
        calendar.trade_date(ct((2012, 1, 3), (9, 0, 0))),
        Some(day(2012, 1, 3))
    );
}

/// The widened window's edges answer as the module declares: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// not applied and the detached calendar agreeing.
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
    assert!(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        bare.is_open(ct((2009, 12, 25), (10, 0, 0)))
    );
}
