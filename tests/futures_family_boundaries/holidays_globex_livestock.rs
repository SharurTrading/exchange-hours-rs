// SPDX-License-Identifier: MIT-0

//! `globex_livestock`'s built-in holiday rows, 2025-01-01 .. 2027-12-31.
//!
//! Every probe is stated in `America/Chicago` wall clock and converted, so a
//! DST slip in either direction fails rather than passing on a coincidence.
//!
//! The family's grid is flat: from 2016-02-29 CME Live Cattle, Feeder Cattle
//! and Lean Hogs run one Monday-Friday 08:30-13:05 CT regular session, queued
//! by an 08:00-08:30 CT Pre-Open and followed by a 14:30-16:00 CT Post-Close
//! that already carries the **next** trade date. Nothing wraps a midnight. Two
//! of the design memo's seven per-family cases therefore have no positive
//! instance in this window and are discharged as negatives, each in its own
//! test with the reason stated:
//!
//! * **a late open** — CME publishes none for this family in 2025-2027. The
//!   whole table is 31 `Closed` rows and 5 `EarlyClose` rows, which
//!   [`the_table_ships_no_late_open_in_this_window`] proves by walking every
//!   date of the coverage window.
//! * **a wrap removed by a closure** — the grid has no wrapping occurrence, so
//!   the only phase a closure reaches back into on the previous civil day is
//!   the Post-Close queue that carried the closed trade date.
//!   [`a_closure_removes_the_previous_days_post_close_queue`] is that case.

use chrono::{DateTime, Datelike as _, Days, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key,
};

const LE: MarketHoursKey = MarketHoursKey::GlobexLivestock;

/// The declared coverage window, as the module states it.
const FIRST: (i32, u32, u32) = (2010, 1, 1);
const LAST: (i32, u32, u32) = (2027, 12, 31);

/// A probe instant stated in the venue's own wall clock.
fn ct(date: (i32, u32, u32), time: (u32, u32, u32)) -> DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous Central instant")
        .with_timezone(&Utc)
}

fn day(date: (i32, u32, u32)) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn calendar() -> ExchangeCalendar {
    calendar_for_market_hours_key(LE)
}

/// The same calendar with the built-in table detached — the A/B control.
fn bare() -> ExchangeCalendar {
    calendar().without_holidays()
}

/// Case 1: a full closure removes the whole trade date, and the crate agrees
/// with CME's own "no events published" for Christmas Day 2025.
///
/// This grid has no wrapping occurrence, so the civil day is closed outright —
/// which is exactly where `globex_livestock` differs from the wrapping Globex
/// families, and why `is_closed_all_day_on` is asserted here rather than
/// merely `is_closed_trade_date`.
#[test]
fn a_closed_trade_date_removes_the_whole_civil_day() {
    let calendar = calendar();
    let christmas = day((2025, 12, 25));

    assert_eq!(
        calendar.holiday_on(christmas).map(Holiday::kind),
        Some(HolidayKind::Closed),
        "CME publishes no events for Livestock on 2025-12-25"
    );
    let row = calendar
        .holiday_on(christmas)
        .expect("2025-12-25 is a shipped row");
    assert_eq!(row.tier(), EvidenceTier::T2);
    assert_eq!(row.document_id(), "CME-SVC-2025-12-24");

    assert!(calendar.is_closed_trade_date(christmas, SessionKind::Both));
    assert!(calendar.is_closed_all_day_on(christmas, SessionKind::Both));

    // Three probes inside the civil day: the Pre-Open, the regular session and
    // the Post-Close. Nothing matches at any of them.
    for time in [(8, 15, 0), (10, 0, 0), (15, 0, 0)] {
        let instant = ct((2025, 12, 25), time);
        assert!(!calendar.is_open(instant), "still open at {time:?}");
    }
    // The holiday's own Pre-Open queue belongs to the closed trade date and
    // goes with it.
    assert!(!calendar.is_accepting_orders(ct((2025, 12, 25), (8, 15, 0))));

    // The declared order-entry gap, fenced so it stays visible: the
    // 14:30-16:00 CT Post-Close on the holiday already carries Friday's trade
    // date, so a `Closed` row on 2025-12-25 cannot reach it and the crate
    // still reports order entry there. CME published no events at all that
    // day. `DayPolicy`'s scalar vocabulary has no order-entry boundary, so
    // this is recorded as a gap in docs/evidence/globex_livestock.md rather
    // than modelled. It changes no `is_open` answer.
    assert!(calendar.is_accepting_orders(ct((2025, 12, 25), (15, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), (15, 0, 0))),
        Some(day((2025, 12, 26)))
    );

    // From just after the eve's own early close, the next session is Friday's
    // normal 08:30 CT open: Christmas Day is skipped entirely.
    assert_eq!(
        calendar.next_session_open_after(ct((2025, 12, 24), (12, 20, 0))),
        Some(ct((2025, 12, 26), (8, 30, 0)))
    );
    // Detaching the table restores the normal Thursday.
    assert!(bare().is_open(ct((2025, 12, 25), (10, 0, 0))));
}

