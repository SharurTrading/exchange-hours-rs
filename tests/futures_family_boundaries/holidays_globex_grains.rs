// SPDX-License-Identifier: MIT-0

//! `globex_grains` built-in holiday rows, 2025-2027 (LAW-HOLIDAY-SCOPE).
//!
//! Every probe is stated in America/Chicago wall clock and converted, so a DST
//! slip in either direction fails rather than passing on a coincidence. The
//! family's sourced grid wraps — an evening leg at 19:00 CT running to 07:45 CT
//! the next morning, then an 08:30-13:20 CT day session whose close is the
//! trading day's final close — so each case below is chosen to exercise the
//! clip on the civil day the operator states it, not on the day the session
//! opened.
//!
//! The dates are CME's own: Christmas 2025 (Thursday closure, 12:05 CT
//! Christmas Eve, no Friday evening leg), the day after Thanksgiving 2025
//! (08:30-12:05 CT and nothing else) and Martin Luther King Jr. Day 2025 (a
//! Monday whose trade date does not exist, while the Monday 19:00 CT open that
//! begins Tuesday's trade date runs normally).

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key, hours_for_market_hours_key,
};

const ZC: MarketHoursKey = MarketHoursKey::GlobexGrains;

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

fn open_at(instant: DateTime<Utc>) -> bool {
    calendar_for_market_hours_key(ZC).is_open(instant)
}

/// Case 1 — a closed trade date removes its whole trading day, including the
/// leg that opened the previous evening, and leaves the evening leg that begins
/// the *next* trade date alone.
#[test]
fn christmas_2025_closes_its_trade_date_and_the_prior_evening_leg() {
    let calendar = calendar_for_market_hours_key(ZC);
    let closed = day((2025, 12, 25));

    assert_eq!(
        calendar.holiday_on(closed).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );
    assert!(calendar.is_closed_trade_date(closed, SessionKind::Both));

    for probe in [(2, 0, 0), (9, 0, 0), (12, 0, 0), (20, 0, 0)] {
        assert!(
            !open_at(ct((2025, 12, 25), probe)),
            "no session belongs to trade date 2025-12-25 at {probe:?} CT"
        );
    }

    // The Wednesday-evening leg would have carried trade date 2025-12-25.
    assert!(!open_at(ct((2025, 12, 24), (19, 0, 0))));
    assert!(!open_at(ct((2025, 12, 24), (23, 30, 0))));
}

/// Case 1, the converse — the Monday holiday's own trade date is gone, but the
/// Monday 19:00 CT open that begins Tuesday's trade date is untouched.
#[test]
fn mlk_2025_removes_the_trade_date_not_the_evening_leg_that_begins_the_next() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.holiday_on(day((2025, 1, 20))).map(Holiday::kind),
        Some(HolidayKind::Closed),
    );
    // The Sunday-evening leg fed trade date 2025-01-20 and is deleted with it.
    assert!(!open_at(ct((2025, 1, 19), (19, 30, 0))));
    assert!(!open_at(ct((2025, 1, 20), (9, 0, 0))));
    assert!(!open_at(ct((2025, 1, 20), (12, 0, 0))));
    // The Monday-evening leg carries trade date 2025-01-21 and runs normally.
    assert!(open_at(ct((2025, 1, 20), (19, 30, 0))));
    assert!(open_at(ct((2025, 1, 21), (9, 0, 0))));
}

/// Case 2 — the instant before an early close is still open.
#[test]
fn christmas_eve_2025_is_open_one_second_before_the_early_close() {
    assert!(open_at(
        ct((2025, 12, 24), (12, 5, 0)) - Duration::seconds(1)
    ));
    assert!(open_at(ct((2025, 12, 24), (10, 0, 0))));
}

/// Case 3 — the early close is end-exclusive, ends the session and the daily
/// bar, and deletes what the normal week would have run after it.
#[test]
fn christmas_eve_2025_closes_at_1205_central() {
    let calendar = calendar_for_market_hours_key(ZC);
    let cutoff = ct((2025, 12, 24), (12, 5, 0));
    let probe = ct((2025, 12, 24), (10, 0, 0));

    assert!(!open_at(cutoff));
    assert_eq!(
        calendar.session_bounds(probe),
        Some((ct((2025, 12, 24), (8, 30, 0)), cutoff)),
    );
    assert_eq!(
        calendar.candle_end(probe, CalendarResolution::Daily),
        Some(cutoff),
    );
    // The normal week runs the day session to 13:20 CT; the row removes it.
    assert!(!open_at(ct((2025, 12, 24), (13, 0, 0))));
}

/// Case 4 — a late open stated at 08:30 CT, which is earlier than the trading
/// day's normal 19:00 CT first open, so the cutoff lands on the trade date
/// itself and deletes the prior-evening leg rather than the day session.
#[test]
fn boxing_day_2025_opens_late_at_0830_central() {
    let cutoff = ct((2025, 12, 26), (8, 30, 0));

    // Thursday 19:00 CT would normally have opened trade date 2025-12-26.
    assert!(!open_at(ct((2025, 12, 25), (19, 30, 0))));
    assert!(!open_at(ct((2025, 12, 26), (6, 0, 0))));
    assert!(!open_at(cutoff - Duration::seconds(1)));
    assert!(open_at(cutoff));
    assert_eq!(
        calendar_for_market_hours_key(ZC).session_bounds(cutoff),
        Some((cutoff, ct((2025, 12, 26), (13, 20, 0)))),
    );
}

/// Case 4 again, on the row that moves both boundaries: the day after
/// Thanksgiving 2025 is 08:30-12:05 CT and nothing else.
#[test]
fn day_after_thanksgiving_2025_is_a_single_0830_to_1205_block() {
    let calendar = calendar_for_market_hours_key(ZC);
    let open = ct((2025, 11, 28), (8, 30, 0));
    let close = ct((2025, 11, 28), (12, 5, 0));

    assert_eq!(
        calendar.holiday_on(day((2025, 11, 28))).map(Holiday::kind),
        Some(HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 8 * 3_600 + 30 * 60,
            close_ssm: 12 * 3_600 + 5 * 60,
        }),
    );
    assert!(!open_at(ct((2025, 11, 27), (19, 30, 0))));
    assert!(!open_at(open - Duration::seconds(1)));
    assert!(open_at(open));
    assert!(open_at(close - Duration::seconds(1)));
    assert!(!open_at(close));
    assert_eq!(calendar.session_bounds(open), Some((open, close)));
}

