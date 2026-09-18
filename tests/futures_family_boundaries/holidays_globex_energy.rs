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

use chrono::{
    DateTime, Datelike as _, Days, Duration, NaiveDate, TimeDelta, TimeZone as _, Utc, Weekday,
};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key,
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

/// Case 1 — a closed day. Christmas 2025 falls on a Thursday, so its trade
/// date is removed together with the Wednesday-evening leg that fed it.
#[test]
fn christmas_2025_removes_the_whole_trade_date() {
    let calendar = calendar();
    let christmas = day((2025, 12, 25));

    assert!(calendar.is_closed_trade_date(christmas, SessionKind::Both));
    assert_eq!(
        calendar
            .holiday_on(christmas)
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );
    for (hour, minute) in [(0_u32, 30_u32), (10, 0), (15, 30)] {
        assert!(
            !calendar.is_open(ct((2025, 12, 25), hour, minute)),
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

    assert!(!calendar.is_open(ct((2025, 12, 24), 17, 30)));
    assert!(!calendar.is_open(ct((2025, 12, 24), 23, 0)));

    let (open, close) = calendar
        .next_session_after(ct((2025, 12, 24), 13, 0))
        .expect("a session opens after the shortened Christmas Eve");
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

    assert!(calendar.is_open(one_second_before(cutoff)));
    assert!(!calendar.is_open(cutoff));

    let (open, close) = calendar
        .session_bounds(ct((2025, 11, 28), 10, 0))
        .expect("the shortened Friday session is the containing session");
    assert_eq!(open, ct((2025, 11, 27), 17, 0));
    assert_eq!(close, cutoff);

    assert_eq!(
        calendar.candle_end(ct((2025, 11, 28), 10, 0), CalendarResolution::Daily),
        Some(cutoff)
    );
    assert!(!calendar.is_open(ct((2025, 11, 28), 15, 0)));
}

/// The Thanksgiving pre-open shape CME publishes instead of a close: matching
/// stops at 13:30 CT and the ordinary 17:00 CT open still starts the next
/// trade date, so one civil day carries the end of one trading day and the
/// start of the next.
#[test]
fn the_2025_thanksgiving_early_close_stops_matching_at_13_30_and_reopens_at_17_00() {
    let calendar = calendar();
    let cutoff = ct((2025, 11, 27), 13, 30);

    assert!(calendar.is_open(one_second_before(cutoff)));
    assert!(!calendar.is_open(cutoff));
    assert!(!calendar.is_open(ct((2025, 11, 27), 16, 59)));
    assert!(calendar.is_open(ct((2025, 11, 27), 17, 0)));
}

/// The Friday holidays of 2026 and 2027, where CME prints the early close but
/// dates it to the following Monday. The crate keys the session to the
/// venue-local date of its own final close, so the row stays on the Friday and
/// the Thursday-evening leg it clips is kept rather than deleted.
#[test]
fn juneteenth_2026_clips_the_friday_and_keeps_the_thursday_evening_leg() {
    let calendar = calendar();
    let cutoff = ct((2026, 6, 19), 12, 0);

    assert!(calendar.is_open(ct((2026, 6, 18), 17, 30)));
    assert!(calendar.is_open(one_second_before(cutoff)));
    assert!(!calendar.is_open(cutoff));
    assert_eq!(
        calendar.session_bounds(ct((2026, 6, 19), 9, 0)),
        Some((ct((2026, 6, 18), 17, 0), cutoff))
    );
    assert_eq!(
        calendar.trade_date(ct((2026, 6, 19), 9, 0)),
        Some(day((2026, 6, 19)))
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
            // and is not a late open. The fence this test draws is unchanged.
            assert!(
                matches!(
                    holiday.kind(),
                    HolidayKind::Closed | HolidayKind::EarlyClose { .. } | HolidayKind::Unsourced
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
#[test]
fn trade_dates_follow_the_rows_rather_than_the_civil_day() {
    let calendar = calendar();

    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), 12, 0)),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 27), 18, 0)),
        Some(day((2025, 11, 28)))
    );
    assert_eq!(
        calendar.trade_date(ct((2025, 12, 25), 18, 0)),
        Some(day((2025, 12, 26)))
    );
    assert_eq!(calendar.trade_date(ct((2025, 12, 25), 10, 0)), None);
}