/// Case 2 and 3: the early close, one second before it and at it.
///
/// The day after Thanksgiving 2025 is CME's `12:05 closed` for Livestock
/// against a normal 13:05 CT regular close. The 08:30 CT open does **not**
/// move, the regular remainder disappears, and the trading day now ends at the
/// cutoff.
#[test]
fn an_early_close_ends_the_trading_day_at_the_stated_instant() {
    let calendar = calendar();
    let friday = day((2025, 11, 28));

    assert_eq!(
        calendar.holiday_on(friday).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 5 * 60
        })
    );

    // The first open is untouched, and still end-exclusive on its own side.
    assert!(!calendar.is_open(ct((2025, 11, 28), (8, 29, 59))));
    assert!(calendar.is_open(ct((2025, 11, 28), (8, 30, 0))));

    // One second before the cutoff, and at it: closes are end-exclusive.
    assert!(calendar.is_open(ct((2025, 11, 28), (12, 4, 59))));
    assert!(!calendar.is_open(ct((2025, 11, 28), (12, 5, 0))));
    // The rest of the normal 08:30-13:05 CT session is gone.
    assert!(!calendar.is_open(ct((2025, 11, 28), (13, 0, 0))));

    assert_eq!(
        calendar.session_bounds(ct((2025, 11, 28), (10, 0, 0))),
        Some((
            ct((2025, 11, 28), (8, 30, 0)),
            ct((2025, 11, 28), (12, 5, 0))
        ))
    );
    assert_eq!(
        calendar.candle_end(ct((2025, 11, 28), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2025, 11, 28), (12, 5, 0)))
    );

    // The second half of the declared order-entry gap: CME publishes no
    // `14:30 pcp` on this date either, but that window carries Monday's trade
    // date, so the early close cannot reach it. Again no `is_open` change.
    assert!(calendar.is_accepting_orders(ct((2025, 11, 28), (14, 30, 0))));
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), (14, 30, 0))),
        Some(day((2025, 12, 1)))
    );

    // Without the table the same instants are an ordinary Friday.
    assert!(bare().is_open(ct((2025, 11, 28), (12, 5, 0))));
    assert!(bare().is_open(ct((2025, 11, 28), (13, 0, 0))));
}

/// The two Christmas Eves carry **different** instants, so neither can be a
/// copy of the other: 12:15 CT in 2025, 12:05 CT in 2026.
///
/// This is the mutation fence for the early-close instants — perturbing either
/// row by one minute fails here.
#[test]
fn each_early_close_carries_its_own_sourced_instant() {
    let calendar = calendar();

    assert!(calendar.is_open(ct((2025, 12, 24), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2025, 12, 24), (12, 15, 0))));
    assert_eq!(
        calendar.candle_end(ct((2025, 12, 24), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2025, 12, 24), (12, 15, 0)))
    );

    assert!(calendar.is_open(ct((2026, 12, 24), (12, 4, 59))));
    assert!(!calendar.is_open(ct((2026, 12, 24), (12, 5, 0))));
    assert_eq!(
        calendar.candle_end(ct((2026, 12, 24), (10, 0, 0)), CalendarResolution::Daily),
        Some(ct((2026, 12, 24), (12, 5, 0)))
    );
    // 12:15 CT would still be open on the 2026 eve if the rows were copied.
    assert!(!calendar.is_open(ct((2026, 12, 24), (12, 14, 59))));
}