/// Case 5 — the removed wrap, read through the query a consumer would use:
/// after Christmas Eve's 12:05 CT close the next grain session is Friday's
/// 08:30 CT day open, because both intervening evening legs are gone.
#[test]
fn next_session_after_christmas_eve_2025_is_the_friday_day_open() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.next_session_open_after(ct((2025, 12, 24), (12, 10, 0))),
        Some(ct((2025, 12, 26), (8, 30, 0))),
    );
    assert_eq!(
        calendar.next_session_after(ct((2025, 12, 24), (12, 10, 0))),
        Some((
            ct((2025, 12, 26), (8, 30, 0)),
            ct((2025, 12, 26), (13, 20, 0))
        )),
    );
}

/// Case 6 — the trade-date consequence. A shortened day keeps its own trade
/// date; the holiday evening's open carries the post-holiday date; and the
/// evening leg that fed a closed date has no trade date at all because it has
/// no session.
#[test]
fn holiday_rows_move_the_trade_date_the_way_cme_prints_it() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.trade_date(ct((2025, 12, 24), (10, 0, 0))),
        Some(day((2025, 12, 24))),
    );
    assert_eq!(
        calendar.trade_date(ct((2025, 11, 28), (10, 0, 0))),
        Some(day((2025, 11, 28))),
    );
    // CME prints "19:00 open (trade date 2025-01-21)" on MLK Monday.
    assert_eq!(
        calendar.trade_date(ct((2025, 1, 20), (19, 30, 0))),
        Some(day((2025, 1, 21))),
    );
    // The Sunday leg that would have fed the closed Monday is gone entirely.
    assert_eq!(calendar.trade_date(ct((2025, 1, 19), (19, 30, 0))), None);
}

/// Case 7 — both edges of the declared coverage window. Inside it a date with
/// no row is audited normal; outside it the table has no answer and the
/// normal week is served unmodified, so a holiday one day below the window and
/// a holiday well above it are both ignored.
#[test]
fn coverage_window_is_declared_and_does_not_extend() {
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a holiday table");

    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(coverage.contains(day((2010, 1, 1))));
    assert!(coverage.contains(day((2027, 12, 31))));
    assert!(!coverage.contains(day((2009, 12, 31))));
    assert!(!coverage.contains(day((2028, 1, 1))));

    assert_eq!(calendar.holiday_on(day((2009, 12, 31))), None);
    assert_eq!(calendar.holiday_on(day((2028, 1, 1))), None);

    // Christmas 2009 is a real CME closure below the window; the table says
    // nothing about it and the normal week answers. Christmas 2009 fell on a
    // Friday, whose 09:30-13:15 CT day session is over by 13:15, so the probe
    // is the Thursday morning session that traded normally into it.
    assert_eq!(calendar.holiday_on(day((2009, 12, 25))), None);
    assert!(open_at(ct((2009, 12, 24), (9, 30, 0))));
    assert!(open_at(ct((2009, 12, 24), (12, 0, 0))));

    // Martin Luther King Jr. Day 2028 is above the window; the table does not
    // extrapolate the pattern it holds for 2025, 2026 and 2027.
    assert_eq!(calendar.holiday_on(day((2028, 1, 17))), None);
    assert!(open_at(ct((2028, 1, 17), (9, 0, 0))));
}

/// Case 7's inner half — a date inside coverage with no row is audited normal.
/// Good Friday eve 2026 is one of the eleven dates on which CME publishes the
/// whole day session and then withholds the evening leg; the leg belongs to the
/// closed Friday's trade date and the `Closed` row alone removes it.
#[test]
fn good_friday_eve_2026_is_audited_normal_and_ships_no_row() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(calendar.holiday_on(day((2026, 4, 2))), None);
    assert!(open_at(ct((2026, 4, 2), (9, 0, 0))));
    assert!(open_at(ct((2026, 4, 2), (13, 19, 0))));
    assert!(!open_at(ct((2026, 4, 2), (13, 20, 0))));
    // The 19:00 CT leg would have carried trade date 2026-04-03.
    assert!(!open_at(ct((2026, 4, 2), (19, 30, 0))));
    assert!(calendar.is_closed_trade_date(day((2026, 4, 3)), SessionKind::Both));
}

/// `without_holidays` restores exactly the pre-table answer, over a dense
/// instant grid across the whole Christmas 2025 week and at each shipped row's
/// own probes. This is the consumer's escape hatch and the benchmark control.
#[test]
fn without_holidays_reproduces_the_normal_week() {
    let detached = calendar_for_market_hours_key(ZC).without_holidays();

    assert_eq!(detached.holiday_coverage(), None);
    assert_eq!(detached.holiday_on(day((2025, 12, 25))), None);
    assert!(detached.is_open(ct((2025, 12, 25), (9, 0, 0))));
    assert!(detached.is_open(ct((2025, 12, 24), (13, 0, 0))));
    assert!(detached.is_open(ct((2025, 12, 25), (19, 30, 0))));
    assert!(detached.is_open(ct((2025, 1, 20), (9, 0, 0))));
    assert!(detached.is_open(ct((2025, 11, 28), (13, 0, 0))));

    let mut probe = ct((2025, 12, 21), (0, 0, 0));
    let end = ct((2025, 12, 29), (0, 0, 0));
    while probe < end {
        assert_eq!(
            detached.is_open(probe),
            hours_for_market_hours_key(ZC, probe).is_open(probe),
            "detached calendar must equal the identity-erased snapshot at {probe}"
        );
        probe += Duration::minutes(5);
    }
}

// ---------------------------------------------------------------------------
// The 2010-2012 rows.
// ---------------------------------------------------------------------------

/// 12:00 CT, the only early final close this family's 2010-2012 block carries.
const ERA_HALF_DAY_CLOSE: u32 = 12 * 3_600;