/// Case 7 — both edges of the coverage window. Inside it a date with no row is
/// audited normal; outside it the table has no answer at all and must not
/// silently extend to a holiday it never audited.
#[test]
fn the_coverage_window_bounds_every_answer_the_table_gives() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a holiday table");

    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(coverage.contains(day((2010, 1, 1))));
    assert!(coverage.contains(day((2027, 12, 31))));

    assert_eq!(
        calendar
            .holiday_on(day((2025, 1, 1)))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(calendar.holiday_on(day((2024, 12, 31))).is_none());
    assert!(calendar.holiday_on(day((2028, 1, 1))).is_none());
    // 2024-12-31 became an audited date when the 2022-2024 wave shipped, so
    // its `None` is now "audited normal" rather than "no answer".
    assert!(coverage.contains(day((2024, 12, 31))));

    // Christmas 2020 joined the table when the 2019-2021 wave shipped, so the
    // The probe outside the audited windows is Christmas 2028, which the
    // families' tables end before: it must answer exactly as the normal week
    // does. Christmas 2013 is inside the 2013-2015 wave now and ships a row.
    assert!(coverage.contains(day((2020, 12, 25))));
    assert_eq!(
        calendar.holiday_on(day((2020, 12, 25))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(coverage.contains(day((2013, 12, 25))));
    assert_eq!(
        calendar.holiday_on(day((2013, 12, 25))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(!coverage.contains(day((2028, 12, 25))));
    let probe = ct((2028, 12, 25), 10, 0);
    assert!(calendar.holiday_on(probe.date_naive()).is_none());
    assert_eq!(
        calendar.is_open(probe),
        calendar.without_holidays().is_open(probe)
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
    assert!(detached.is_open(ct((2025, 12, 25), 10, 0)));
    assert!(detached.is_open(ct((2025, 11, 28), 15, 0)));

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
            if calendar.is_open(probe) != detached.is_open(probe) {
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
                calendar.is_open(probe),
                detached.is_open(probe),
                "an audited-normal week must answer identically at {probe}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// The era's early closes clip a trading day that opened 17:00 CT the previous
/// evening: 15:15 CT on the Martin Luther King Jr. Day eve, 12:15 CT on the
/// holiday itself, and 12:45 CT on the day after Thanksgiving.
#[test]
fn era_early_closes_end_the_wrapped_trading_day_at_the_stated_instant() {
    let calendar = calendar();

    for (date, previous_day, close_ssm) in [
        ((2010, 1, 15), (2010, 1, 14), 15 * 3_600 + 15 * 60),
        ((2010, 1, 18), (2010, 1, 17), 12 * 3_600 + 15 * 60),
        ((2010, 11, 26), (2010, 11, 25), 12 * 3_600 + 45 * 60),
    ] {
        let row = calendar
            .holiday_on(day(date))
            .unwrap_or_else(|| panic!("{date:?} ships a row"));
        assert_eq!(row.kind(), HolidayKind::EarlyClose { close_ssm });

        // The evening leg that feeds this trade date is clipped, not deleted.
        assert!(calendar.is_open(ct(previous_day, 17, 0)), "{date:?}");
        assert!(
            calendar.is_open(one_second_before(ct(
                date,
                close_ssm / 3_600,
                (close_ssm % 3_600) / 60
            ))),
            "{date:?}: {close_ssm} is too early"
        );
        assert!(
            !calendar.is_open(ct(date, close_ssm / 3_600, (close_ssm % 3_600) / 60)),
            "{date:?}: {close_ssm} is not end-exclusive"
        );
        assert_eq!(
            calendar.session_bounds(ct(date, 9, 0)),
            Some((
                ct(previous_day, 17, 0),
                ct(date, close_ssm / 3_600, (close_ssm % 3_600) / 60)
            )),
            "{date:?}"
        );
    }

    // The Monday holiday's same-evening 17:00 CT leg still opens the next
    // trade date, untouched.
    assert_eq!(
        calendar.trade_date(ct((2010, 1, 18), 18, 0)),
        Some(day((2010, 1, 19)))
    );
    // After the Thanksgiving Friday cut, the next open is the Sunday leg.
    assert_eq!(
        calendar.next_session_open_after(ct((2010, 11, 26), 13, 0)),
        Some(ct((2010, 11, 28), 17, 0))
    );
}

/// No 2010-2012 row moves a first open: the block is 30 early closes and
/// nothing else, so every post-holiday reopen is the family's ordinary 17:00
/// CT leg.
///
/// The absence is the assertion. A late open is the one kind whose branch
/// choice is data-dependent — a statement at or after 17:00 CT resolves on the
/// preceding local date — so its absence in this block is fenced by walking
/// every date of it, not assumed.
#[test]
fn era_ships_no_late_open_and_reopens_at_the_normal_1700_ct() {
    let calendar = calendar();
    let (mut early_closes, mut closures, mut date) = (0_usize, 0_usize, day((2010, 1, 1)));
    let last = day((2012, 12, 31));

    while date <= last {
        if let Some(row) = calendar.holiday_on(date) {
            match row.kind() {
                HolidayKind::EarlyClose { .. } => early_closes += 1,
                HolidayKind::Closed => closures += 1,
                other => panic!("{date} ships an unexpected holiday kind: {other:?}"),
            }
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the era stays inside the representable calendar");
    }
    assert_eq!(early_closes, 30, "early closes, 2010-2012");
    assert_eq!(closures, 8, "full closures, 2010-2012");

    // New Year's Eve 2010 is one of the 15:15 CT cuts, and the next open after
    // it is the ordinary Sunday 17:00 CT leg, not a moved one.
    assert_eq!(
        calendar.next_session_open_after(ct((2010, 12, 31), 16, 0)),
        Some(ct((2011, 1, 2), 17, 0))
    );
}

/// A date inside the widened window with no row is audited normal: the crate
/// serves the profile's own 2010 week, and the detached calendar agrees, so
/// the answer comes from the row set and not from a profile change.
#[test]
fn era_dates_without_rows_are_audited_normal() {
    let calendar = calendar();
    let detached = calendar.without_holidays();

    for (date, previous_day, time) in [
        ((2010, 6, 15), (2010, 6, 14), (10, 0)),
        ((2011, 3, 9), (2011, 3, 8), (9, 0)),
        ((2012, 10, 10), (2012, 10, 9), (14, 0)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date)),
            None,
            "{date:?} must ship no row"
        );
        for probe in [(previous_day, 17_u32), (date, time.0)] {
            assert!(
                calendar.is_open(ct(probe.0, probe.1, 0)),
                "{:?} {:02}:00 CT is an ordinary trading instant",
                probe.0,
                probe.1
            );
            assert_eq!(
                calendar.is_open(ct(probe.0, probe.1, 0)),
                detached.is_open(ct(probe.0, probe.1, 0)),
                "the row set must not change {:?}",
                probe.0
            );
        }
    }
}

/// The widened window's edges answer as the module declares: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// not applied and the detached calendar agreeing.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = calendar();
    let detached = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a table");

    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(!coverage.contains(day((2009, 12, 25))));

    assert_eq!(calendar.holiday_on(day((2009, 12, 25))), None);
    assert!(calendar.is_open(ct((2009, 12, 25), 10, 0)));
    assert_eq!(
        calendar.is_open(ct((2009, 12, 25), 10, 0)),
        detached.is_open(ct((2009, 12, 25), 10, 0))
    );
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// The era's early closes clip the wrapped trading day at the printed instant,
/// including the three year-end half-days that close at 12:45 CT rather than
/// the financial families' 12:15.
#[test]
fn wave2_early_closes_end_the_wrapped_trading_day_at_the_printed_instant() {
    let calendar = calendar();

    for (date, previous_day, close_ssm) in [
        ((2016, 1, 18), (2016, 1, 17), 12 * 3_600),
        ((2017, 7, 4), (2017, 7, 3), 12 * 3_600),
        ((2016, 11, 25), (2016, 11, 24), 12 * 3_600 + 45 * 60),
        ((2018, 12, 24), (2018, 12, 23), 12 * 3_600 + 45 * 60),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::EarlyClose { close_ssm }),
            "{date:?}"
        );
        let cutoff = ct(date, close_ssm / 3_600, (close_ssm % 3_600) / 60);
        assert!(calendar.is_open(ct(previous_day, 17, 0)), "{date:?}");
        assert!(calendar.is_open(cutoff - TimeDelta::seconds(1)), "{date:?}");
        assert!(!calendar.is_open(cutoff), "{date:?}: end-exclusive");
        assert_eq!(
            calendar.candle_end(ct(date, 9, 0), CalendarResolution::Daily),
            Some(cutoff),
            "{date:?}"
        );
    }

    // 2017-07-03 and 2018-07-03 print the family's ordinary 16:00 CT close.
    for date in [(2017, 7, 3), (2018, 7, 3)] {
        assert_eq!(calendar.holiday_on(day(date)), None, "{date:?}");
        assert!(calendar.is_open(ct(date, 15, 30)), "{date:?}");
    }
}

/// A closure removes its trade date and the prior-evening leg; the era ships
/// no late open and no `Unsourced` row.
#[test]
fn wave2_closures_remove_the_trade_date_and_ship_no_late_open() {
    let calendar = calendar();

    assert_eq!(
        calendar.holiday_on(day((2016, 3, 25))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(!calendar.is_open(ct((2016, 3, 24), 18, 0)));
    assert!(!calendar.is_open(ct((2016, 3, 25), 10, 0)));
    assert_eq!(
        calendar.next_session_open_after(ct((2016, 3, 25), 10, 0)),
        Some(ct((2016, 3, 27), 17, 0))
    );

    let mut date = day((2016, 1, 1));
    let mut closures = 0_usize;
    let mut early_closes = 0_usize;
    while date <= day((2018, 12, 31)) {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::Closed) => closures += 1,
            Some(HolidayKind::EarlyClose { .. }) => early_closes += 1,
            None => {}
            Some(other) => panic!("{date}: the era ships no {other:?}"),
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(closures, 9, "2016-2018 closures");
    assert_eq!(early_closes, 22, "2016-2018 early closes");
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 13:30 CT, the Monday and Thursday holiday close this era prints.
const ERA_THIRTEEN_THIRTY: u32 = 13 * 3_600 + 30 * 60;
/// 12:45 CT, the Thanksgiving Friday and Christmas Eve close.
const ERA_TWELVE_FORTY_FIVE: u32 = 12 * 3_600 + 45 * 60;
/// 13:45 CT, the day after Thanksgiving 2024 as the operator's own service
/// prints it.
const ERA_THIRTEEN_FORTY_FIVE: u32 = 13 * 3_600 + 45 * 60;

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
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 2, 21),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 5, 30),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 6, 20),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 9, 5),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2022, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
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
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 6, 19),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 9, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 23),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2023, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TWELVE_FORTY_FIVE,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2024, 1, 15),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 2, 19),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T2,
    ),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 6, 19),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_THIRTY,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: ERA_THIRTEEN_FORTY_FIVE,
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
fn ct_on(date: NaiveDate, hour: u32, minute: u32) -> DateTime<Utc> {
    ct((date.year(), date.month(), date.day()), hour, minute)
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
    ct_on(reopen, 17, 0)
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = calendar();
    let mut index = 0_usize;
    let (mut thirteen_thirty, mut twelve_forty_five, mut thirteen_forty_five) = (0_usize, 0, 0);
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
                        ERA_THIRTEEN_THIRTY => thirteen_thirty += 1,
                        ERA_TWELVE_FORTY_FIVE => twelve_forty_five += 1,
                        ERA_THIRTEEN_FORTY_FIVE => thirteen_forty_five += 1,
                        other => panic!("{date}: the era ships no {other} CT close"),
                    }
                    let cutoff = ct_on(date, close_ssm / 3_600, (close_ssm % 3_600) / 60);
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(calendar.is_open(ct_on(day_before(date), 17, 0)), "{date}");
                    assert!(calendar.is_open(ct_on(day_before(date), 19, 30)), "{date}");
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), 18, 0)),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(calendar.is_open(cutoff - TimeDelta::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle.
                    assert_eq!(
                        calendar.session_bounds(ct_on(date, 9, 0)),
                        Some((ct_on(day_before(date), 17, 0), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(ct_on(date, 9, 0), CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - TimeDelta::seconds(1)),
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
        (
            thirteen_thirty,
            twelve_forty_five,
            thirteen_forty_five,
            closures,
            unsourced
        ),
        (19, 3, 1, 7, 3),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
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
        // The evening leg that would have carried this trade date is gone.
        assert!(!calendar.is_open(ct_on(day_before(date), 17, 0)), "{date}");
        assert!(!calendar.is_open(ct_on(day_before(date), 19, 30)), "{date}");
        // And so is the trade date's own civil day.
        assert!(!calendar.is_open(ct_on(date, 9, 0)), "{date}");
        assert!(!calendar.is_open(ct_on(date, 15, 59)), "{date}");
        assert_eq!(calendar.trade_date(ct_on(date, 10, 0)), None, "{date}");

        let reopen = era_reopen_after_closure(date);
        assert_eq!(
            calendar.next_session_open_after(ct_on(date, 10, 0)),
            Some(reopen),
            "{date}: the next session is the ordinary evening open"
        );
        if reopen == ct_on(date, 17, 0) {
            assert_eq!(calendar.trade_date(reopen), Some(day_after(date)), "{date}");
        }
    }
    assert_eq!(closures, 7, "the era's closures");
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = calendar();
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
        ct_on(day_before(date), 18, 0),
        ct_on(date, 9, 0),
        ct_on(date, 15, 59),
        ct_on(date, 18, 0),
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
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
            (day((2013, 1, 1)), day((2015, 12, 31))),
            (day((2016, 1, 1)), day((2018, 12, 31))),
            (day((2019, 1, 1)), day((2021, 12, 31))),
            (day((2022, 1, 1)), day((2024, 12, 31))),
            (day((2025, 1, 1)), day((2027, 12, 31))),
        ]
    );
    assert!(coverage.contains(day((2022, 1, 1))));
    assert!(coverage.contains(day((2024, 12, 31))));
    assert!(coverage.contains(day((2021, 12, 31))));
    assert_eq!(calendar.holiday_on(day((2022, 1, 1))), None);
    assert_eq!(calendar.holiday_on(day((2024, 12, 31))), None);

    // The 2013-2015 wave shipped after this test was written: 2015-12-24 is
    // its 12:45 CT early close and 2015-12-25 its Christmas closure.
    assert_eq!(
        calendar.holiday_on(day((2015, 12, 24))).map(Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60
        })
    );
    assert_eq!(
        calendar.holiday_on(day((2015, 12, 25))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(!calendar.is_open(ct((2015, 12, 24), 13, 0)));
    assert!(!calendar.is_open(ct((2015, 12, 25), 10, 0)));
}

/// The era's closes are this venue's own — 13:30 CT on the Monday and Thursday
/// holidays, 12:45 CT on the Thanksgiving Fridays and Christmas Eve, 13:45 CT
/// once — and never the financial families' 12:00 or 12:15 CT. Asserted against
/// the equity index on the same dates, so a copied instant fails here.
#[test]
fn era_2022_2024_close_instants_are_the_energy_venues_not_the_financial_ones() {
    let energy = calendar();
    let equity = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    // 2022-01-17: the financial families stop at 12:00 CT; energy trades on.
    assert!(!equity.is_open(ct((2022, 1, 17), 12, 0)));
    assert!(energy.is_open(ct((2022, 1, 17), 12, 0)));
    assert!(energy.is_open(ct((2022, 1, 17), 13, 29)));
    assert!(!energy.is_open(ct((2022, 1, 17), 13, 30)));

    // 2022-11-25: the equity index stops at 12:15 CT, energy at 12:45 CT.
    assert!(!equity.is_open(ct((2022, 11, 25), 12, 15)));
    assert!(energy.is_open(ct((2022, 11, 25), 12, 15)));
    assert!(energy.is_open(ct((2022, 11, 25), 12, 44)));
    assert!(!energy.is_open(ct((2022, 11, 25), 12, 45)));

    // 2024-11-29: the one 13:45 CT close, one hour after 2024-12-24's.
    assert!(energy.is_open(ct((2024, 11, 29), 13, 44)));
    assert!(!energy.is_open(ct((2024, 11, 29), 13, 45)));
    assert!(energy.is_open(ct((2024, 12, 24), 12, 44)));
    assert!(!energy.is_open(ct((2024, 12, 24), 12, 45)));
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// 12:00 CT, the Monday and Thursday holiday close this era prints.
const ERA_NOON: u32 = 12 * 3_600;

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. An early
/// close must be open one second before its printed instant and closed at it;
/// a closure must take the trade date and the evening leg that opened it.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = calendar();
    let (mut noons, mut twelve_forty_fives) = (0_usize, 0);
    let (mut closures, mut unsourced) = (0_usize, 0);
    let mut rows = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
        if let Some(row) = calendar.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::EarlyClose { close_ssm } => {
                    match close_ssm {
                        ERA_NOON => noons += 1,
                        ERA_TWELVE_FORTY_FIVE => twelve_forty_fives += 1,
                        other => {
                            panic!("{date}: this family ships no {other}-second CT close here")
                        }
                    }
                    let cutoff = ct_on(date, close_ssm / 3_600, (close_ssm % 3_600) / 60);
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(calendar.is_open(ct_on(day_before(date), 17, 0)), "{date}");
                    assert!(calendar.is_open(ct_on(day_before(date), 19, 30)), "{date}");
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), 18, 0)),
                        Some(date),
                        "{date}"
                    );
                    // One second before the close is open; at it, closed.
                    assert!(calendar.is_open(cutoff - TimeDelta::seconds(1)), "{date}");
                    assert!(!calendar.is_open(cutoff), "{date}: end-exclusive");
                    // The trading day's bounds end at the printed instant, and
                    // so does the daily candle. The probe sits just inside the
                    // session, so an earlier-than-expected close cannot make
                    // the query answer `None` instead.
                    let inside = cutoff - TimeDelta::minutes(1);
                    assert_eq!(
                        calendar.session_bounds(inside),
                        Some((ct_on(day_before(date), 17, 0), cutoff)),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.candle_end(inside, CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                    assert_eq!(
                        calendar.trade_date(cutoff - TimeDelta::seconds(1)),
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
                    assert!(!calendar.is_open(ct_on(day_before(date), 17, 0)), "{date}");
                    assert!(!calendar.is_open(ct_on(day_before(date), 19, 30)), "{date}");
                    assert!(!calendar.is_open(ct_on(date, 9, 0)), "{date}");
                    assert!(!calendar.is_open(ct_on(date, 15, 59)), "{date}");
                    assert_eq!(calendar.trade_date(ct_on(date, 10, 0)), None, "{date}");
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
        (noons, twelve_forty_fives, closures, unsourced),
        (18, 5, 9, 3),
        "the era's shape"
    );
}

/// Every `Unsourced` row the era ships changes no answer: the row states that
/// the date was audited, makes no scheduling claim, and clips nothing.
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = calendar();
    let mut checked = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
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
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a table");
    let era = (day((2019, 1, 1)), day((2021, 12, 31)));

    assert!(
        coverage.windows().contains(&era),
        "the 2019-2021 window is declared as a window of its own"
    );
    let inside = |date: NaiveDate| era.0 <= date && date <= era.1;
    assert!(inside(day((2019, 1, 1))));
    assert!(inside(day((2021, 12, 31))));
    assert!(
        !inside(day((2018, 12, 31))),
        "2018-12-31 belongs to the 2016-2018 wave, not to this era"
    );
    assert!(
        !inside(day((2022, 1, 1))),
        "2022-01-01 belongs to the 2022-2024 wave, not to this era"
    );

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is an ordinary Friday this table audited.
    assert_eq!(
        calendar.holiday_on(day((2019, 1, 1))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(calendar.holiday_on(day((2021, 12, 31))), None);
    assert!(calendar.is_open(ct((2021, 12, 31), 9, 0)));
    // The neighbouring dates, which other waves audit, carry no row here.
    assert_eq!(calendar.holiday_on(day((2018, 12, 31))), None);
    assert_eq!(calendar.holiday_on(day((2022, 1, 1))), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = calendar();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_energy ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
            (day((2013, 1, 1)), day((2015, 12, 31))),
            (day((2016, 1, 1)), day((2018, 12, 31))),
            (day((2019, 1, 1)), day((2021, 12, 31))),
            (day((2022, 1, 1)), day((2024, 12, 31))),
            (day((2025, 1, 1)), day((2027, 12, 31))),
        ]
    );

    // Every shipped row of the era is inside the era's own window: the walk
    // reads the module, and the declared window is what must contain it.
    let mut rows = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
        if calendar.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 35, "the era's rows");
    for probe in [day((2018, 12, 31)), day((2022, 1, 1))] {
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
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2019, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60,
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
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2020, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60,
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
    ((2021, 4, 2), HolidayKind::Closed, EvidenceTier::T1),
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
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2021, 12, 24), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2019_2021_rows_are_the_audited_date_kind_and_tier_set() {
    let calendar = calendar();
    let mut index = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
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
#[test]
fn era_2013_2015_sweeps_every_shipped_row_kind_and_instant() {
    let venue = calendar();
    let (mut closed, mut early, mut late, mut both) = (0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = day((2013, 1, 1));
    while date <= day((2015, 12, 31)) {
        if let Some(row) = venue.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::Closed => {
                    closed += 1;
                    assert!(
                        venue.is_closed_trade_date(date, SessionKind::Both),
                        "{date}"
                    );
                    assert!(
                        !venue.is_open(ct_on(day_before(date), 17, 0)),
                        "{date}: the eve leg is gone"
                    );
                }
                HolidayKind::EarlyClose { close_ssm } => {
                    early += 1;
                    let (h, m, _s) = (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let cutoff = ct_on(date, h, m);
                    assert!(venue.is_open(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!venue.is_open(cutoff), "{date}: end-exclusive");
                    assert_eq!(
                        venue.trade_date(cutoff - Duration::seconds(1)),
                        Some(date),
                        "{date}"
                    );
                    assert_eq!(
                        venue.candle_end(cutoff - Duration::minutes(1), CalendarResolution::Daily),
                        Some(cutoff),
                        "{date}"
                    );
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late += 1;
                    let (h, m, _s) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let open = ct_on(date, h, m);
                    assert!(!venue.is_open(open - Duration::seconds(1)), "{date}");
                    assert!(
                        venue.is_open(open),
                        "{date}: matching starts at the printed instant"
                    );
                    assert_eq!(
                        venue.trade_date(open + Duration::hours(1)),
                        Some(date),
                        "{date}: keyed to its own trade date"
                    );
                }
                HolidayKind::LateOpenAndEarlyClose {
                    open_ssm,
                    close_ssm,
                } => {
                    both += 1;
                    let (oh, om, _os) = (open_ssm / 3_600, (open_ssm % 3_600) / 60, open_ssm % 60);
                    let (ch, cm, _cs) =
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60);
                    let open = ct_on(date, oh, om);
                    let cutoff = ct_on(date, ch, cm);
                    assert!(!venue.is_open(open - Duration::seconds(1)), "{date}");
                    assert!(venue.is_open(open), "{date}");
                    assert!(venue.is_open(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!venue.is_open(cutoff), "{date}: end-exclusive");
                }
                other => panic!("{date}: this era ships no {other:?}"),
            }
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        (closed, early, late, both),
        (9, 24, 0, 0),
        "the era's shape"
    );
}

/// The era is its own declared window: 2013-01-01 is inside it and 2012-12-31
/// and 2016-01-01 belong to the waves either side and lie outside it.
#[test]
fn era_2013_2015_window_edges_answer_as_the_module_declares() {
    let venue = calendar();
    let coverage = venue
        .holiday_coverage()
        .expect("globex_energy ships a table");
    let era = (day((2013, 1, 1)), day((2015, 12, 31)));

    assert!(
        coverage.windows().contains(&era),
        "the 2013-2015 window is declared as a window of its own"
    );
    assert!(coverage.contains(era.0));
    assert!(coverage.contains(era.1));

    // The era's own edges answer for themselves: its first day is the shipped
    // New Year closure, and its last is the last date the window declares.
    assert_eq!(
        venue.holiday_on(day((2013, 1, 1))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(
        coverage.contains(day((2015, 12, 31))),
        "the era's last day is inside the declared window"
    );
    // The neighbouring dates are their own eras' business: 2012-12-31 is
    // audited normal by the wave below, 2016-01-01 opens the next declared
    // window, and neither is this era.
    assert_eq!(venue.holiday_on(day((2012, 12, 31))), None);
    assert!(
        coverage.contains(day((2016, 1, 1))),
        "2016-01-01 is the next declared window's first day"
    );
    // A date below the January-2010 floor is outside every window, so this
    // table has no answer for it at all.
    assert!(!coverage.contains(day((2009, 12, 31))));
    assert_eq!(venue.holiday_on(day((2009, 12, 31))), None);
}

/// The family's coverage names its windows in order, and the 2013-2015 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2013_2015_window_is_declared_in_order_and_bounds_every_row() {
    let venue = calendar();
    let coverage = venue
        .holiday_coverage()
        .expect("globex_energy ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
            (day((2013, 1, 1)), day((2015, 12, 31))),
            (day((2016, 1, 1)), day((2018, 12, 31))),
            (day((2019, 1, 1)), day((2021, 12, 31))),
            (day((2022, 1, 1)), day((2024, 12, 31))),
            (day((2025, 1, 1)), day((2027, 12, 31))),
        ]
    );

    let mut rows = 0_usize;
    let mut date = day((2013, 1, 1));
    while date <= day((2015, 12, 31)) {
        if venue.holiday_on(date).is_some() {
            assert!(coverage.contains(date), "{date} ships outside its window");
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 33, "the era's rows");
    assert_eq!(venue.holiday_on(day((2012, 12, 31))), None);
    assert!(!coverage.contains(day((2028, 1, 1))));
    assert_eq!(venue.holiday_on(day((2028, 1, 1))), None);
}

/// The 2013-2015 rows as the block records them: date, kind and tier in order,
/// handwritten here rather than read back from the module. The era-wide sweep
/// counts kinds and instants, which a row moved to another audited date with
/// the same kind and instant would leave unchanged; this pins the date set
/// itself, in the order `holiday_on` must answer it.
const ERA_2013_2015_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2013, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 1, 21),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 2, 18),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2013, 3, 29), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2013, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2014, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 1, 20),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 2, 17),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 4, 18), HolidayKind::Closed, EvidenceTier::T1),
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
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2014, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2015, 1, 19),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600,
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
    ((2015, 4, 3), HolidayKind::Closed, EvidenceTier::T1),
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
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 45 * 60,
        },
        EvidenceTier::T1,
    ),
    ((2015, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era's audited date, kind and tier set, in order.
#[test]
fn era_2013_2015_rows_are_the_audited_date_kind_and_tier_set() {
    let venue = calendar();
    let mut index = 0_usize;
    let mut date = day((2013, 1, 1));
    while date <= day((2015, 12, 31)) {
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