/// Case 4, as a negative: no row in this window moves a first open.
///
/// A late open is the one kind whose branch choice is data-dependent and
/// silently 24 hours wrong if it flips, so its absence is asserted rather than
/// assumed. The walk also fixes the table's shape over every audited window:
/// 73 closures, 16 early closes, the four 2010-2012 late opens and the three
/// 2023 `Unsourced` statements, nothing else.
#[test]
fn the_table_ships_no_late_open_in_this_window() {
    let calendar = calendar();
    let mut closed = 0_usize;
    let mut early = 0_usize;
    let mut late = 0_usize;
    let mut unsourced = 0_usize;
    let mut date = day(FIRST);
    let last = day(LAST);

    while date <= last {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { .. }) => early += 1,
            Some(HolidayKind::LateOpen { .. }) => late += 1,
            Some(HolidayKind::Unsourced) => unsourced += 1,
            Some(other) => panic!("{date} ships an unexpected holiday kind: {other:?}"),
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }

    assert_eq!(closed, 73, "closed rows over every audited window");
    assert_eq!(early, 16, "early-close rows over every audited window");
    assert_eq!(late, 4, "late-open rows, 2010-2012");
    assert_eq!(
        unsourced, 3,
        "unsourced rows, the 2023 dates no document covers"
    );
    assert!(
        !calendar
            .holiday_on(day((2024, 11, 29)))
            .is_some_and(|row| matches!(row.kind(), HolidayKind::LateOpen { .. })),
        "the 2022-2024 window ships no late open"
    );

    // Every early close keeps the family's normal 08:30 CT first open.
    for eve in [
        (2025, 11, 28),
        (2025, 12, 24),
        (2026, 11, 27),
        (2026, 12, 24),
        (2027, 11, 26),
    ] {
        assert!(
            !calendar.is_open(ct(eve, (8, 29, 59))),
            "{eve:?} opened early"
        );
        assert!(calendar.is_open(ct(eve, (8, 30, 0))), "{eve:?} opened late");
    }
}

/// Case 5: the closure reaches back into the previous civil day.
///
/// `globex_livestock` has no evening leg, so a closure cannot delete a wrap —
/// but the 14:30-16:00 CT Post-Close queue of the day before already carries
/// the closed trade date, and it goes with it. Thursday 2027-12-23 is audited
/// normal and trades in full; its queue does not run, because Friday
/// 2027-12-24 is closed.
#[test]
fn a_closure_removes_the_previous_days_post_close_queue() {
    let calendar = calendar();

    assert_eq!(calendar.holiday_on(day((2027, 12, 23))), None);
    assert_eq!(
        calendar.holiday_on(day((2027, 12, 24))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );

    // The Thursday itself is an ordinary trading day to its 13:05 CT close.
    assert!(calendar.is_open(ct((2027, 12, 23), (13, 4, 59))));
    assert!(!calendar.is_open(ct((2027, 12, 23), (13, 5, 0))));

    // Its Post-Close queue is gone: it belonged to trade date 2027-12-24.
    assert!(!calendar.is_accepting_orders(ct((2027, 12, 23), (14, 30, 0))));
    assert!(bare().is_accepting_orders(ct((2027, 12, 23), (14, 30, 0))));

    // So the next session is Monday's open, not Friday's.
    assert_eq!(
        calendar.next_session_open_after(ct((2027, 12, 23), (13, 10, 0))),
        Some(ct((2027, 12, 27), (8, 30, 0)))
    );
    assert_eq!(
        bare().next_session_open_after(ct((2027, 12, 23), (13, 10, 0))),
        Some(ct((2027, 12, 24), (8, 30, 0)))
    );
}

/// Case 6: what the rows do to the trade date.
///
/// Inside a shortened day the trade date is still the holiday's own; at the
/// eve's Post-Close queue it is the *following* trade date, which is how a
/// closure reaches backwards at all; and once a date is closed nothing is
/// assigned to it — this family has `has_weekend_close`, so the
/// following-business-day roll does not apply and the queue simply disappears.
#[test]
fn the_rows_reach_the_trade_date() {
    let calendar = calendar();

    // A shortened day keeps its own trade date, up to the cutoff.
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), (10, 0, 0))),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(calendar.trade_date(ct((2025, 11, 28), (12, 5, 0))), None);

    // The eve's Post-Close queue carries the next trade date.
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 23), (15, 0, 0))),
        Some(day((2025, 12, 24)))
    );
    // And on Christmas Eve that next trade date is closed, so nothing is
    // assigned and nothing rolls forward onto 2025-12-26.
    assert_eq!(calendar.trade_date(ct((2025, 12, 24), (15, 0, 0))), None);
    assert_eq!(
        bare().trade_date(ct((2025, 12, 24), (15, 0, 0))),
        Some(day((2025, 12, 25)))
    );
}