/// The era's early close clips the day session of a trading day that opened
/// 18:00 CT the previous evening: 2010-11-26 stops at 12:00 CT, before the
/// regular session's own 13:15 CT close, and every other early close in the
/// block states the same instant.
#[test]
fn era_early_closes_end_the_day_session_at_1200_central() {
    let calendar = calendar_for_market_hours_key(ZC);
    let row = calendar
        .holiday_on(day((2010, 11, 26)))
        .expect("2010-11-26 ships a row");
    assert_eq!(
        row.kind(),
        HolidayKind::EarlyClose {
            close_ssm: ERA_HALF_DAY_CLOSE
        }
    );

    // The Thursday-evening leg opened normally and ran to its 07:15 CT pause.
    assert!(open_at(ct((2010, 11, 25), (18, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2010, 11, 25), (18, 0, 0))),
        Some(day((2010, 11, 26)))
    );
    // One second before the close, and at it: closes are end-exclusive.
    assert!(open_at(ct((2010, 11, 26), (11, 59, 59))));
    assert!(!open_at(ct((2010, 11, 26), (12, 0, 0))));
    // The rest of the regular 09:30-13:15 CT session is gone.
    assert!(!open_at(ct((2010, 11, 26), (13, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2010, 11, 26), (11, 0, 0))),
        Some((
            ct((2010, 11, 26), (9, 30, 0)),
            ct((2010, 11, 26), (12, 0, 0))
        ))
    );
    assert_eq!(calendar.trade_date(ct((2010, 11, 26), (12, 0, 0))), None);

    // Every other early close in the block states the same 12:00 CT instant.
    for date in [(2010, 12, 31), (2011, 11, 25), (2012, 7, 3), (2012, 12, 24)] {
        let row = calendar
            .holiday_on(day(date))
            .unwrap_or_else(|| panic!("{date:?} ships a row"));
        assert_eq!(
            row.kind(),
            HolidayKind::EarlyClose {
                close_ssm: ERA_HALF_DAY_CLOSE
            },
            "{date:?}"
        );
        assert!(open_at(ct(date, (11, 59, 59))), "{date:?} closed too early");
        assert!(
            !open_at(ct(date, (12, 0, 0))),
            "{date:?} is not end-exclusive"
        );
    }
}

/// The era's late opens split by where their cutoff lands, exactly as the
/// scalar vocabulary resolves it: an instant earlier than the trading day's
/// normal first open lands on the trade date itself, and one at or after it
/// lands on the preceding local date.
///
/// 2011-12-27 states 09:30 CT, earlier than the era's 18:00 CT evening open,
/// so trade date 2011-12-27 begins on its own civil day.
///
/// 2012-05-28 states 19:00 CT, at or after the 17:00 CT open in force from
/// 2012-05-20, so the cutoff lands on the preceding local date. That is exactly
/// what CME states: the sheet prints `1900 CT (Mon May 28, for trade date
/// Tue May 29)`, and the crate's grid opens the evening leg *for trade date D*
/// at 17:00 CT on `D - 1`, so on 05-28 itself the 19:00 instant withholds the
/// Sunday-evening leg that would have fed 05-28 and the ordinary 17:00 CT leg
/// still opens 05-29. Keying the row to 05-29 instead would put the cutoff at
/// 19:00 on 05-28 — precisely when that evening leg opens — and change no
/// answer at all. The residual, that CME prints 05-29 as the instant's own
/// trade date, is recorded in the family's evidence file.
#[test]
fn era_late_opens_land_where_the_normal_first_open_puts_them() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar
            .holiday_on(day((2011, 12, 27)))
            .expect("2011-12-27 ships a row")
            .kind(),
        HolidayKind::LateOpen {
            open_ssm: 9 * 3_600 + 30 * 60
        }
    );
    assert!(!open_at(ct((2011, 12, 26), (18, 0, 0))));
    assert!(!open_at(ct((2011, 12, 27), (9, 29, 59))));
    assert!(open_at(ct((2011, 12, 27), (9, 30, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2011, 12, 27), (9, 30, 0))),
        Some((
            ct((2011, 12, 27), (9, 30, 0)),
            ct((2011, 12, 27), (13, 15, 0))
        ))
    );

    assert_eq!(
        calendar
            .holiday_on(day((2012, 5, 28)))
            .expect("2012-05-28 ships a row")
            .kind(),
        HolidayKind::LateOpen {
            open_ssm: 19 * 3_600
        }
    );
    assert!(!open_at(ct((2012, 5, 27), (18, 59, 59))));
    assert!(open_at(ct((2012, 5, 27), (19, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2012, 5, 27), (19, 0, 0))),
        Some((ct((2012, 5, 27), (19, 0, 0)), ct((2012, 5, 28), (9, 30, 0))))
    );
    assert_eq!(
        calendar.trade_date(ct((2012, 5, 27), (19, 0, 0))),
        Some(day((2012, 5, 28)))
    );
    // The holiday's own evening leg still opens at the ordinary 17:00 CT.
    assert_eq!(
        calendar.trade_date(ct((2012, 5, 28), (17, 30, 0))),
        Some(day((2012, 5, 29)))
    );
}

/// Thanksgiving Friday 2012 is the era's one late-open-and-early-close row: no
/// prior-evening leg and a single 09:30-12:00 CT block, after which the next
/// session is the ordinary Sunday 17:00 CT open.
#[test]
fn era_late_open_and_early_close_2012_11_23_is_one_0930_to_1200_block() {
    let calendar = calendar_for_market_hours_key(ZC);
    let holiday = calendar
        .holiday_on(day((2012, 11, 23)))
        .expect("2012-11-23 ships a row");
    assert_eq!(
        holiday.kind(),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: 9 * 3_600 + 30 * 60,
            close_ssm: ERA_HALF_DAY_CLOSE,
        }
    );

    // The Wednesday-evening leg that would have carried this trade date did
    // not run; the day begins at 09:30 CT on its own civil date.
    assert!(!open_at(ct((2012, 11, 22), (17, 0, 0))));
    assert!(!open_at(ct((2012, 11, 23), (9, 29, 59))));
    assert!(open_at(ct((2012, 11, 23), (9, 30, 0))));
    assert!(open_at(ct((2012, 11, 23), (11, 59, 59))));
    assert!(!open_at(ct((2012, 11, 23), (12, 0, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2012, 11, 23), (10, 0, 0))),
        Some((
            ct((2012, 11, 23), (9, 30, 0)),
            ct((2012, 11, 23), (12, 0, 0))
        ))
    );
    assert_eq!(
        calendar.next_session_open_after(ct((2012, 11, 23), (12, 10, 0))),
        Some(ct((2012, 11, 25), (17, 0, 0)))
    );
    // Thanksgiving Day itself is a full closure in this family: CME's 2012
    // sheet prints the reopening for Friday 2012-11-23 at 09:30 CT.
    assert_eq!(
        calendar
            .holiday_on(day((2012, 11, 22)))
            .expect("2012-11-22 ships a row")
            .kind(),
        HolidayKind::Closed
    );
    assert!(!open_at(ct((2012, 11, 22), (9, 0, 0))));
}

/// A date inside the widened window with no row is audited normal under both of
/// the era's grids: 2010's 18:00 CT evening open and, from 2012-05-20, the
/// 17:00 CT one. The detached calendar agrees at every probe.
#[test]
fn era_dates_without_rows_are_audited_normal_under_both_grids() {
    let calendar = calendar_for_market_hours_key(ZC);
    let detached = calendar.without_holidays();

    for (date, previous_day, evening_open) in [
        ((2010, 6, 15), (2010, 6, 14), (18, 0)),
        ((2012, 6, 15), (2012, 6, 14), (17, 0)),
    ] {
        assert_eq!(calendar.holiday_on(day(date)), None, "{date:?}");
        assert!(open_at(ct(
            previous_day,
            (evening_open.0, evening_open.1, 0)
        )));
        assert!(open_at(ct(date, (10, 0, 0))));
        assert_eq!(
            calendar.session_bounds(ct(previous_day, (evening_open.0, evening_open.1, 0))),
            detached.session_bounds(ct(previous_day, (evening_open.0, evening_open.1, 0))),
            "{date:?}: the row set must not change the normal week"
        );
    }
}

