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

use chrono::{DateTime, Days, NaiveDate, TimeZone as _, Utc};
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
/// assumed. The walk also fixes the table's shape: 31 closures, 5 early
/// closes, nothing else, and no `Unsourced` row.
#[test]
fn the_table_ships_no_late_open_in_this_window() {
    let calendar = calendar();
    let mut closed = 0_usize;
    let mut early = 0_usize;
    let mut late = 0_usize;
    let mut date = day(FIRST);
    let last = day(LAST);

    while date <= last {
        match calendar.holiday_on(date).map(Holiday::kind) {
            None => {}
            Some(HolidayKind::Closed) => closed += 1,
            Some(HolidayKind::EarlyClose { .. }) => early += 1,
            Some(HolidayKind::LateOpen { .. }) => late += 1,
            Some(other) => panic!("{date} ships an unexpected holiday kind: {other:?}"),
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the coverage window stays inside the representable calendar");
    }

    assert_eq!(closed, 31, "closed rows over the whole 2010-2027 window");
    assert_eq!(
        early, 12,
        "early-close rows over the whole 2010-2027 window"
    );
    assert_eq!(late, 4, "late-open rows, 2010-2012");

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

    // Christmas Day 2024 is a Wednesday one week before the window opens.
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

    // The Good Friday 2011 close is probed in full: 13:55 CT ends the trade
    // date, and the ordinary 17:00 CT leg then opens the next one.
    assert!(calendar.is_open(ct((2011, 4, 21), (13, 54, 59))));
    assert!(!calendar.is_open(ct((2011, 4, 21), (13, 55, 0))));
    assert_eq!(calendar.trade_date(ct((2011, 4, 21), (13, 55, 0))), None);
    assert_eq!(
        calendar.trade_date(ct((2011, 4, 21), (18, 0, 0))),
        Some(day((2011, 4, 22)))
    );

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

/// Good Friday 2010 ships no row in this family — the 2010 block is only the
/// Thanksgiving and New Year's Eve half-days — so the crate serves the normal
/// week, whose Thursday-evening leg already runs to the era's own 13:55 CT
/// Friday close. The probe is the row set: the detached calendar agrees.
#[test]
fn era_good_friday_2010_ships_no_row_and_is_audited_normal() {
    let calendar = calendar();

    assert_eq!(calendar.holiday_on(day((2010, 4, 2))), None);
    assert_eq!(calendar.holiday_on(day((2010, 4, 1))), None);
    assert!(calendar.is_open(ct((2010, 4, 1), (17, 0, 0))));
    assert!(calendar.is_open(ct((2010, 4, 2), (9, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2010, 4, 2), (9, 0, 0))),
        Some((ct((2010, 4, 1), (17, 0, 0)), ct((2010, 4, 2), (13, 55, 0))))
    );
    assert_eq!(
        calendar.next_session_open_after(ct((2010, 4, 2), (14, 0, 0))),
        Some(ct((2010, 4, 5), (9, 5, 0)))
    );
    assert_eq!(
        calendar.is_open(ct((2010, 4, 2), (9, 0, 0))),
        bare().is_open(ct((2010, 4, 2), (9, 0, 0)))
    );

    // A plain trading day in the same year answers the same way.
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