/// The sourced Saturday row is honest and answer-neutral.
///
/// CME's live service answers for Saturday 2025-11-29 with an empty schedule
/// for every Livestock product, so the date is a sourced `Closed` rather than a
/// gap. The family never trades a Saturday, so the row changes no answer — and
/// that is asserted, not assumed.
#[test]
fn the_sourced_saturday_row_changes_no_answer() {
    let calendar = calendar();
    let saturday = day((2025, 11, 29));

    assert_eq!(
        calendar.holiday_on(saturday).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(calendar.is_closed_trade_date(saturday, SessionKind::Both));
    assert!(bare().is_closed_trade_date(saturday, SessionKind::Both));
    assert!(!calendar.is_open(ct((2025, 11, 29), (10, 0, 0))));
    assert!(!bare().is_open(ct((2025, 11, 29), (10, 0, 0))));
}

/// Case 7: both edges of the coverage window.
///
/// Inside the window a date with no row is audited normal; outside it the
/// table has no answer at all, and a real CME holiday one step outside is
/// **not** applied — Christmas 2024 and Martin Luther King Jr. Day 2028 both
/// trade, exactly as they do with the table detached.
#[test]
fn the_table_answers_only_inside_its_coverage_window() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_livestock ships a table");

    assert_eq!(coverage.first(), day(FIRST));
    assert_eq!(coverage.last(), day(LAST));
    assert!(coverage.contains(day((2026, 7, 3))));
    assert!(!coverage.contains(day((2009, 12, 25))));
    assert!(!coverage.contains(day((2028, 1, 17))));

    assert_eq!(calendar.holiday_on(day((2009, 12, 31))), None);
    assert_eq!(calendar.holiday_on(day((2028, 1, 1))), None);

    // Christmas Day 2009 is a year below the 2010-2012 window that opens this
    // table; it trades under the normal week, and the detached calendar agrees.
    // (Christmas 2024, which this probe's comment used to name, is now a
    // shipped `Closed` row inside the 2022-2024 window.)
    assert!(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        bare().is_open(ct((2009, 12, 25), (10, 0, 0)))
    );
    // MLK 2028 is a Monday two and a half weeks after it closes.
    assert!(calendar.is_open(ct((2028, 1, 17), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2028, 1, 17), (10, 0, 0))),
        bare().is_open(ct((2028, 1, 17), (10, 0, 0)))
    );

    // In coverage with no row means audited normal, not unknown: Thursday
    // 2027-12-23 is CME's own normal Livestock grid and trades in full.
    assert_eq!(calendar.holiday_on(day((2027, 12, 23))), None);
    assert!(calendar.is_open(ct((2027, 12, 23), (8, 30, 0))));
    assert!(calendar.is_open(ct((2027, 12, 23), (13, 4, 59))));
}