/// The widened window's edges answer as the module declares: 2010-01-01 to
/// 2027-12-31, with Christmas Day 2009 — a real CME closure one year below it —
/// not applied and the detached calendar agreeing.
#[test]
fn era_window_edges_answer_as_the_module_declares() {
    let calendar = calendar_for_market_hours_key(ZC);
    let detached = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");

    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));
    assert!(!coverage.contains(day((2009, 12, 25))));

    assert_eq!(calendar.holiday_on(day((2009, 12, 25))), None);
    assert!(open_at(ct((2009, 12, 25), (10, 0, 0))));
    assert_eq!(
        open_at(ct((2009, 12, 25), (10, 0, 0))),
        detached.is_open(ct((2009, 12, 25), (10, 0, 0)))
    );
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------

/// 08:30 CT is this era's day-session open, and the three day-after-Thanksgiving
/// rows carry it as a late open beside the 12:05 CT early close: CME withdrew
/// the prior-evening leg and printed the morning session instead.
#[test]
fn wave2_day_after_thanksgiving_opens_late_and_closes_early() {
    let calendar = calendar_for_market_hours_key(ZC);

    for date in [(2016, 11, 25), (2017, 11, 24), (2018, 11, 23)] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: 8 * 3_600 + 30 * 60,
                close_ssm: 12 * 3_600 + 5 * 60,
            }),
            "{date:?}"
        );
        assert!(
            !open_at(ct(date, (8, 29, 0))),
            "{date:?}: the day opens at 08:30"
        );
        assert!(open_at(ct(date, (8, 30, 0))), "{date:?}");
        assert!(
            open_at(ct(date, (12, 4, 0))),
            "{date:?}: 12:05 is end-exclusive"
        );
        assert!(!open_at(ct(date, (12, 5, 0))), "{date:?}");
        // The prior-evening leg that would have fed this trade date is gone:
        // the evening before is the holiday itself.
        let eve = (date.0, date.1, date.2 - 1);
        assert!(
            !open_at(ct(eve, (19, 30, 0))),
            "{date:?}: the holiday evening carries no leg"
        );
        assert!(
            calendar.holiday_on(day(eve)).map(Holiday::kind) == Some(HolidayKind::Closed),
            "{date:?}: the eve must ship the closure"
        );
        // These three dates are Fridays: the grid has no Friday-evening leg,
        // so the next session is the Sunday 19:00 CT open into Monday's trade
        // date, exactly as CME publishes it.
        assert!(
            open_at(ct((date.0, date.1, date.2 + 3), (19, 30, 0))),
            "{date:?}"
        );
        assert_eq!(
            calendar_for_market_hours_key(ZC)
                .trade_date(ct((date.0, date.1, date.2 + 3), (19, 30, 0))),
            Some(day((date.0, date.1, date.2 + 4))),
            "{date:?}"
        );
    }
}

/// The era's plain early closes are 12:05 CT with the ordinary evening leg
/// intact, and 2018-12-26 opens late at 08:30 CT after the Christmas closure.
#[test]
fn wave2_early_closes_and_the_late_open_land_on_the_printed_instants() {
    let calendar = calendar_for_market_hours_key(ZC);

    for (date, eve) in [
        ((2016, 12, 23), (2016, 12, 22)),
        ((2017, 7, 3), (2017, 7, 2)),
        ((2018, 7, 3), (2018, 7, 2)),
        ((2018, 12, 24), (2018, 12, 23)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 12 * 3_600 + 5 * 60
            }),
            "{date:?}"
        );
        assert!(
            open_at(ct(eve, (19, 30, 0))),
            "{date:?}: the eve leg is intact"
        );
        assert!(open_at(ct(date, (12, 4, 0))), "{date:?}");
        assert!(!open_at(ct(date, (12, 5, 0))), "{date:?}");
    }

    assert_eq!(
        calendar.holiday_on(day((2018, 12, 26))).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: 8 * 3_600 + 30 * 60
        })
    );
    assert!(!open_at(ct((2018, 12, 26), (8, 29, 0))));
    assert!(open_at(ct((2018, 12, 26), (8, 30, 0))));
    assert_eq!(
        calendar.trade_date(ct((2018, 12, 26), (9, 0, 0))),
        Some(day((2018, 12, 26)))
    );
}

/// Every early close the era ships states 12:05 CT, and every combined row states
/// 08:30 CT beside it: the instants, not only the kinds, are the fence.
///
/// The venue tables and the family counts both survive a one-minute change to an
/// early close's instant, so this walks the era and pins each one.
#[test]
fn wave2_every_early_close_instant_is_the_printed_one() {
    let calendar = calendar_for_market_hours_key(ZC);
    let (mut plain, mut combined) = (0_usize, 0_usize);
    let mut date = day((2016, 1, 1));
    while date <= day((2018, 12, 31)) {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::EarlyClose { close_ssm }) => {
                assert_eq!(close_ssm, 12 * 3_600 + 5 * 60, "{date}");
                plain += 1;
            }
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm,
                close_ssm,
            }) => {
                assert_eq!(open_ssm, 8 * 3_600 + 30 * 60, "{date}");
                assert_eq!(close_ssm, 12 * 3_600 + 5 * 60, "{date}");
                combined += 1;
            }
            Some(HolidayKind::LateOpen { open_ssm }) => {
                assert_eq!(open_ssm, 8 * 3_600 + 30 * 60, "{date}");
            }
            _ => {}
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the era stays inside the representable calendar");
    }
    assert_eq!((plain, combined), (5, 3), "the era's early closes");
}

