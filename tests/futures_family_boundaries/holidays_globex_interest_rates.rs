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

    // Over every audited window, every row is a closure, an early close or an
    // `Unsourced` statement, and the three late opens are the 2010-2012 ones;
    // the counts pin the shape of all five eras, so a row on a date no test
    // names fails here.
    let coverage = calendar
        .holiday_coverage()
        .expect("this family ships a table");
    let (mut closed, mut early, mut late, mut unsourced) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = coverage.first();
    while date <= coverage.last() {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
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
        (39, 120, 3, 6),
        "closed, early-close, late-open and unsourced rows over all five windows"
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

    // One day below the wave that audits this family next, and a known CME
    // closure below it: neither is answered, and the normal week stands. The
    // 2019-2021 interval joined the table when that wave shipped, so
    // 2020-12-25 — a silence probe before it — is now a shipped `Closed` row
    // and the unaudited probe is Christmas 2015.
    assert!(!coverage.contains(day(2015, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2015, 12, 31)), None);
    assert_eq!(calendar.holiday_on(day(2015, 12, 25)), None);
    assert!(calendar.is_open(ct((2015, 12, 25), (9, 0, 0))));
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

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening, and the operator's own 17:00 CT evening leg begins the next trade
/// date untouched.
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
        assert!(calendar.is_open(ct(previous_day, (17, 0, 0))), "{date:?}");
        assert!(
            calendar.is_open(cutoff - Duration::seconds(1)),
            "{date:?}: the second before the close is still open"
        );
        assert!(!calendar.is_open(cutoff), "{date:?}: end-exclusive");
        assert_eq!(
            calendar.candle_end(ct(date, (9, 0, 0)), CalendarResolution::Daily),
            Some(cutoff),
            "{date:?}"
        );
        assert_eq!(
            calendar.trade_date(ct(date, (9, 0, 0))),
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
        assert!(calendar.is_open(ct(date, (15, 0, 0))), "{date:?}");
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
#[test]
fn wave2_closures_remove_the_trade_date_and_ship_no_late_open() {
    let calendar = rates();

    assert_eq!(
        calendar.holiday_on(day(2016, 3, 25)).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(calendar.is_closed_trade_date(day(2016, 3, 25), SessionKind::Both));
    assert!(!calendar.is_open(ct((2016, 3, 24), (18, 0, 0))));
    assert!(!calendar.is_open(ct((2016, 3, 25), (10, 0, 0))));
    assert_eq!(
        calendar.next_session_open_after(ct((2016, 3, 25), (10, 0, 0))),
        Some(ct((2016, 3, 27), (17, 0, 0)))
    );
    assert_eq!(
        calendar.trade_date(ct((2016, 3, 27), (18, 0, 0))),
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
    assert!(!coverage.contains(day(2015, 12, 31)));
    // 2019-01-01 became the first date of a window of its own when the
    // 2019-2021 wave shipped: the only interval no wave audits is 2013-2015.
    assert!(coverage.contains(day(2019, 1, 1)));
    for date in [(2013, 6, 14), (2015, 12, 25)] {
        assert!(!coverage.contains(day(date.0, date.1, date.2)), "{date:?}");
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }
    // Christmas 2015 is a CME closure one week below the era: not applied.
    assert!(calendar.is_open(ct((2015, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2015, 12, 25), (10, 0, 0))),
        bare.is_open(ct((2015, 12, 25), (10, 0, 0)))
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
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
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

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = rates();
    let mut index = 0_usize;
    let (mut noons, mut quarter_past_noon) = (0_usize, 0);
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
                        other => panic!("{date}: the era ships no {other} CT close"),
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(
                        calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        "{date}"
                    );
                    assert!(
                        calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(calendar.is_open(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle.
                    assert_eq!(
                        calendar.session_bounds(ct_on(date, (9, 0, 0))),
                        Some((ct_on(day_before(date), (17, 0, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(ct_on(date, (9, 0, 0)), CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - Duration::seconds(1)),
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
        (noons, quarter_past_noon, closures, unsourced),
        (19, 4, 7, 3),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
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
            calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
        // The evening leg that would have carried this trade date is gone.
        assert!(
            !calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
            "{date}"
        );
        assert!(
            !calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
            "{date}"
        );
        // And so is the trade date's own civil day.
        assert!(!calendar.is_open(ct_on(date, (9, 0, 0))), "{date}");
        assert!(!calendar.is_open(ct_on(date, (15, 59, 0))), "{date}");
        assert_eq!(calendar.trade_date(ct_on(date, (10, 0, 0))), None, "{date}");

        let reopen = era_reopen_after_closure(date);
        assert_eq!(
            calendar.next_session_open_after(ct_on(date, (10, 0, 0))),
            Some(reopen),
            "{date}: the next session is the ordinary evening open"
        );
        if reopen == ct_on(date, (17, 0, 0)) {
            assert_eq!(calendar.trade_date(reopen), Some(day_after(date)), "{date}");
        }
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = rates();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    // An `Unsourced` row closes nothing. 2021-06-19 is a Saturday, which the
    // grid has no session on whether or not a row exists, so the strict claim
    // is made where the family could trade and the equality claim where it
    // could not.
    if matches!(date.weekday(), Weekday::Sat | Weekday::Sun) {
        assert_eq!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            detached.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
    } else {
        assert!(
            !calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date} must not be reported closed"
        );
    }

    for probe in [
        ct_on(day_before(date), (18, 0, 0)),
        ct_on(date, (9, 0, 0)),
        ct_on(date, (15, 59, 0)),
        ct_on(date, (18, 0, 0)),
    ] {
        assert_eq!(calendar.is_open(probe), detached.is_open(probe), "{probe}");
        assert_eq!(
            calendar.trade_date(probe),
            detached.trade_date(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.session_bounds(probe),
            detached.session_bounds(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.next_session_open_after(probe),
            detached.next_session_open_after(probe),
            "{probe}"
        );
        assert_eq!(
            calendar.candle_end(probe, CalendarResolution::Daily),
            detached.candle_end(probe, CalendarResolution::Daily),
            "{probe}"
        );
    }
}

#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = rates();
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date.0, date.1, date.2);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_unsourced_changes_nothing(date, row, EvidenceTier::T2);
    }
}

/// The 2022-2024 window sits fourth in the declared coverage, its edges
/// answer, and the 2013-2015 interval below the 2016-2018 wave stays
/// unaudited. (The 2019-2021 interval this test used to fence became a window
/// of its own when that wave shipped; the section below fences it.)
#[test]
fn era_2022_2024_window_sits_fourth_and_the_2013_2015_interval_is_unaudited() {
    let calendar = rates();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_interest_rates ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2010, 1, 1), day(2012, 12, 31)),
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

    for date in [(2013, 6, 14), (2015, 12, 25)] {
        assert!(!coverage.contains(day(date.0, date.1, date.2)), "{date:?}");
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }
    // Christmas 2015 is a real CME closure no wave audited; the ordinary
    // Friday answers, and the detached calendar answers it the same.
    for probe in [
        ct((2015, 12, 25), (10, 0, 0)),
        ct((2015, 12, 24), (18, 0, 0)),
    ] {
        assert!(calendar.is_open(probe), "{probe}");
        assert_eq!(calendar.is_open(probe), bare.is_open(probe), "{probe}");
    }
}

/// The era's closes are the financial families' 12:00 CT on the Monday and
/// Thursday holidays and 12:15 CT on the Thanksgiving Fridays and the two
/// Christmas Eves — checked against the energy venue, which keeps trading to
/// 13:30 CT on the same holiday, so a copied instant fails here.
#[test]
fn era_2022_2024_close_instants_are_the_financial_ones_not_the_energy_venue() {
    let rates = rates();
    let energy = calendar_for_market_hours_key(MarketHoursKey::GlobexEnergy);

    // 2022-01-17: rates stop at 12:00 CT; energy trades on to 13:30 CT.
    assert!(rates.is_open(ct((2022, 1, 17), (11, 59, 59))));
    assert!(!rates.is_open(ct((2022, 1, 17), (12, 0, 0))));
    assert!(energy.is_open(ct((2022, 1, 17), (12, 0, 0))));

    // 2022-11-25: the Thanksgiving Friday closes at 12:15 CT here, 12:45 CT
    // there.
    assert!(rates.is_open(ct((2022, 11, 25), (12, 14, 59))));
    assert!(!rates.is_open(ct((2022, 11, 25), (12, 15, 0))));
    assert!(energy.is_open(ct((2022, 11, 25), (12, 15, 0))));
    assert!(!energy.is_open(ct((2022, 11, 25), (12, 45, 0))));

    // 2024-12-24: 12:15 CT here, 12:45 CT there.
    assert!(rates.is_open(ct((2024, 12, 24), (12, 14, 59))));
    assert!(!rates.is_open(ct((2024, 12, 24), (12, 15, 0))));
    assert!(energy.is_open(ct((2024, 12, 24), (12, 15, 0))));
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// 10:15 CT, the one Good Friday 2021 close this era prints.
const ERA_TEN_FIFTEEN: u32 = 10 * 3_600 + 15 * 60;

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. An early
/// close must be open one second before its printed instant and closed at it;
/// a closure must take the trade date and the leg that opened it.
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
                        calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        "{date}"
                    );
                    assert!(
                        calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), (18, 0, 0))),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(calendar.is_open(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle. The probe sits just inside the
                    // session, so the 10:15 Good Friday close cannot make the
                    // query answer `None` instead.
                    let inside = cutoff - Duration::minutes(1);
                    assert_eq!(
                        calendar.session_bounds(inside),
                        Some((ct_on(day_before(date), (17, 0, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(inside, CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - Duration::seconds(1)),
                        Some(date),
                        "{date}"
                    );
                }
                HolidayKind::Closed => {
                    closures += 1;
                    assert!(
                        calendar.is_closed_trade_date(date, SessionKind::Both),
                        "{date}"
                    );
                    // The evening leg that would have carried this trade date
                    // is gone, and so is the trade date's own session.
                    assert!(
                        !calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        "{date}"
                    );
                    assert!(
                        !calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        "{date}"
                    );
                    assert!(!calendar.is_open(ct_on(date, (9, 0, 0))), "{date}");
                    assert!(!calendar.is_open(ct_on(date, (15, 59, 0))), "{date}");
                    assert_eq!(calendar.trade_date(ct_on(date, (10, 0, 0))), None, "{date}");
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
    assert!(calendar.is_open(ct((2021, 12, 31), (9, 0, 0))));
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