/// `without_holidays` restores the normal-week answer, row for row.
///
/// The detached calendar reports no table and no rows, and every instant a row
/// changes goes back to what the normal week says.
#[test]
fn without_holidays_restores_the_normal_week() {
    let calendar = calendar();
    let bare = bare();

    assert_eq!(bare.holiday_coverage(), None);
    assert_eq!(bare.holiday_on(day((2025, 12, 25))), None);
    assert_eq!(bare.holiday_on(day((2025, 11, 28))), None);

    // Every closure this family ships is a weekday closure except the sourced
    // Saturday, so the detached calendar trades each of them 08:30-13:05 CT.
    for closure in [
        (2025, 1, 1),
        (2025, 7, 4),
        (2026, 4, 3),
        (2026, 12, 25),
        (2027, 3, 26),
        (2027, 12, 24),
    ] {
        assert!(!calendar.is_open(ct(closure, (10, 0, 0))), "{closure:?}");
        assert!(bare.is_open(ct(closure, (10, 0, 0))), "{closure:?}");
        assert_eq!(
            bare.session_bounds(ct(closure, (10, 0, 0))),
            Some((ct(closure, (8, 30, 0)), ct(closure, (13, 5, 0)))),
            "{closure:?}"
        );
    }

    // And every early close goes back to the normal 13:05 CT final close.
    for eve in [
        (2025, 11, 28),
        (2025, 12, 24),
        (2026, 11, 27),
        (2026, 12, 24),
        (2027, 11, 26),
    ] {
        assert_eq!(
            bare.session_bounds(ct(eve, (10, 0, 0))),
            Some((ct(eve, (8, 30, 0)), ct(eve, (13, 5, 0)))),
            "{eve:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 13:55 CT, the era's Good Friday final close.
const ERA_GOOD_FRIDAY_CLOSE: u32 = 13 * 3_600 + 55 * 60;

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening: 13:55 CT on the Good Friday eves of 2011 and 2012, 12:00 CT on the
/// day after Thanksgiving 2010, and 12:15 CT on the later half-days.
#[test]
fn era_early_closes_end_the_wrapped_trading_day_at_the_stated_instant() {
    let calendar = calendar();
    let eves = [
        ((2011, 4, 21), (2011, 4, 20), ERA_GOOD_FRIDAY_CLOSE),
        ((2012, 4, 5), (2012, 4, 4), ERA_GOOD_FRIDAY_CLOSE),
        ((2010, 11, 26), (2010, 11, 25), 12 * 3_600),
        ((2010, 12, 31), (2010, 12, 30), 12 * 3_600 + 15 * 60),
        ((2011, 11, 25), (2011, 11, 24), 12 * 3_600 + 15 * 60),
        ((2012, 11, 23), (2012, 11, 22), 12 * 3_600 + 15 * 60),
    ];
    for (date, previous_day, close_ssm) in eves {
        let row = calendar
            .holiday_on(day(date))
            .unwrap_or_else(|| panic!("{date:?} ships a row"));
        assert_eq!(
            row.kind(),
            HolidayKind::EarlyClose { close_ssm },
            "{date:?}"
        );
        assert_eq!(row.tier(), EvidenceTier::T1, "{date:?}");

        // The evening leg that feeds this trade date is clipped, not deleted.
        assert!(calendar.is_open(ct(previous_day, (17, 0, 0))), "{date:?}");
        assert!(
            calendar.is_open(
                ct(
                    date,
                    (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60)
                ) - chrono::TimeDelta::seconds(1)
            ),
            "{date:?}: {close_ssm} is too early"
        );
        assert!(
            !calendar.is_open(ct(
                date,
                (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60)
            )),
            "{date:?}: {close_ssm} is not end-exclusive"
        );
        assert_eq!(
            calendar.session_bounds(ct(date, (9, 0, 0))),
            Some((
                ct(previous_day, (17, 0, 0)),
                ct(
                    date,
                    (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60)
                )
            )),
            "{date:?}"
        );
    }

    // The Good Friday 2011 early close is probed in full: 13:55 CT ends the
    // trade date, and the Thursday-evening leg is the next thing the Good
    // Friday 2011 closure removes.
    assert!(calendar.is_open(ct((2011, 4, 21), (13, 54, 59))));
    assert!(!calendar.is_open(ct((2011, 4, 21), (13, 55, 0))));
    assert_eq!(calendar.trade_date(ct((2011, 4, 21), (13, 55, 0))), None);
    assert_eq!(
        calendar
            .holiday_on(day((2011, 4, 22)))
            .expect("2011-04-22 ships a row")
            .kind(),
        HolidayKind::Closed
    );
    assert!(!calendar.is_open(ct((2011, 4, 21), (18, 0, 0))));

    // Christmas Eve 2012 falls on a Monday, whose trade date has no
    // Sunday-evening leg on this era's grid: it opens at the era's own 09:05
    // CT Monday session and is clipped at 12:15 CT.
    assert_eq!(
        calendar
            .holiday_on(day((2012, 12, 24)))
            .expect("2012-12-24 ships a row")
            .kind(),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        }
    );
    assert!(!calendar.is_open(ct((2012, 12, 23), (17, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2012, 12, 24), (10, 0, 0))),
        Some((
            ct((2012, 12, 24), (9, 5, 0)),
            ct((2012, 12, 24), (12, 15, 0))
        ))
    );
}

/// Every late open the era ships states 09:05 CT — earlier than the family's
/// normal 17:00 CT first open — so each cutoff lands on the trade date itself:
/// the Monday-evening leg that would have opened trade date 2011-12-27 did not
/// run, and the trade date begins on its own civil day.
#[test]
fn era_late_opens_land_on_the_trade_date_itself() {
    let calendar = calendar();
    let reopen = 9 * 3_600 + 5 * 60;

    for (year, month, date) in [(2011, 12, 27), (2012, 1, 3), (2012, 7, 5), (2012, 12, 26)] {
        let row = calendar
            .holiday_on(day((year, month, date)))
            .unwrap_or_else(|| panic!("{year}-{month:02}-{date:02} ships a row"));
        assert_eq!(
            row.kind(),
            HolidayKind::LateOpen { open_ssm: reopen },
            "{year}-{month:02}-{date:02}"
        );
    }

    assert!(!calendar.is_open(ct((2011, 12, 26), (17, 0, 0))));
    assert!(!calendar.is_open(ct((2011, 12, 27), (9, 4, 59))));
    assert!(calendar.is_open(ct((2011, 12, 27), (9, 5, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2011, 12, 27), (9, 5, 0))),
        Some((
            ct((2011, 12, 27), (9, 5, 0)),
            ct((2011, 12, 27), (16, 0, 0))
        ))
    );
    assert_eq!(
        calendar.trade_date(ct((2011, 12, 27), (12, 0, 0))),
        Some(day((2011, 12, 27)))
    );
}

/// Good Friday 2010 is a **full closure** for this family: CME's 2010 sheet
/// names livestock among the products that stay closed until their regularly
/// scheduled Monday open, so the crate removes the trade date and the
/// Thursday-evening leg that fed it.
#[test]
fn era_good_friday_2010_is_a_full_closure() {
    let calendar = calendar();

    assert_eq!(
        calendar
            .holiday_on(day((2010, 4, 2)))
            .expect("2010-04-02 ships a row")
            .kind(),
        HolidayKind::Closed
    );
    assert!(!calendar.is_open(ct((2010, 4, 1), (17, 0, 0))));
    assert!(!calendar.is_open(ct((2010, 4, 2), (9, 5, 0))));
    assert_eq!(calendar.trade_date(ct((2010, 4, 2), (9, 5, 0))), None);
    // Monday 2010-04-05 opens normally and carries its own trade date.
    assert_eq!(
        calendar.trade_date(ct((2010, 4, 5), (9, 5, 0))),
        Some(day((2010, 4, 5)))
    );

    // A plain trading day in the same year answers the normal week.
    assert_eq!(calendar.holiday_on(day((2010, 6, 15))), None);
    assert_eq!(
        calendar.session_bounds(ct((2010, 6, 15), (9, 0, 0))),
        bare().session_bounds(ct((2010, 6, 15), (9, 0, 0)))
    );
}

/// The widened window's edges answer as the module declares: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// not applied and the detached calendar agreeing.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_livestock ships a table");

    assert_eq!(coverage.first(), day(FIRST));
    assert_eq!(coverage.last(), day(LAST));
    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(!coverage.contains(day((2009, 12, 25))));

    assert_eq!(calendar.holiday_on(day((2009, 12, 25))), None);
    assert!(calendar.is_open(ct((2009, 12, 25), (10, 0, 0))));
    assert_eq!(
        calendar.is_open(ct((2009, 12, 25), (10, 0, 0))),
        bare().is_open(ct((2009, 12, 25), (10, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 12:05 CT, the era's Thanksgiving Friday close.
const ERA_FIVE_PAST_NOON: u32 = 12 * 3_600 + 5 * 60;
/// 12:15 CT, the era's Christmas Eve close.
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
    ((2022, 1, 17), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 2, 21), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 5, 30), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 6, 20), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 9, 5), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 11, 24), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: ERA_FIVE_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2022, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 1, 16), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 2, 20), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 4, 7), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2023, 5, 29), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 6, 19), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 9, 4), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 11, 23), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_FIVE_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 15), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 2, 19), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 5, 27), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 6, 19), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 7, 4), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 9, 2), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 11, 28), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: ERA_FIVE_PAST_NOON,
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