/// The era's shape counts, and its closures.
#[test]
fn wave2_ships_twenty_seven_closures_and_no_unsourced_row() {
    let calendar = calendar_for_market_hours_key(ZC);
    let (mut closures, mut early_closes, mut late_opens, mut combined) = (0, 0, 0, 0);
    let mut date = day((2016, 1, 1));
    while date <= day((2018, 12, 31)) {
        match calendar.holiday_on(date).map(Holiday::kind) {
            Some(HolidayKind::Closed) => closures += 1,
            Some(HolidayKind::EarlyClose { .. }) => early_closes += 1,
            Some(HolidayKind::LateOpen { .. }) => late_opens += 1,
            Some(HolidayKind::LateOpenAndEarlyClose { .. }) => combined += 1,
            None => {}
            Some(other) => panic!("{date}: the era ships no {other:?}"),
        }
        date = date
            .checked_add_days(Days::new(1))
            .expect("the era stays inside the representable calendar");
    }
    assert_eq!(
        (closures, early_closes, late_opens, combined),
        (27, 5, 1, 3)
    );

    // A closure removes the day session and the prior-evening leg, and the
    // Monday-evening leg that follows still opens the next trade date.
    assert_eq!(
        calendar.holiday_on(day((2016, 3, 25))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert!(!open_at(ct((2016, 3, 24), (19, 30, 0))));
    assert!(!open_at(ct((2016, 3, 25), (9, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2016, 3, 27), (20, 0, 0))),
        Some(day((2016, 3, 28)))
    );
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 19:00 CT, the ordinary evening leg that opens the next trade date.
const ERA_EVENING_OPEN: u32 = 19 * 3_600;
/// 08:30 CT, the day session's open and the late-open instant.
const ERA_DAY_OPEN: u32 = 8 * 3_600 + 30 * 60;
/// 13:20 CT, the day session's close, which every late open keeps.
const ERA_DAY_CLOSE: u32 = 13 * 3_600 + 20 * 60;
/// 12:05 CT, the era's only early final close.
const ERA_HALF_DAY_FIVE_PAST: u32 = 12 * 3_600 + 5 * 60;

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence: the sweep below walks the whole window
/// and compares against this list row for row, so a dropped, added or moved
/// row fails as loudly as a wrong instant. A sample would let a slipped close
/// move unnoticed on the dates nobody probed, and this family ships more
/// unusual rows than any other — six late opens, three late-open-and-early
/// closes and one early close.
const ERA_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    ((2022, 1, 17), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 2, 21), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 4, 15), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 5, 30), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 6, 20), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 7, 4), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2022, 9, 5), HolidayKind::Closed, EvidenceTier::T1),
    ((2022, 11, 24), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2022, 11, 25),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: ERA_DAY_OPEN,
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
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
    (
        (2023, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2023, 9, 4), HolidayKind::Closed, EvidenceTier::T1),
    ((2023, 11, 23), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2023, 11, 24),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: ERA_DAY_OPEN,
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
        },
        EvidenceTier::T1,
    ),
    ((2023, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2023, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2024, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2024, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T1,
    ),
    ((2024, 1, 15), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 2, 19), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 3, 29), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 5, 27), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 6, 19), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 7, 4), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 7, 5),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T2,
    ),
    ((2024, 9, 2), HolidayKind::Closed, EvidenceTier::T2),
    ((2024, 11, 28), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 11, 29),
        HolidayKind::LateOpenAndEarlyClose {
            open_ssm: ERA_DAY_OPEN,
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
        },
        EvidenceTier::T2,
    ),
    (
        (2024, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_HALF_DAY_FIVE_PAST,
        },
        EvidenceTier::T2,
    ),
    ((2024, 12, 25), HolidayKind::Closed, EvidenceTier::T2),
    (
        (2024, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN,
        },
        EvidenceTier::T2,
    ),
];

/// `ct` for a date the walk computed rather than spelled, so day arithmetic
/// cannot drift out of step with a hand-written tuple.
fn ct_on(date: NaiveDate, time: (u32, u32, u32)) -> DateTime<Utc> {
    ct((date.year(), date.month(), date.day()), time)
}

/// `ct_on` for an instant the module states in venue-local seconds since
/// midnight, so the sweep reads the same constant the row does.
fn ct_at(date: NaiveDate, ssm: u32) -> DateTime<Utc> {
    ct_on(date, (ssm / 3_600, (ssm % 3_600) / 60, ssm % 60))
}