/// The first 08:30 CT open after a closed trade date. This family's session
/// lies inside one civil day, so the reopen is the next business day's own
/// morning, never an evening leg.
fn era_reopen_after_closure(date: NaiveDate) -> DateTime<Utc> {
    let mut reopen = day_after(date);
    while matches!(reopen.weekday(), Weekday::Sat | Weekday::Sun) {
        reopen = day_after(reopen);
    }
    ct_on(reopen, (8, 30, 0))
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = calendar();
    let mut index = 0_usize;
    let (mut five_past_noon, mut quarter_past_noon) = (0_usize, 0);
    let (mut closures, mut unsourced) = (0_usize, 0);
    let mut date = day((2022, 1, 1));
    while date <= day((2024, 12, 31)) {
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
                        ERA_FIVE_PAST_NOON => five_past_noon += 1,
                        ERA_QUARTER_PAST_NOON => quarter_past_noon += 1,
                        other => panic!("{date}: the era ships no {other} CT close"),
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // This grid does not wrap: the previous evening carries no
                    // leg of this trade date at all.
                    assert!(
                        !calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
                        "{date}"
                    );
                    assert!(
                        !calendar.is_open(ct_on(day_before(date), (19, 30, 0))),
                        "{date}"
                    );
                    // The day session opens at 08:30 CT on its own civil date.
                    assert!(calendar.is_open(ct_on(date, (8, 30, 0))), "{date}");
                    // One second before the close is open; at it, closed.
                    assert!(
                        calendar.is_open(cutoff - chrono::TimeDelta::seconds(1)),
                        "{date}"
                    );
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle.
                    assert_eq!(
                        calendar.session_bounds(ct_on(date, (9, 0, 0))),
                        Some((ct_on(date, (8, 30, 0)), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(ct_on(date, (9, 0, 0)), CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - chrono::TimeDelta::seconds(1)),
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
        (five_past_noon, quarter_past_noon, closures, unsourced),
        (3, 1, 26, 3),
        "the era's shape"
    );
}

/// A closure deletes the whole civil day, and the next session the family
/// offers is the next business day's ordinary 08:30 CT open — named here so a
/// shifted reopen fails.
#[test]
fn era_2022_2024_closures_remove_the_whole_civil_day() {
    let calendar = calendar();
    let mut closures = 0_usize;
    for (date, kind, _) in ERA_ROWS {
        if *kind != HolidayKind::Closed {
            continue;
        }
        closures += 1;
        let date = day(*date);
        assert!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            "{date}"
        );
        assert!(
            calendar.is_closed_all_day_on(date, SessionKind::Both),
            "{date}: no wrapping leg survives a closure on this grid"
        );
        // The pre-open, the session and the post-close are all gone, and the
        // previous civil day carries no leg of this trade date.
        for hour in [8, 9, 13, 15] {
            assert!(!calendar.is_open(ct_on(date, (hour, 0, 0))), "{date}");
        }
        assert!(
            !calendar.is_open(ct_on(day_before(date), (17, 0, 0))),
            "{date}"
        );
        assert_eq!(calendar.trade_date(ct_on(date, (10, 0, 0))), None, "{date}");

        let reopen = era_reopen_after_closure(date);
        assert_eq!(
            calendar.next_session_open_after(ct_on(date, (10, 0, 0))),
            Some(reopen),
            "{date}: the next session is the next business day's 08:30 CT open"
        );
        assert_eq!(
            calendar.trade_date(reopen),
            Some(reopen.date_naive()),
            "{date}"
        );
    }
    assert_eq!(closures, 26, "the era's closures");
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday) {
    let calendar = calendar();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), EvidenceTier::T2, "{date}");
    assert!(
        !calendar.is_closed_trade_date(date, SessionKind::Both),
        "{date} must not be reported closed"
    );
    assert!(
        !calendar.is_closed_all_day_on(date, SessionKind::Both),
        "{date} must not be reported closed all day"
    );

    for probe in [
        ct_on(day_before(date), (18, 0, 0)),
        ct_on(date, (8, 30, 0)),
        ct_on(date, (13, 4, 0)),
        ct_on(date, (15, 0, 0)),
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
    let calendar = calendar();
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date);
        let row = calendar
            .holiday_on(date)
            .unwrap_or_else(|| panic!("{date} ships a row"));
        assert_unsourced_changes_nothing(date, row);
    }
}

/// The new window sits second in the declared coverage, its edges answer, and
/// the unaudited 2013-2021 interval below it stays unaudited.
#[test]
fn era_2022_2024_window_sits_second_and_the_2013_2021_interval_is_unaudited() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_livestock ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
            (day((2022, 1, 1)), day((2024, 12, 31))),
            (day((2025, 1, 1)), day((2027, 12, 31))),
        ]
    );
    assert!(coverage.contains(day((2022, 1, 1))));
    assert!(coverage.contains(day((2024, 12, 31))));
    assert!(!coverage.contains(day((2021, 12, 31))));
    assert_eq!(calendar.holiday_on(day((2022, 1, 1))), None);
    assert_eq!(calendar.holiday_on(day((2024, 12, 31))), None);

    for date in [(2013, 1, 1), (2019, 1, 1), (2020, 12, 25), (2021, 7, 5)] {
        assert!(!coverage.contains(day(date)), "{date:?}");
        assert_eq!(calendar.holiday_on(day(date)), None, "{date:?}");
        assert_eq!(
            calendar.is_open(ct(date, (10, 0, 0))),
            bare().is_open(ct(date, (10, 0, 0))),
            "{date:?}"
        );
    }
    // Christmas 2020 is a real CME closure no wave audited; the ordinary
    // Friday answers, and the detached calendar answers it the same.
    for probe in [
        ct((2020, 12, 25), (10, 0, 0)),
        ct((2020, 12, 24), (10, 0, 0)),
    ] {
        assert!(calendar.is_open(probe), "{probe}");
        assert_eq!(calendar.is_open(probe), bare().is_open(probe), "{probe}");
    }
}