fn day_before(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

fn day_after(date: NaiveDate) -> NaiveDate {
    date.checked_add_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// Whether the trade date after `date` ships a row that moves its own first
/// open, which is exactly the case in which the closure on `date` also deletes
/// that date's 19:00 CT evening leg.
fn era_successor_opens_late(date: NaiveDate) -> bool {
    matches!(
        calendar_for_market_hours_key(ZC)
            .holiday_on(day_after(date))
            .map(Holiday::kind),
        Some(HolidayKind::LateOpen { .. } | HolidayKind::LateOpenAndEarlyClose { .. })
    )
}

/// The ordinary 19:00 CT evening open that follows a closed trade date: this
/// civil date's own leg when the week and the successor's row leave one,
/// otherwise the next session the family offers at all — Sunday's leg after a
/// Friday closure, the successor's 08:30 CT day open where that successor
/// ships a late open.
fn era_reopen_after_closure(date: NaiveDate) -> DateTime<Utc> {
    if era_successor_opens_late(date) {
        return ct_on(day_after(date), (8, 30, 0));
    }
    let reopen = if date.weekday() == Weekday::Fri {
        date.checked_add_days(Days::new(2))
            .expect("the era is far from the representable bound")
    } else {
        date
    };
    ct_at(reopen, ERA_EVENING_OPEN)
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = calendar_for_market_hours_key(ZC);
    let mut index = 0_usize;
    let (mut late_opens, mut combined, mut early_closes) = (0_usize, 0_usize, 0_usize);
    let (mut closures, mut unsourced) = (0_usize, 0_usize);
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
                    early_closes += 1;
                    assert_eq!(close_ssm, ERA_HALF_DAY_FIVE_PAST, "{date}");
                    let cutoff = ct_on(date, (12, 5, 0));
                    // The evening leg that opened this trade date is clipped,
                    // not deleted, and it still carries the trade date.
                    assert!(open_at(ct_at(day_before(date), ERA_EVENING_OPEN)), "{date}");
                    assert_eq!(
                        calendar.trade_date(ct_on(day_before(date), (19, 30, 0))),
                        Some(date),
                        "{date}"
                    );
                    assert!(open_at(cutoff - Duration::seconds(1)), "{date}");
                    assert!(!open_at(cutoff), "{date}: end-exclusive");
                    assert_eq!(
                        calendar.session_bounds(ct_on(date, (9, 0, 0))),
                        Some((ct_on(date, (8, 30, 0)), cutoff)),
                        "{date}"
                    );
                    // The ordinary 13:20 CT day-session close is not reached.
                    assert!(!open_at(ct_at(date, ERA_DAY_CLOSE)), "{date}");
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late_opens += 1;
                    assert_eq!(open_ssm, ERA_DAY_OPEN, "{date}");
                    let open = ct_on(date, (8, 30, 0));
                    // The prior-evening leg is gone, and the day it would have
                    // opened is not: the 08:30 CT day session opens on the
                    // trade date's own civil day and the 13:20 CT close still
                    // ends it.
                    assert!(
                        !open_at(ct_at(day_before(date), ERA_EVENING_OPEN)),
                        "{date}"
                    );
                    assert!(!open_at(ct_on(day_before(date), (19, 30, 0))), "{date}");
                    assert!(
                        !open_at(open - Duration::seconds(1)),
                        "{date}: the day opens at 08:30"
                    );
                    assert!(open_at(open), "{date}");
                    assert_eq!(
                        calendar.session_bounds(open),
                        Some((open, ct_at(date, ERA_DAY_CLOSE))),
                        "{date}"
                    );
                    assert!(
                        open_at(ct_at(date, ERA_DAY_CLOSE) - Duration::seconds(1)),
                        "{date}"
                    );
                    assert!(!open_at(ct_at(date, ERA_DAY_CLOSE)), "{date}");
                    assert_eq!(calendar.trade_date(open), Some(date), "{date}");
                }
                HolidayKind::LateOpenAndEarlyClose {
                    open_ssm,
                    close_ssm,
                } => {
                    combined += 1;
                    assert_eq!(
                        (open_ssm, close_ssm),
                        (ERA_DAY_OPEN, ERA_HALF_DAY_FIVE_PAST),
                        "{date}"
                    );
                    let open = ct_on(date, (8, 30, 0));
                    let close = ct_on(date, (12, 5, 0));
                    assert!(
                        !open_at(ct_at(day_before(date), ERA_EVENING_OPEN)),
                        "{date}"
                    );
                    assert!(!open_at(open - Duration::seconds(1)), "{date}");
                    assert!(open_at(open), "{date}");
                    assert!(open_at(close - Duration::seconds(1)), "{date}");
                    assert!(!open_at(close), "{date}: end-exclusive");
                    assert_eq!(calendar.session_bounds(open), Some((open, close)), "{date}");
                    assert_eq!(calendar.trade_date(open), Some(date), "{date}");
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
        (late_opens, combined, early_closes, closures, unsourced),
        (6, 3, 1, 26, 3),
        "the era's shape"
    );
}

/// The three days after Thanksgiving carry both boundaries — no prior-evening
/// leg and a single 08:30-12:05 CT block — and each follows CME's own
/// Thanksgiving closure.
#[test]
fn era_2022_2024_days_after_thanksgiving_open_late_and_close_early() {
    let calendar = calendar_for_market_hours_key(ZC);

    for (date, thanksgiving) in [
        ((2022, 11, 25), (2022, 11, 24)),
        ((2023, 11, 24), (2023, 11, 23)),
        ((2024, 11, 29), (2024, 11, 28)),
    ] {
        let open = ct(date, (8, 30, 0));
        let close = ct(date, (12, 5, 0));
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: ERA_DAY_OPEN,
                close_ssm: ERA_HALF_DAY_FIVE_PAST,
            }),
            "{date:?}"
        );
        // The Thursday closure the operator pairs with it.
        assert_eq!(
            calendar.holiday_on(day(thanksgiving)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{thanksgiving:?}"
        );
        assert!(!open_at(ct(thanksgiving, (19, 0, 0))), "{date:?}");
        assert!(!open_at(ct(date, (7, 45, 0))), "{date:?}");
        assert!(!open_at(open - Duration::seconds(1)), "{date:?}");
        assert!(open_at(open), "{date:?}");
        assert!(open_at(close - Duration::seconds(1)), "{date:?}");
        assert!(!open_at(close), "{date:?}");
        assert_eq!(
            calendar.session_bounds(open),
            Some((open, close)),
            "{date:?}"
        );
        assert_eq!(
            calendar.trade_date(close - Duration::seconds(1)),
            Some(day(date))
        );
        // The block is followed by the ordinary Sunday 19:00 CT open into
        // Monday's trade date, not by a Friday-evening leg.
        let sunday = day_after(day_after(day(date)));
        assert_eq!(
            calendar.next_session_open_after(close + Duration::minutes(1)),
            Some(ct_on(sunday, (19, 0, 0))),
            "{date:?}"
        );
        assert!(!open_at(ct(date, (19, 0, 0))), "{date:?}");
    }
}