/// The four early closes are this family's own instants on its flat grid:
/// 12:05 CT on the three Thanksgiving Fridays and 12:15 CT on Christmas Eve
/// 2024, each ending a day that opened at 08:30 CT the same morning and never
/// at the ordinary 13:05 CT session close.
#[test]
fn era_2022_2024_early_closes_end_the_flat_day_at_the_printed_instant() {
    let calendar = calendar();

    for date in [(2022, 11, 25), (2023, 11, 24), (2024, 11, 29)] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: ERA_FIVE_PAST_NOON
            }),
            "{date:?}"
        );
        assert!(calendar.is_open(ct(date, (12, 4, 59))), "{date:?}");
        assert!(!calendar.is_open(ct(date, (12, 5, 0))), "{date:?}");
        // The ordinary 13:05 CT close is not reached.
        assert!(!calendar.is_open(ct(date, (13, 5, 0))), "{date:?}");
    }

    assert_eq!(
        calendar.holiday_on(day((2024, 12, 24))).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON
        })
    );
    assert!(calendar.is_open(ct((2024, 12, 24), (12, 14, 59))));
    assert!(!calendar.is_open(ct((2024, 12, 24), (12, 15, 0))));
    assert!(!calendar.is_open(ct((2024, 12, 24), (13, 5, 0))));
}