/// The counter-example the late-open ruling rests on: Christmas 2022 fell on a
/// Sunday, so the observed closure on Monday 2022-12-26 carried the operator's
/// ordinary 19:00 CT evening leg for trade date 2022-12-27, and 2022-12-27
/// therefore ships no late open and opens normally.
#[test]
fn era_2022_12_27_opens_normally_because_the_holiday_carried_the_evening_leg() {
    let calendar = calendar_for_market_hours_key(ZC);

    assert_eq!(
        calendar.holiday_on(day((2022, 12, 26))).map(Holiday::kind),
        Some(HolidayKind::Closed)
    );
    assert_eq!(
        calendar.holiday_on(day((2022, 12, 27))),
        None,
        "the evening leg on the holiday means this trade date opens normally"
    );

    // The holiday's own 19:00 CT leg runs and carries 2022-12-27.
    assert!(!open_at(ct((2022, 12, 26), (18, 59, 59))));
    assert!(open_at(ct((2022, 12, 26), (19, 0, 0))));
    assert_eq!(
        calendar.trade_date(ct((2022, 12, 26), (19, 0, 0))),
        Some(day((2022, 12, 27)))
    );

    // 2022-12-27 is an ordinary trading day: the overnight leg runs to its
    // 07:45 CT pause, the 08:30-13:20 CT day session runs in full, and 13:20
    // ends the trade date.
    assert!(open_at(ct((2022, 12, 27), (7, 44, 0))));
    assert!(!open_at(ct((2022, 12, 27), (8, 0, 0))));
    assert!(open_at(ct((2022, 12, 27), (8, 30, 0))));
    assert!(open_at(ct((2022, 12, 27), (13, 19, 59))));
    assert!(!open_at(ct((2022, 12, 27), (13, 20, 0))));
    assert_eq!(
        calendar.session_bounds(ct((2022, 12, 27), (9, 0, 0))),
        Some((
            ct((2022, 12, 27), (8, 30, 0)),
            ct((2022, 12, 27), (13, 20, 0))
        ))
    );
    assert_eq!(
        calendar.next_session_open_after(ct((2022, 12, 26), (10, 0, 0))),
        Some(ct((2022, 12, 26), (19, 0, 0))),
        "the closure's next session is the ordinary evening open"
    );

    // The contrast: Christmas 2024's closure carried no evening leg, so its
    // successor is one of the six late opens.
    assert!(!open_at(ct((2024, 12, 25), (19, 0, 0))));
    assert_eq!(
        calendar.holiday_on(day((2024, 12, 26))).map(Holiday::kind),
        Some(HolidayKind::LateOpen {
            open_ssm: ERA_DAY_OPEN
        })
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and the next session the family offers is its ordinary 19:00 CT
/// evening open — except where the successor itself ships a late open, which
/// removes that evening leg too and reopens at 08:30 CT. Both are named here so
/// a shifted reopen fails.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = calendar_for_market_hours_key(ZC);
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
        assert!(
            !open_at(ct_at(day_before(date), ERA_EVENING_OPEN)),
            "{date}"
        );
        assert!(!open_at(ct_on(day_before(date), (19, 30, 0))), "{date}");
        // And so is the trade date's own session: overnight, day and close.
        assert!(!open_at(ct_on(date, (2, 0, 0))), "{date}");
        assert!(!open_at(ct_on(date, (9, 0, 0))), "{date}");
        assert!(!open_at(ct_on(date, (13, 19, 0))), "{date}");
        assert_eq!(calendar.trade_date(ct_on(date, (10, 0, 0))), None, "{date}");

        let reopen = era_reopen_after_closure(date);
        assert_eq!(
            calendar.next_session_open_after(ct_on(date, (10, 0, 0))),
            Some(reopen),
            "{date}: the next session is the ordinary evening open"
        );
        // The whole civil day is closed only where no evening leg survives:
        // after a Friday closure, and where the successor opens late.
        let successor_moves_the_open = era_successor_opens_late(date);
        assert_eq!(
            calendar.is_closed_all_day_on(date, SessionKind::Both),
            successor_moves_the_open || date.weekday() == Weekday::Fri,
            "{date}"
        );
        if reopen == ct_at(date, ERA_EVENING_OPEN) {
            assert_eq!(calendar.trade_date(reopen), Some(day_after(date)), "{date}");
        }
    }
    assert_eq!(closures, 26, "the era's closures");
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = calendar_for_market_hours_key(ZC);
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
        ct_on(day_before(date), (19, 30, 0)),
        ct_on(date, (9, 0, 0)),
        ct_on(date, (13, 19, 0)),
        ct_on(date, (19, 30, 0)),
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
    let calendar = calendar_for_market_hours_key(ZC);
    for date in [(2023, 1, 16), (2023, 2, 20), (2023, 4, 7)] {
        let date = day(date);
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
    let calendar = calendar_for_market_hours_key(ZC);
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
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

    for date in [(2013, 6, 14), (2015, 12, 25)] {
        assert!(!coverage.contains(day(date)), "{date:?}");
        assert_eq!(calendar.holiday_on(day(date)), None, "{date:?}");
    }
    // Christmas 2015 is a real CME closure no wave audited; the ordinary
    // Friday answers, and the detached calendar answers it the same.
    for probe in [
        ct((2015, 12, 25), (9, 0, 0)),
        ct((2015, 12, 24), (19, 30, 0)),
    ] {
        assert!(open_at(probe), "{probe}");
        assert_eq!(calendar.is_open(probe), bare.is_open(probe), "{probe}");
    }
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. Each
/// branch's own fence lives in the `assert_era_*` helper it calls.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = calendar_for_market_hours_key(ZC);
    let (mut late_opens, mut combined, mut early_closes) = (0_usize, 0_usize, 0_usize);
    let (mut closures, mut unsourced) = (0_usize, 0_usize);
    let mut rows = 0_usize;
    let mut date = day((2019, 1, 1));
    while date <= day((2021, 12, 31)) {
        if let Some(row) = calendar.holiday_on(date) {
            assert_eq!(row.tier(), EvidenceTier::T1, "{date}");
            assert!(!row.document_id().is_empty(), "{date} cites no artifact");
            match row.kind() {
                HolidayKind::EarlyClose { close_ssm } => {
                    early_closes += 1;
                    assert_eq!(close_ssm, ERA_HALF_DAY_FIVE_PAST, "{date}");
                    assert_era_early_close(calendar, date);
                }
                HolidayKind::LateOpen { open_ssm } => {
                    late_opens += 1;
                    assert_eq!(open_ssm, ERA_DAY_OPEN, "{date}");
                    assert_era_late_open(calendar, date);
                }
                HolidayKind::LateOpenAndEarlyClose {
                    open_ssm,
                    close_ssm,
                } => {
                    combined += 1;
                    assert_eq!(
                        (open_ssm, close_ssm),
                        (ERA_DAY_OPEN, ERA_HALF_DAY_FIVE_PAST),
                        "{date}"
                    );
                    assert_era_late_open_and_early_close(calendar, date);
                }
                HolidayKind::Closed => {
                    closures += 1;
                    assert!(
                        calendar.is_closed_trade_date(date, SessionKind::Both),
                        "{date}"
                    );
                    assert_era_closure(calendar, date);
                }
                HolidayKind::Unsourced => unsourced += 1,
                other => panic!("{date}: this era ships no {other:?}"),
            }
            rows += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(rows, 42, "the era's rows");
    assert_eq!(
        (late_opens, combined, early_closes, closures, unsourced),
        (5, 3, 4, 27, 3),
        "the era's shape"
    );
}

/// An early close ends the day session at the printed instant and clips the
/// evening leg that opened the trade date rather than deleting it.
fn assert_era_early_close(calendar: ExchangeCalendar, date: NaiveDate) {
    let cutoff = ct_at(date, ERA_HALF_DAY_FIVE_PAST);
    // The evening leg that opened this trade date is clipped, not deleted, and
    // it still carries the trade date.
    assert!(open_at(ct_at(day_before(date), ERA_EVENING_OPEN)), "{date}");
    assert_eq!(
        calendar.trade_date(ct_on(day_before(date), (19, 30, 0))),
        Some(date),
        "{date}"
    );
    // One second before the close is open; at it, closed.
    assert!(open_at(cutoff - Duration::seconds(1)), "{date}");
    assert!(!open_at(cutoff), "{date}: end-exclusive");
    // The day session's bounds end at the printed instant, and so does the
    // daily candle. The probe sits just inside the session, so the printed
    // close cannot make the query answer `None` instead.
    let inside = cutoff - Duration::minutes(1);
    assert_eq!(
        calendar.session_bounds(inside),
        Some((ct_on(date, (8, 30, 0)), cutoff)),
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
    // The ordinary 13:20 CT day-session close is not reached.
    assert!(!open_at(ct_at(date, ERA_DAY_CLOSE)), "{date}");
}

/// A late open deletes the prior-evening leg the closure before it printed no
/// reopen for, and the day session that follows opens at 08:30 CT on its own
/// civil date and still ends at the ordinary 13:20 CT close.
fn assert_era_late_open(calendar: ExchangeCalendar, date: NaiveDate) {
    let open = ct_on(date, (8, 30, 0));
    assert!(
        !open_at(ct_at(day_before(date), ERA_EVENING_OPEN)),
        "{date}"
    );
    assert!(!open_at(ct_on(day_before(date), (19, 30, 0))), "{date}");
    assert!(!open_at(open - Duration::seconds(1)), "{date}");
    assert!(open_at(open), "{date}");
    assert_eq!(
        calendar.session_bounds(open),
        Some((open, ct_at(date, ERA_DAY_CLOSE))),
        "{date}"
    );
    assert!(
        open_at(ct_at(date, ERA_DAY_CLOSE) - Duration::seconds(1)),
        "{date}"
    );
    assert!(!open_at(ct_at(date, ERA_DAY_CLOSE)), "{date}");
    assert_eq!(calendar.trade_date(open), Some(date), "{date}");
}

/// The day-after-Thanksgiving shape: neither boundary is reachable from the
/// ordinary grid — no prior-evening leg, no 07:45 CT pause — and the single
/// block runs 08:30 to 12:05 CT.
fn assert_era_late_open_and_early_close(calendar: ExchangeCalendar, date: NaiveDate) {
    let open = ct_on(date, (8, 30, 0));
    let close = ct_on(date, (12, 5, 0));
    assert!(
        !open_at(ct_at(day_before(date), ERA_EVENING_OPEN)),
        "{date}"
    );
    assert!(!open_at(ct_on(date, (7, 45, 0))), "{date}");
    assert!(!open_at(open - Duration::seconds(1)), "{date}");
    assert!(open_at(open), "{date}");
    assert!(open_at(close - Duration::seconds(1)), "{date}");
    assert!(!open_at(close), "{date}: end-exclusive");
    assert_eq!(calendar.session_bounds(open), Some((open, close)), "{date}");
    assert_eq!(calendar.trade_date(open), Some(date), "{date}");
}

/// A closure takes the whole trade date — the evening leg that would have
/// carried it, the overnight session, the day session and the close.
fn assert_era_closure(calendar: ExchangeCalendar, date: NaiveDate) {
    assert!(
        !open_at(ct_at(day_before(date), ERA_EVENING_OPEN)),
        "{date}"
    );
    assert!(!open_at(ct_on(day_before(date), (19, 30, 0))), "{date}");
    assert!(!open_at(ct_on(date, (2, 0, 0))), "{date}");
    assert!(!open_at(ct_on(date, (9, 0, 0))), "{date}");
    assert!(!open_at(ct_on(date, (13, 19, 0))), "{date}");
    assert_eq!(calendar.trade_date(ct_on(date, (10, 0, 0))), None, "{date}");
}

/// The day-after-closure shapes this wave derives, date for date: five 08:30 CT
/// late opens and three 08:30-12:05 CT blocks on the Thanksgiving Fridays, each
/// paired with the closure the operator printed beside it and neither reachable
/// from the ordinary 19:00-13:20 CT grid.
#[test]
fn era_2019_2021_day_after_closure_shapes_are_the_derived_dates() {
    let calendar = calendar_for_market_hours_key(ZC);

    for (date, closure) in [
        ((2019, 1, 2), (2019, 1, 1)),
        ((2019, 7, 5), (2019, 7, 4)),
        ((2019, 12, 26), (2019, 12, 25)),
        ((2020, 1, 2), (2020, 1, 1)),
        ((2021, 7, 6), (2021, 7, 5)),
    ] {
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpen {
                open_ssm: ERA_DAY_OPEN
            }),
            "{date:?}"
        );
        assert_eq!(
            calendar.holiday_on(day(closure)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{closure:?}"
        );
        // No evening leg survives the closure, so the successor's day session
        // opens at 08:30 CT on its own civil date.
        assert!(!open_at(ct(closure, (19, 0, 0))), "{date:?}");
        assert!(!open_at(ct(date, (8, 29, 59))), "{date:?}");
        assert!(open_at(ct(date, (8, 30, 0))), "{date:?}");
    }

    for (date, thanksgiving) in [
        ((2019, 11, 29), (2019, 11, 28)),
        ((2020, 11, 27), (2020, 11, 26)),
        ((2021, 11, 26), (2021, 11, 25)),
    ] {
        let open = ct(date, (8, 30, 0));
        let close = ct(date, (12, 5, 0));
        assert_eq!(
            calendar.holiday_on(day(date)).map(Holiday::kind),
            Some(HolidayKind::LateOpenAndEarlyClose {
                open_ssm: ERA_DAY_OPEN,
                close_ssm: ERA_HALF_DAY_FIVE_PAST,
            }),
            "{date:?}"
        );
        assert_eq!(
            calendar.holiday_on(day(thanksgiving)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{thanksgiving:?}"
        );
        assert!(!open_at(ct(thanksgiving, (19, 0, 0))), "{date:?}");
        assert!(!open_at(ct(date, (7, 45, 0))), "{date:?}");
        assert!(!open_at(open - Duration::seconds(1)), "{date:?}");
        assert!(open_at(open), "{date:?}");
        assert!(open_at(close - Duration::seconds(1)), "{date:?}");
        assert!(!open_at(close), "{date:?}");
        assert_eq!(
            calendar.session_bounds(open),
            Some((open, close)),
            "{date:?}"
        );
        assert_eq!(
            calendar.trade_date(close - Duration::seconds(1)),
            Some(day(date))
        );
    }
}

/// Every `Unsourced` row the era ships changes no answer: the row states that
/// the date was audited, makes no scheduling claim, and clips nothing.
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = calendar_for_market_hours_key(ZC);
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
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");
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
    assert!(open_at(ct((2021, 12, 31), (9, 0, 0))));
    // The neighbouring dates, which other waves audit, carry no row here.
    assert_eq!(calendar.holiday_on(day((2018, 12, 31))), None);
    assert_eq!(calendar.holiday_on(day((2022, 1, 1))), None);
}

/// The family's coverage names its windows in order, and the 2019-2021 window
/// is one of them: every row the era ships lies inside it, and no row ships on
/// the era's outer neighbours.
#[test]
fn era_2019_2021_window_is_declared_in_order_and_bounds_every_row() {
    let calendar = calendar_for_market_hours_key(ZC);
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_grains ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day((2010, 1, 1)), day((2012, 12, 31))),
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
    assert_eq!(rows, 42, "the era's rows");
    for probe in [day((2018, 12, 31)), day((2022, 1, 1))] {
        assert_eq!(calendar.holiday_on(probe), None, "{probe}");
    }
}
