// SPDX-License-Identifier: MIT-0

//! CME Nikkei 225 Dollar (`NKD`): the built-in holiday table, 2025-2027.
//!
//! Every probe is stated in `America/Chicago` wall clock and converted, because
//! that is the clock CME publishes in and the clock the table's trade dates are
//! keyed to. The family runs one wrapped envelope per trade date — 17:00 CT on
//! the previous evening to 16:00 CT on the trade date — so an early close is
//! always a clip of a session that opened the day before, and a closure always
//! deletes a previous-evening leg. Both shapes are fenced here.
//!
//! The seven cases the design memo requires of every shipped family are
//! `a_closed_trade_date_*`, `an_early_close_*` (before and at),
//! `the_table_ships_no_late_open`, `a_closure_removes_the_previous_evening_wrap`,
//! `the_trade_date_follows_the_clipped_close`, `holiday_coverage_*` and
//! `without_holidays_restores_the_normal_week`.

use chrono::{DateTime, Datelike as _, Days, Duration, NaiveDate, TimeZone as _, Utc, Weekday};
use chrono_tz::US;
use exchange_hours::{
    CalendarQueryError, CalendarResolution, EvidenceTier, ExceptionBlockKind, ExchangeCalendar,
    Holiday, HolidayKind, MarketHoursKey, SessionKind, SessionState, calendar_for_market_hours_key,
};

/// The family calendar under test, with its built-in table attached.
fn nkd() -> ExchangeCalendar {
    calendar_for_market_hours_key(MarketHoursKey::GlobexNikkei225Dollar)
}

/// A `America/Chicago` wall clock, converted to the UTC instant the public
/// surface takes.
fn ct(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> chrono::DateTime<Utc> {
    US::Central
        .with_ymd_and_hms(year, month, day, hour, minute, second)
        .single()
        .expect("fixture must name an unambiguous Chicago wall clock")
        .with_timezone(&Utc)
}

fn day(year: i32, month: u32, date: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, date).expect("fixture must be a valid date")
}

/// The kind of the built-in row on `date`, or `None` when the table holds no
/// row for it. The orphan rule keeps this a function rather than a `From`.
fn kind_on(calendar: ExchangeCalendar, date: NaiveDate) -> Option<HolidayKind> {
    calendar.holiday_on(date).map(Holiday::kind)
}

/// Case 1 — a closed trade date.
///
/// Christmas 2025 falls on a Thursday. CME publishes no close on it: the
/// Wednesday-evening leg that would have carried trade date 2025-12-25 is
/// absent, and the 17:00 CT open that evening already carries 2025-12-26.
/// `Closed(2025-12-25)` is what says "there is no such trade date".
#[test]
fn a_closed_trade_date_deletes_its_whole_trading_day() {
    let nkd = nkd();
    let christmas = day(2025, 12, 25);

    assert_eq!(
        kind_on(nkd, christmas),
        Some(HolidayKind::Closed),
        "2025-12-25 must ship as a full closure"
    );
    assert!(
        nkd.is_closed_trade_date(christmas, SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );

    for (hour, minute) in [(0, 30), (10, 0), (15, 59)] {
        assert!(
            !nkd.is_open(ct(2025, 12, 25, hour, minute, 0))
                .expect("the coverage contract must answer a covered date"),
            "no session belongs to trade date 2025-12-25, so {hour:02}:{minute:02} CT is closed"
        );
    }

    // The previous evening's normal 17:00 CT open is gone with it: that
    // occurrence's trade date was the closed Thursday.
    assert!(
        !nkd.is_open(ct(2025, 12, 24, 17, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "the Wednesday-evening leg carried trade date 2025-12-25 and must be deleted"
    );

    // ...while the Thursday-evening leg, whose trade date is Friday, survives.
    assert!(
        nkd.is_open(ct(2025, 12, 25, 18, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "the 17:00 CT open on Christmas Day carries trade date 2025-12-26 and still runs"
    );
    assert!(
        !nkd.is_closed_all_day_on(christmas, SessionKind::Both)
            .expect("the coverage contract must answer a covered date"),
        "a full closure of a trade date is not a closed civil day: the next trade \
         date's session opens that evening"
    );
}

/// Case 1, continued — Good Friday 2025, the closure taken from the Equity
/// Index line of the same capture because CME publishes no Nikkei line for
/// that window.
#[test]
fn the_good_friday_2025_closure_skips_to_the_sunday_open() {
    let nkd = nkd();

    assert_eq!(kind_on(nkd, day(2025, 4, 18)), Some(HolidayKind::Closed));
    assert!(
        !nkd.is_open(ct(2025, 4, 18, 9, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !nkd.is_open(ct(2025, 4, 17, 17, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "the Thursday-evening leg carried the closed Friday and must be deleted"
    );
    assert_eq!(
        nkd.next_session_after(ct(2025, 4, 17, 12, 0, 0))
            .expect("the coverage contract must answer a covered date")
            .map(|(open, _close)| open),
        Some(ct(2025, 4, 20, 17, 0, 0)),
        "the next session after the eve is the Sunday-evening open for trade date 2025-04-21"
    );
}

/// Case 2 — an early close, one second before the cutoff.
///
/// Christmas Eve 2025 is a Wednesday whose session opened Tuesday 17:00 CT.
/// The clip is stated on the trade date, so it lands on the Wednesday.
#[test]
fn an_early_close_is_open_one_second_before_its_cutoff() {
    let nkd = nkd();

    assert_eq!(
        kind_on(nkd, day(2025, 12, 24)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60
        })
    );
    assert!(
        nkd.is_open(ct(2025, 12, 24, 12, 14, 59))
            .expect("the coverage contract must answer a covered date"),
        "12:14:59 CT is inside the 12:15 CT early close"
    );
    assert!(
        nkd.is_open(ct(2025, 12, 24, 3, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "the overnight leg of the shortened day is untouched"
    );
}

/// Case 3 — an early close, at the cutoff.
///
/// Closes are end-exclusive, so the cutoff instant itself is closed, the
/// session bounds end exactly there, and the daily bar closes with them.
#[test]
fn an_early_close_is_shut_at_its_cutoff_and_moves_the_session_bounds() {
    let nkd = nkd();
    let cutoff = ct(2025, 12, 24, 12, 15, 0);

    assert!(
        !nkd.is_open(cutoff)
            .expect("the coverage contract must answer a covered date"),
        "an instant equal to a close is closed"
    );
    assert_eq!(
        nkd.session_bounds(ct(2025, 12, 24, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some((ct(2025, 12, 23, 17, 0, 0), cutoff)),
        "the session that opened Tuesday 17:00 CT now ends at the clipped close"
    );
    assert_eq!(
        nkd.candle_end(ct(2025, 12, 24, 10, 0, 0), CalendarResolution::Daily)
            .expect("the coverage contract must answer a covered date"),
        Some(cutoff),
        "the daily bar closes with the trade date"
    );
    assert!(
        !nkd.is_open(ct(2025, 12, 24, 15, 59, 0))
            .expect("the coverage contract must answer a covered date"),
        "the normal 16:00 CT close is no longer reachable on this trade date"
    );
}

/// Case 3, continued — Good Friday 2026, the one date on which this family's
/// instant is not shared with its `Globex` neighbours.
///
/// `NKD` and `NIY` close 08:15 CT with the Equity Index line; `ZN`, `6E` and
/// `BTC` close 10:15 CT. Encoding the neighbours' instant here would leave the
/// contract reported open for two hours it was shut, so both are probed.
#[test]
fn good_friday_2026_closes_at_the_equity_index_instant_not_the_rates_instant() {
    let nkd = nkd();

    assert_eq!(
        kind_on(nkd, day(2026, 4, 3)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 8 * 3_600 + 15 * 60
        })
    );
    assert!(
        nkd.is_open(ct(2026, 4, 3, 8, 14, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !nkd.is_open(ct(2026, 4, 3, 8, 15, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !nkd.is_open(ct(2026, 4, 3, 10, 14, 0))
            .expect("the coverage contract must answer a covered date"),
        "10:15 CT is the Interest Rates, FX and Cryptocurrency instant, not this family's"
    );
    assert_eq!(
        nkd.next_session_after(ct(2026, 4, 3, 9, 0, 0))
            .expect("the coverage contract must answer a covered date")
            .map(|(open, _close)| open),
        Some(ct(2026, 4, 5, 17, 0, 0)),
        "the grid has no Friday-evening open, so the next session is Sunday's"
    );
}

/// Case 4 — the 2025-2027 era ships no late open, and the table says so.
///
/// Every CME re-open in that era is the grid's own 17:00 CT evening open, so
/// there is no row to push an occurrence later. The memo's late-open case is
/// therefore vacuous there, and this is the fence that keeps it honest: it
/// walks every date of that era, so a late open introduced by a later edit
/// fails immediately rather than going untested. The 2016-2018 era's one late
/// open — 2018-12-26 at 15:30 CT — is asserted by
/// `wave2_early_close_and_late_open_bounds_are_end_and_start_exclusive`.
#[test]
fn the_table_ships_no_late_open_and_holds_exactly_its_audited_rows() {
    let nkd = nkd();

    let mut closed = 0_usize;
    let mut early = 0_usize;
    let mut date = day(2025, 1, 1);
    let last = day(2027, 12, 31);
    while date <= last {
        if let Some(holiday) = nkd.holiday_on(date) {
            assert_eq!(
                holiday.tier(),
                EvidenceTier::T2,
                "{date} is sourced from CME's own trading-hours service, which is T2"
            );
            assert!(
                holiday.document_id().starts_with("CME-SVC-"),
                "{date} must cite a CME trading-hours-service artifact: {}",
                holiday.document_id()
            );
            match holiday.kind() {
                HolidayKind::Closed => closed += 1,
                HolidayKind::EarlyClose { .. } => early += 1,
                // A complete-day replacement for a Saturday session: neither a
                // closure nor a boundary move, so it counts in neither column.
                HolidayKind::ReplacementBlocks(_) => {}
                other => panic!("{date} ships an unexpected holiday kind: {other:?}"),
            }
        }
        date = date.succ_opt().expect("coverage stays inside the calendar");
    }

    assert_eq!(closed, 9, "nine full closures across 2025-2027");
    // Twenty-five: the three day-after-Thanksgiving Fridays became replacement
    // rows, because Thanksgiving Day publishes no final close of its own and the
    // span now carries the Friday's trade date.
    assert_eq!(early, 25, "twenty-five early closes across 2025-2027");

    // The 2016-2018 era is the other shape: 34 rows, all T1.
    let (mut closed, mut early, mut late) = (0_usize, 0_usize, 0_usize);
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        if let Some(holiday) = nkd.holiday_on(date) {
            assert_eq!(
                holiday.tier(),
                EvidenceTier::T1,
                "{date} is a CME published schedule"
            );
            match holiday.kind() {
                HolidayKind::Closed => closed += 1,
                HolidayKind::EarlyClose { .. } => early += 1,
                HolidayKind::LateOpen { .. } => late += 1,
                other => panic!("{date} ships an unexpected holiday kind: {other:?}"),
            }
        }
        date = date.succ_opt().expect("coverage stays inside the calendar");
    }
    assert_eq!(
        (closed, early, late),
        (9, 24, 1),
        "nine closures, twenty-four early closes and one late open across 2016-2018"
    );

    // Both post-closure reopens the family has, fenced on each side of the
    // ordinary 17:00 CT open: a late open would move one of them.
    assert!(
        !nkd.is_open(ct(2025, 12, 25, 16, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        nkd.is_open(ct(2025, 12, 25, 17, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "Christmas evening reopens at the grid's own 17:00 CT, not later"
    );
    assert!(
        !nkd.is_open(ct(2027, 3, 28, 16, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        nkd.is_open(ct(2027, 3, 28, 17, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "the Sunday after Good Friday 2027 reopens at the grid's own 17:00 CT"
    );
}

/// Case 5 — a closure removes the previous evening's wrap.
///
/// The Christmas shape, stated the way the memo states it: nothing at 17:30 CT
/// on the eve, and the next session after the eve's clipped close is the
/// following evening's open, not the eve's own.
#[test]
fn a_closure_removes_the_previous_evening_wrap() {
    let nkd = nkd();

    assert!(
        !nkd.is_open(ct(2025, 12, 24, 17, 30, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        nkd.next_session_after(ct(2025, 12, 24, 12, 20, 0))
            .expect("the coverage contract must answer a covered date"),
        Some((ct(2025, 12, 25, 17, 0, 0), ct(2025, 12, 26, 16, 0, 0))),
        "the eve's own 17:00 CT open is gone; the next one is Christmas evening's, \
         which carries trade date 2025-12-26"
    );
}

/// Case 6 — the trade-date consequence.
///
/// Inside a shortened day the trade date is still the holiday's own; at the
/// eve's evening open it is already the post-holiday date. Both branches are
/// data-dependent and a mis-keyed row silently moves a whole day of bars.
#[test]
fn the_trade_date_follows_the_clipped_close() {
    let nkd = nkd();

    assert_eq!(
        nkd.trade_date(ct(2025, 12, 24, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 24)),
        "a shortened day keeps its own trade date"
    );
    assert_eq!(
        nkd.trade_date(ct(2025, 12, 25, 18, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 12, 26)),
        "the evening open on the closed date already belongs to the next trade date"
    );

    // A Monday holiday, where the clipped session opened on the Sunday.
    assert!(
        nkd.is_open(ct(2026, 1, 19, 11, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !nkd.is_open(ct(2026, 1, 19, 12, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    // The span through the holiday's own morning carries the *next* trade date:
    // CME publishes no final close for the holiday, so the operator labels the
    // whole Sunday-evening-to-Tuesday afternoon span with 2026-01-20.
    assert_eq!(
        nkd.trade_date(ct(2026, 1, 19, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 1, 20))
    );
    assert_eq!(
        nkd.trade_date(ct(2026, 1, 19, 18, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2026, 1, 20)),
        "the Monday-evening leg runs normally into Tuesday's trade date"
    );
    assert!(
        nkd.is_open(ct(2026, 1, 19, 17, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "an early close ends a trade date; it does not shut the evening re-open"
    );
}

/// Case 6, continued — the 2025 rows taken from the Equity Index line.
///
/// CME's service no longer answers for the eight windows through Labor Day
/// 2025, so `NKD` and `NIY` publish nothing for them and those rows are the
/// Equity Index line of the same ten-product capture. They must still clip.
#[test]
fn the_2025_rows_inferred_from_the_equity_index_line_clip_the_same_way() {
    let nkd = nkd();

    // Martin Luther King Jr. Day 2025, a Monday: 12:00 CT.
    assert!(
        nkd.is_open(ct(2025, 1, 20, 11, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !nkd.is_open(ct(2025, 1, 20, 12, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    // Independence Day eve 2025, the one date where the Equity Index line
    // diverges from Rates, FX, Energy, Metals and Cryptocurrency: 12:15 CT
    // against their 16:00 CT.
    assert!(
        nkd.is_open(ct(2025, 7, 3, 12, 14, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !nkd.is_open(ct(2025, 7, 3, 12, 15, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        nkd.is_open(ct(2025, 7, 3, 17, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "the evening leg runs normally into trade date 2025-07-04"
    );
    // ...which is itself an early close at 12:00 CT.
    assert!(
        nkd.is_open(ct(2025, 7, 4, 11, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !nkd.is_open(ct(2025, 7, 4, 12, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
}

/// Case 7 — both edges of the coverage window.
///
/// Inside the window a date with no row is audited normal; outside it the
/// table has no holiday answer. Christmas 2015 is the 2013-2015 window's own
/// closure since the 2026-09-29 wave backfilled 2011-2015, and 2010-01-01 —
/// the first window's opening day since the #225 remainder moved its start to
/// the floor on 2026-09-30 UTC — is the old-grid era's own closure.
#[test]
fn holiday_coverage_bounds_what_the_table_answers_for() {
    let nkd = nkd();
    let coverage = nkd
        .holiday_coverage()
        .expect("the family ships a built-in table");

    assert_eq!(coverage.first(), day(2010, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2026, 4, 3)));
    assert!(coverage.contains(day(2016, 1, 1)));
    // 2015-12-31 entered the table with the 2011-2015 wave: it is the last
    // audited date of the 2013-2015 window.
    assert!(coverage.contains(day(2015, 12, 31)));
    // 2019-01-01 entered the table with the 2019-2021 wave: it is the New Year
    // closure the crate served as an ordinary Tuesday before that wave.
    assert!(coverage.contains(day(2019, 1, 1)));
    // 2024-12-31 entered the table with the 2022-2024 wave, as an `Unsourced`
    // statement: inside a window, so `contains` is true and the row is the
    // family's own note that no document covered the date.
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    // 2015-12-31 is the 2013-2015 window's last audited date since the
    // 2011-2015 wave shipped, and it ships no row: audited normal.
    assert_eq!(kind_on(nkd, day(2015, 12, 31)), None);
    assert_eq!(kind_on(nkd, day(2019, 1, 1)), Some(HolidayKind::Closed));
    assert_eq!(
        kind_on(nkd, day(2024, 12, 31)),
        Some(HolidayKind::Unsourced)
    );
    assert!(nkd.holiday_on(day(2028, 1, 1)).is_none());
    assert!(
        nkd.holiday_on(day(2010, 4, 11)).is_none(),
        "the notice-dated grid start is inside the window and audited normal"
    );
    assert_eq!(
        kind_on(nkd, day(2010, 12, 24)),
        Some(HolidayKind::Closed),
        "the 2010 rows entered with the 2026-09-30 wave"
    );
    // The #225 remainder joined the window on 2026-09-30 UTC: 2010-01-01 is
    // the old-grid era's one closure and the rest of Q1 2010 is audited normal
    // on the modelled old grid.
    assert!(coverage.contains(day(2010, 1, 1)));
    assert!(coverage.contains(day(2010, 2, 15)));
    assert_eq!(
        kind_on(nkd, day(2010, 1, 1)),
        Some(HolidayKind::Closed),
        "the New Year sheet's venue-wide closure is the era's only row"
    );
    assert_eq!(
        kind_on(nkd, day(2010, 2, 15)),
        None,
        "the President's Day sheet excepts NKD onto its regular grid, so no row ships"
    );

    // Christmas 2015 is a shipped `Closed` row since the 2011-2015 wave: the
    // attached identity answers it closed, and the detached calendar states
    // the ordinary week. (Christmas 2021, which this probe used before the
    // 2019-2021 wave shipped, is also a shipped `Closed` row now.)
    assert_eq!(kind_on(nkd, day(2015, 12, 25)), Some(HolidayKind::Closed));
    assert!(
        !nkd.is_open(ct(2015, 12, 25, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "2015-12-25 is the 2013-2015 window's own closure"
    );
    assert!(
        nkd.without_holidays()
            .is_open(ct(2015, 12, 25, 10, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );

    // The day before the window opens is below the support floor, so both the
    // attached identity and the detached snapshot refuse the date as pre-floor
    // rather than answering it.
    let probe = ct(2009, 12, 31, 10, 0, 0);
    assert!(
        nkd.is_open(probe)
            .is_err_and(|error| matches!(error, CalendarQueryError::BeforeSupportFloor { .. })),
        "{probe} is below the 2010 floor"
    );
    assert!(
        nkd.without_holidays()
            .is_open(probe)
            .is_err_and(|error| matches!(error, CalendarQueryError::BeforeSupportFloor { .. })),
        "{probe}: the detached snapshot is identity-backed and refuses the floor too"
    );
}

/// `without_holidays()` restores exactly the pre-table answer.
///
/// The control is the crate itself: the week of 2025-12-15 carries no row, so
/// an attached calendar answers the normal week there. A detached calendar over
/// the Christmas week must agree with it instant for instant at the same
/// Chicago wall clock — both weeks are `CST`, so a whole-week UTC shift is the
/// same wall clock — while the attached calendar must not.
#[test]
fn without_holidays_restores_the_normal_week() {
    let nkd = nkd();
    let bare = nkd.without_holidays();

    assert!(bare.holiday_on(day(2025, 12, 25)).is_none());
    assert!(bare.holiday_coverage().is_none());
    assert!(
        bare.is_open(ct(2025, 12, 25, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "detached, Christmas Day is an ordinary Thursday trade date"
    );
    assert!(
        bare.is_open(ct(2025, 12, 24, 15, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );

    let week = Duration::days(7);
    let mut probe = ct(2025, 12, 22, 0, 0, 0);
    let end = ct(2025, 12, 27, 0, 0, 0);
    let mut diverged = false;
    while probe < end {
        assert_eq!(
            bare.is_open(probe)
                .expect("the coverage contract must answer a covered date"),
            nkd.is_open(probe - week)
                .expect("the coverage contract must answer a covered date"),
            "a detached calendar must answer the normal week at {probe}"
        );
        diverged |= nkd
            .is_open(probe)
            .expect("the coverage contract must answer a covered date")
            != bare
                .is_open(probe)
                .expect("the coverage contract must answer a covered date");
        probe += Duration::minutes(15);
    }
    assert!(
        diverged,
        "the attached calendar must differ somewhere in the Christmas week, or \
         this fence proves nothing"
    );
}

/// The 2026 and 2027 half-days, probed on both sides of every distinct instant
/// the table ships.
///
/// A one-minute slip in any of these rows flips one of these assertions, which
/// is the mutation check the design memo asks for.
#[test]
fn every_shipped_early_close_instant_is_fenced_on_both_sides() {
    let nkd = nkd();

    // 12:00 CT — the Monday and Thursday holidays, and the two Friday
    // observances whose close CME labels with the following Monday.
    for (year, month, date) in [
        (2025, 2, 17),
        (2025, 5, 26),
        (2025, 6, 19),
        (2025, 9, 1),
        (2025, 11, 27),
        (2026, 2, 16),
        (2026, 5, 25),
        (2026, 6, 19),
        (2026, 7, 3),
        (2026, 9, 7),
        (2026, 11, 26),
        (2027, 1, 18),
        (2027, 2, 15),
        (2027, 5, 31),
        (2027, 6, 18),
        (2027, 7, 5),
        (2027, 9, 6),
        (2027, 11, 25),
    ] {
        assert!(
            nkd.is_open(ct(year, month, date, 11, 59, 59))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} closes at 12:00 CT, so 11:59:59 is open"
        );
        assert!(
            !nkd.is_open(ct(year, month, date, 12, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} closes at 12:00 CT"
        );
    }

    // 12:15 CT — the two Christmas Eves and the three days after Thanksgiving.
    for (year, month, date) in [
        (2025, 11, 28),
        (2026, 11, 27),
        (2026, 12, 24),
        (2027, 11, 26),
    ] {
        assert!(
            nkd.is_open(ct(year, month, date, 12, 14, 59))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} closes at 12:15 CT, so 12:14:59 is open"
        );
        assert!(
            !nkd.is_open(ct(year, month, date, 12, 15, 0))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} closes at 12:15 CT"
        );
    }
}

/// The remaining full closures, and the eves whose only deviation the
/// neighbouring closure already carries.
///
/// `2027-12-23` is CME's own Christmas 2027 holiday date and ships **no** row:
/// its daytime close is the ordinary 16:00 CT and what is missing is the
/// Thursday-evening leg, which `Closed(2027-12-24)` removes.
#[test]
fn the_year_end_closures_delete_only_the_legs_the_operator_deleted() {
    let nkd = nkd();

    for (year, month, date) in [
        (2025, 1, 1),
        (2026, 1, 1),
        (2026, 12, 25),
        (2027, 1, 1),
        (2027, 3, 26),
        (2027, 12, 24),
    ] {
        let closed = day(year, month, date);
        assert_eq!(
            kind_on(nkd, closed),
            Some(HolidayKind::Closed),
            "{closed} must ship as a full closure"
        );
        // 2025-01-01's derivations resolve the wrap that opened on the
        // withheld 2024-12-31, so the identity refuses them rather than
        // answering; the other closures' eves are audited.
        if (year, month, date) == (2025, 1, 1) {
            assert!(
                nkd.is_closed_trade_date(closed, SessionKind::Both)
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "2025-01-01 reads the withheld 2024-12-31 wrap"
            );
        } else {
            assert!(
                nkd.is_closed_trade_date(closed, SessionKind::Both)
                    .expect("the coverage contract must answer a covered date")
            );
        }
        if (year, month, date) == (2025, 1, 1) {
            assert!(
                nkd.is_open(ct(year, month, date, 10, 0, 0))
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "2025-01-01 reads the withheld 2024-12-31 wrap"
            );
        } else {
            assert!(
                !nkd.is_open(ct(year, month, date, 10, 0, 0))
                    .expect("the coverage contract must answer a covered date")
            );
        }
    }

    assert!(
        nkd.holiday_on(day(2027, 12, 23)).is_none(),
        "2027-12-23 is audited normal: its 16:00 CT close is the family's own"
    );
    assert!(
        nkd.is_open(ct(2027, 12, 23, 15, 59, 0))
            .expect("the coverage contract must answer a covered date"),
        "the Thursday daytime session runs to its normal 16:00 CT close"
    );
    assert!(
        !nkd.is_open(ct(2027, 12, 23, 17, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "the Thursday-evening leg carried the closed Friday and is deleted by that row"
    );
    assert_eq!(
        nkd.next_session_after(ct(2027, 12, 23, 16, 30, 0))
            .expect("the coverage contract must answer a covered date")
            .map(|(open, _close)| open),
        Some(ct(2027, 12, 26, 17, 0, 0)),
        "the next session is the Sunday-evening open for trade date 2027-12-27"
    );
}

/// The Saturday sessions CME publishes, which this table states as rows.
///
/// `2026-06-20`, `2026-07-04` and `2027-06-19` carry
/// `05:00 open; 17:00 closed` in CME's own service, on a week whose normal grid
/// has no Saturday session. Stage 4 (#116) gave the table the replacement-block
/// vocabulary, so each is a complete-day row keyed to the following Monday
/// (2026-06-22, 2026-07-06 and 2027-06-21) rather than a declared gap. This
/// fence records the session's own bounds with its end-exclusive close, the
/// trade date it carries, and that the row left the ordinary Sunday-Monday
/// session intact.
#[test]
fn the_published_saturday_sessions_ship_as_rows_on_the_following_monday() {
    let nkd = nkd();

    for ((year, month, date), (ty, tm, td)) in [
        ((2026, 6, 20), (2026, 6, 22)),
        ((2026, 7, 4), (2026, 7, 6)),
        ((2027, 6, 19), (2027, 6, 21)),
    ] {
        // The session's own bounds, not merely that it is open: the operator
        // prints `05:00 open; 17:00 closed`, so a row that moved either
        // boundary must fail here rather than pass on an interior probe.
        let open = ct(year, month, date, 5, 0, 0);
        let close = ct(year, month, date, 17, 0, 0);
        assert!(
            nkd.is_open(open)
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} is a sourced Saturday session and must be open at its own open"
        );
        assert_eq!(
            nkd.session_bounds(ct(year, month, date, 9, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            Some((open, close)),
            "{year}-{month:02}-{date:02}: the session's bounds are the published ones"
        );
        assert!(
            !nkd.is_open(close)
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02}: the close is end-exclusive"
        );
        assert_eq!(
            nkd.trade_date(ct(year, month, date, 9, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            Some(day(ty, tm, td)),
            "{year}-{month:02}-{date:02} must carry the following Monday"
        );
        // The ordinary Sunday-Monday session survives the row: its evening open
        // and its trade date's day session are both intact.
        let sunday = (year, month, date + 1);
        assert!(
            nkd.is_open(ct(sunday.0, sunday.1, sunday.2, 18, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02}: the Sunday-evening open was deleted by the row"
        );
        // The Thursday-evening leg before the Friday holiday belongs to this row
        // too: the operator prints the Friday noon early close against this trade
        // date, so the session that opened Thursday 17:00 CT ends here.
        let thursday = (year, month, date - 2);
        assert_eq!(
            nkd.trade_date(ct(thursday.0, thursday.1, thursday.2, 18, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            Some(day(ty, tm, td)),
            "{year}-{month:02}-{date:02}: the Thursday-evening leg before the holiday carries the Monday trade date"
        );
        assert!(
            nkd.is_open(ct(ty, tm, td, 10, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02}: the day session was deleted by the row"
        );
        assert!(
            !nkd.is_open(ct(ty, tm, td, 16, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02}: the final close is end-exclusive"
        );
        assert!(nkd.holiday_on(day(year, month, date)).is_none());
    }
}

/// The Sunday Pre-Open queue the same three rows state, fenced at its bounds.
///
/// CME prints `16:00 preopen; 17:00 open` on the Sunday between each published
/// Saturday session and the ordinary Sunday-evening session, and the family's
/// row states it as an order-entry block at offset `-1`. A queue is not a
/// session, so the row's `session_bounds` alone do not state it: this fence
/// pins the queue's own interval from both sides through `session_state`,
/// confirms through `is_order_entry_only` that the interval is a queue and not
/// a session, and confirms 17:00 CT hands it to the matching session.
///
/// The row states this queue on a Sunday whose Monday trade date carries a
/// published Saturday session; the family's ordinary Sunday and Monday-Thursday
/// queues are fenced in `cme_nikkei.rs`.
#[test]
fn the_published_sunday_pre_open_queues_are_fenced_at_their_bounds() {
    let nkd = nkd();

    for ((year, month, date), (ty, tm, td)) in [
        ((2026, 6, 20), (2026, 6, 22)),
        ((2026, 7, 4), (2026, 7, 6)),
        ((2027, 6, 19), (2027, 6, 21)),
    ] {
        // The row's first half: the Saturday session's own bounds, through the
        // same public query the sibling fence uses.
        assert_eq!(
            nkd.session_bounds(ct(year, month, date, 9, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            Some((
                ct(year, month, date, 5, 0, 0),
                ct(year, month, date, 17, 0, 0)
            )),
            "{year}-{month:02}-{date:02}: the Saturday session's bounds are the published ones"
        );

        // The queue's interval, `[16:00, 17:00)` CT on the Sunday. A start that
        // moved later fails the 16:00 and 16:59 probes; one that moved earlier
        // fails the 15:59 probe; an end that moved earlier fails the 16:59 one.
        let sunday = (year, month, date + 1);
        let state = |hour, minute, second| {
            nkd.session_state(ct(sunday.0, sunday.1, sunday.2, hour, minute, second))
                .expect("the coverage contract must answer a covered date")
        };
        assert_ne!(
            state(15, 59, 59),
            SessionState::OrderEntry,
            "{year}-{month:02}-{date:02}: no queue runs before its 16:00 CT open"
        );
        for (hour, minute, second) in [(16, 0, 0), (16, 59, 59)] {
            let instant = ct(sunday.0, sunday.1, sunday.2, hour, minute, second);
            assert_eq!(
                state(hour, minute, second),
                SessionState::OrderEntry,
                "{year}-{month:02}-{date:02}: the queue runs at {hour:02}:{minute:02}:{second:02} CT"
            );
            assert!(
                nkd.is_order_entry_only(instant)
                    .expect("the coverage contract must answer a covered date"),
                "{year}-{month:02}-{date:02}: the queue is order-entry-only at \
                 {hour:02}:{minute:02}:{second:02} CT"
            );
            assert!(
                !nkd.is_open(instant)
                    .expect("the coverage contract must answer a covered date"),
                "{year}-{month:02}-{date:02}: the queue matches no trade and stays out of is_open"
            );
        }
        // 17:00 CT is the queue's end-exclusive close and the matching
        // session's open: one instant, both statements. The matching session
        // is this family's `regular` one — the whole tradeable envelope is
        // its regular session, so the row states it `regular` since #176.
        assert_eq!(
            state(17, 0, 0),
            SessionState::OpenRegular,
            "{year}-{month:02}-{date:02}: 17:00 CT hands the queue to the matching session"
        );
        assert!(
            !nkd.is_order_entry_only(ct(sunday.0, sunday.1, sunday.2, 17, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02}: 17:00 CT is a tradeable session, not a queue"
        );
        assert_eq!(
            nkd.session_bounds(ct(sunday.0, sunday.1, sunday.2, 16, 30, 0))
                .expect("the coverage contract must answer a covered date"),
            Some((
                ct(sunday.0, sunday.1, sunday.2, 17, 0, 0),
                ct(ty, tm, td, 16, 0, 0)
            )),
            "{year}-{month:02}-{date:02}: the queue feeds the Sunday-17:00-to-Monday-16:00 session"
        );
    }
}

/// The three trade dates whose prior local day is a full closure, where the
/// operator's Pre-Open therefore opens at the 17:00 CT session's own 16:00 CT
/// boundary rather than at the weekday `16:45`, are served as order entry.
///
/// CME prints `16:00 preopen /TD <the trade date>; 17:00 open /TD <the trade
/// date>` on the closed day itself, so the queue belongs to the following trade
/// date and the holiday's own `Closed` row would otherwise delete it. The
/// window is fenced from both sides, and 17:00 CT is asserted to hand it to the
/// matching session rather than to a second queue.
#[test]
fn a_day_after_a_closure_serves_its_1600_pre_open() {
    let nkd = nkd();

    // (the closure the queue is printed on, the trade date it carries).
    for (closure, trade_date) in [
        ((2025, 1, 1), (2025, 1, 2)),
        ((2025, 12, 25), (2025, 12, 26)),
        ((2026, 1, 1), (2026, 1, 2)),
    ] {
        let (cy, cm, cd) = closure;
        let (ty, tm, td) = trade_date;
        assert_eq!(
            nkd.holiday_on(day(cy, cm, cd)).map(Holiday::kind),
            Some(HolidayKind::Closed),
            "{cy}-{cm:02}-{cd:02} is a full closure"
        );
        assert!(
            matches!(
                nkd.holiday_on(day(ty, tm, td)).map(Holiday::kind),
                Some(HolidayKind::ReplacementBlocks(_)),
            ),
            "{ty}-{tm:02}-{td:02} must state the queue's own trade date"
        );

        let open = ct(cy, cm, cd, 16, 0, 0);
        let close = ct(cy, cm, cd, 17, 0, 0);
        // One second before the onset is outside the queue. For 2025-01-01 that
        // instant belongs to 2024-12-31, a date the table withholds as
        // `Unsourced`, so the contract's own refusal is the right answer there
        // as well.
        assert!(
            !matches!(
                nkd.is_order_entry_only(open - Duration::seconds(1)),
                Ok(true)
            ),
            "{cy}-{cm:02}-{cd:02}: no queue runs before the published 16:00 CT onset"
        );
        // The queue's opening day is the closure's own civil day; for the
        // 2025-01-01 queue that day-after is the withheld 2024-12-31, so the
        // identity refuses the queue questions instead of answering them.
        let opening_withheld = (cy, cm, cd) == (2025, 1, 1);
        for instant in [open, close - Duration::seconds(1)] {
            if opening_withheld {
                assert!(
                    nkd.session_state(instant).is_err_and(|error| matches!(
                        error,
                        CalendarQueryError::UnresolvedGap { .. }
                    )),
                    "{instant}: the queue's opening day is withheld"
                );
                continue;
            }
            assert_eq!(
                nkd.session_state(instant),
                Ok(SessionState::OrderEntry),
                "{instant} is inside the published Pre-Open"
            );
            assert!(
                nkd.is_order_entry_only(instant)
                    .expect("the coverage contract must answer a covered date")
            );
            assert!(
                nkd.is_accepting_orders(instant)
                    .expect("the coverage contract must answer a covered date")
            );
            assert!(
                !nkd.is_open(instant)
                    .expect("the coverage contract must answer a covered date"),
                "{instant}: a pre-open matches no trade"
            );
            assert_eq!(
                nkd.trade_date(instant)
                    .expect("the coverage contract must answer a covered date"),
                Some(day(ty, tm, td)),
                "{instant} carries the trade date CME prints on it"
            );
        }
        if opening_withheld {
            continue;
        }
        assert_eq!(
            nkd.session_state(close),
            Ok(SessionState::OpenRegular),
            "{cy}-{cm:02}-{cd:02}: 17:00 CT hands the queue to the matching session"
        );
        assert_eq!(
            nkd.session_bounds(ct(cy, cm, cd, 18, 0, 0))
                .expect("the coverage contract must answer a covered date"),
            Some((close, ct(ty, tm, td, 16, 0, 0))),
            "{cy}-{cm:02}-{cd:02}: the queue feeds the 17:00-16:00 CT envelope"
        );
    }
}

// ---------------------------------------------------------------------------
// The 2016-2018 rows.
// ---------------------------------------------------------------------------
/// The era is governed by the Equity Index line, and its nine closures and
/// 24 early closes land on the same trade dates as that line's.
#[test]
fn wave2_rows_follow_the_equity_index_line() {
    let calendar = nkd();
    let equity = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    let mut closures = Vec::new();
    let mut date = day(2016, 1, 1);
    while date <= day(2018, 12, 31) {
        assert_eq!(
            kind_on(calendar, date),
            kind_on(equity, date),
            "{date}: NKD and the Equity Index line agree in this era"
        );
        if kind_on(calendar, date) == Some(HolidayKind::Closed) {
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

/// The era's early closes and its one late open. The row's printed instant is
/// still fenced on both sides, but the fence now holds the refusal those probes
/// earn below the 2025 floor rather than the open state they used to state,
/// and the trade date the late open carried is refused with them.
#[test]
fn wave2_early_close_and_late_open_bounds_are_end_and_start_exclusive() {
    let calendar = nkd();

    let noon = ct(2016, 1, 18, 12, 0, 0);
    assert_eq!(
        kind_on(calendar, day(2016, 1, 18)),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600
        })
    );
    assert!(
        calendar
            .is_open(noon - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(noon)
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2016, 1, 17, 18, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );

    // 2018-12-26: the Christmas sheet's Equity line opens the trade date at
    // 15:30 CT rather than the grid's 15:15.
    let late = ct(2018, 12, 26, 15, 30, 0);
    assert_eq!(
        kind_on(calendar, day(2018, 12, 26)),
        Some(HolidayKind::LateOpen {
            open_ssm: 15 * 3_600 + 30 * 60
        })
    );
    assert!(
        !calendar
            .is_open(late - Duration::seconds(1))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(late)
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(late)
            .expect("the coverage contract must answer a covered date"),
        Some(day(2018, 12, 26))
    );
}

/// The era's own window edges, and the interval below the first window.
///
/// The table claims — which dates the window contains and which ship no row —
/// are unchanged; the first window has opened at the 2010-01-01 floor since
/// the #225 remainder modelled the old grid (2026-09-30 UTC), so the only
/// interval below the windows is pre-floor and refuses as such.
#[test]
fn wave2_window_edges_answer_as_declared() {
    let calendar = nkd();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");

    assert!(coverage.contains(day(2016, 1, 1)));
    assert!(coverage.contains(day(2018, 12, 31)));
    // 2015-12-31 is the 2013-2015 window's last audited date since the
    // 2011-2015 wave shipped.
    assert!(coverage.contains(day(2015, 12, 31)));
    // 2019-01-01 became the first date of a window of its own when the
    // 2019-2021 wave shipped, and the first window has opened at the floor
    // since the #225 remainder shipped.
    assert!(coverage.contains(day(2019, 1, 1)));
    assert!(coverage.contains(day(2010, 4, 10)));
    assert_eq!(calendar.holiday_on(day(2013, 6, 14)), None);
    assert_eq!(
        kind_on(calendar, day(2015, 12, 25)),
        Some(HolidayKind::Closed)
    );
    assert!(
        !calendar
            .is_open(ct(2010, 4, 10, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "2010-04-10 is a covered Saturday: neither old-grid spelling has a session"
    );
    assert!(
        !bare
            .is_open(ct(2010, 4, 10, 10, 0, 0))
            .expect("the detached calendar answers above the 2010 floor from the old grid"),
        "neither old-grid spelling has a Saturday session"
    );
    // The last old-grid weekday and the Monday after the changeover: the
    // detached Friday answers from the CDT old grid through its 17:00-18:00 CT
    // tail, and the detached Monday already answers from the notice's grid —
    // its Sunday-opened wrap runs straight through 02:30 CT, where the old
    // grid's Monday was still closed.
    assert!(
        bare.is_open(ct(2010, 4, 9, 17, 30, 0))
            .expect("the detached calendar answers above the 2010 floor from the old grid"),
        "2010-04-09 17:30 CT is inside the old grid's Friday tail"
    );
    assert!(
        !bare
            .is_open(ct(2010, 4, 9, 18, 0, 0))
            .expect("the detached calendar answers above the 2010 floor from the old grid"),
        "the old grid's Friday tail closes end-exclusively at 18:00 CT"
    );
    assert!(
        bare.is_open(ct(2010, 4, 12, 2, 30, 0))
            .expect("the detached calendar answers above the 2010 floor from the served grid"),
        "2010-04-12 02:30 CT is inside the served grid's Sunday-opened wrap"
    );
}

// ---------------------------------------------------------------------------
// The old-grid era, 2010-01-01..2010-04-10 (#225's remainder).
// ---------------------------------------------------------------------------

/// The old-grid era answers from the modelled grid and ships exactly one row.
///
/// The New Year closure is the era's only sheet-stated closure for NKD (`CME
/// Globex is closed` under every product class of the 2010 New Year sheet).
/// The President's Day sheet excepts the contract onto its regular CST open,
/// and the Martin Luther King and Good Friday sheets name no Nikkei line at
/// all, so those days are audited normal on the modelled grid — fenced here on
/// both sides of every distinct instant the two spellings state.
#[test]
fn the_old_grid_era_answers_from_the_modelled_grid_with_one_row() {
    let calendar = nkd();

    let mut rows: Vec<(NaiveDate, HolidayKind, EvidenceTier)> = Vec::new();
    let mut date = day(2010, 1, 1);
    while date <= day(2010, 4, 10) {
        if let Some(holiday) = calendar.holiday_on(date) {
            rows.push((date, holiday.kind(), holiday.tier()));
        }
        date = date.succ_opt().expect("the era is bounded");
    }
    assert_eq!(rows.len(), 1, "the era ships exactly one row: {rows:?}");
    assert_eq!(rows[0].0, day(2010, 1, 1));
    assert_eq!(
        rows[0].1,
        HolidayKind::Closed,
        "the New Year sheet's venue-wide closure is the era's only row"
    );
    assert_eq!(rows[0].2, EvidenceTier::T1);

    // The closure: the CST grid's Friday sessions do not run, and the next
    // open is the following Monday's regular 02:00 CT.
    for (hour, minute) in [(2, 0), (9, 30), (16, 29)] {
        assert!(
            !calendar
                .is_open(ct(2010, 1, 1, hour, minute, 0))
                .expect("the coverage contract must answer a covered date"),
            "2010-01-01 {hour:02}:{minute:02} CT is inside the venue-wide closure"
        );
    }
    assert_eq!(
        calendar
            .next_session_open_after(ct(2010, 1, 1, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(ct(2010, 1, 4, 2, 0, 0)),
        "the next session is Monday 2010-01-04's regular 02:00 CT open"
    );

    // Martin Luther King 2010-01-18, audited normal: the regular CST grid,
    // whose 02:00 CT open and 16:30 CT close run straight through the holiday.
    assert!(
        calendar
            .is_open(ct(2010, 1, 18, 2, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "02:00 CT opens the holiday Monday on the regular grid"
    );
    assert!(
        calendar
            .is_open(ct(2010, 1, 18, 10, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "10:30 CT trades: the equity sheet's halt is not this family's arrangement"
    );
    assert!(
        !calendar
            .is_open(ct(2010, 1, 18, 16, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "16:30 CT closes the holiday Monday exactly as it closes an ordinary day"
    );

    // President's Day 2010-02-15, the NKD instants the sheet names: the
    // regularly scheduled 02:00 CT open (05:00 is the yen contract's), fenced
    // at the second.
    assert!(
        !calendar
            .is_open(ct(2010, 2, 15, 1, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2010, 2, 15, 2, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "02:00 CT is the excepted NKD open"
    );

    // Good Friday 2010-04-02, audited normal on the CDT grid: the equity
    // line's 08:15 CT early close is not this family's arrangement, and the
    // 17:00-18:00 CT tail still runs.
    assert!(
        calendar
            .is_open(ct(2010, 4, 2, 8, 15, 0))
            .expect("the coverage contract must answer a covered date"),
        "08:15 CT is the equity family's early close, not NKD's: the regular session trades through it"
    );
    assert!(
        !calendar
            .is_open(ct(2010, 4, 2, 15, 20, 0))
            .expect("the coverage contract must answer a covered date"),
        "the CDT grid's own 15:15-15:30 CT break still holds"
    );
    assert!(
        calendar
            .is_open(ct(2010, 4, 2, 17, 30, 0))
            .expect("the coverage contract must answer a covered date"),
        "17:30 CT is inside the CDT grid's evening tail"
    );
    assert!(
        !calendar
            .is_open(ct(2010, 4, 2, 18, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "18:00 CT is the tail's end-exclusive close"
    );
}

// ---------------------------------------------------------------------------
// The 2022-2024 rows.
// ---------------------------------------------------------------------------

/// 12:00 CT, the era's Monday and Thursday holiday close.
const ERA_NOON: u32 = 12 * 3_600;
/// 12:15 CT, the era's Independence Day eve, Thanksgiving Friday and Christmas
/// Eve close.
const ERA_QUARTER_PAST_NOON: u32 = 12 * 3_600 + 15 * 60;

/// Every row the 2022-2024 window ships, in table order: the venue-local trade
/// date, the kind with the instant the module's own `early_close(..)` payload
/// carries, and the tier beside the row.
///
/// This is the era-wide instant fence: the sweep below walks the whole window
/// and compares against this list row for row, so a dropped, added or moved
/// row fails as loudly as a wrong instant. A sample would let a slipped close
/// move unnoticed on the dates nobody probed. The seventeen `Unsourced` rows
/// are the honest half of the wave: the T2 service answers for ten
/// representative products and no Nikkei is among them.
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
    ((2022, 5, 30), HolidayKind::Unsourced, EvidenceTier::T1),
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
        (2023, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
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
    ((2024, 1, 15), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 2, 19), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 3, 29), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 5, 27), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 6, 19), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 7, 3), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 7, 4), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 9, 2), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 11, 28), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 11, 29), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 12, 24), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 12, 25), HolidayKind::Unsourced, EvidenceTier::T2),
    ((2024, 12, 31), HolidayKind::Unsourced, EvidenceTier::T2),
];

/// `ct` for a date the walk computed rather than spelled, so day arithmetic
/// cannot drift out of step with a hand-written tuple.
fn ct_on(date: NaiveDate, time: (u32, u32, u32)) -> DateTime<Utc> {
    ct(
        date.year(),
        date.month(),
        date.day(),
        time.0,
        time.1,
        time.2,
    )
}

fn day_before(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(1))
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

fn day_after(date: NaiveDate) -> NaiveDate {
    date.checked_add_days(Days::new(1))
        .expect("the era is far from the representable bound")
}

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary and the trading day's stated end derived from
/// the row itself.
///
/// The era precedes the 2025 floor, so each derived instant is fenced by the
/// refusal it earns — `BeforeSupportFloor` — instead of by the open state,
/// bounds or candle the crate used to state for it.
#[test]
fn era_2022_2024_sweeps_every_row_kind_tier_and_instant() {
    let calendar = nkd();
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
        (noons, quarter_past_noon, closures, unsourced),
        (11, 3, 5, 17),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open. Every era this table ships a closure in precedes the 2025
/// floor, so the deleted day, its deleted evening leg, the absent trade date
/// and the reopening session are all refused with `BeforeSupportFloor`: what
/// the fence pins now is that each query states the refusal rather than an
/// answer, while the row's own kind, instant and tier stand.
#[test]
fn era_2022_2024_closures_remove_the_trading_day_and_the_prior_evening_wrap() {
    let calendar = nkd();
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
    assert_eq!(closures, 5, "the era's closures");
}

/// Asserts what an `Unsourced` row still states and what it can no longer
/// state.
///
/// The row itself is unchanged: the date ships as `Unsourced`, at its recorded
/// tier, so the table expressly withholds a scheduling claim. The observable
/// neutrality this helper used to fence — "every query answers exactly as the
/// detached calendar does", with the successor-closure exception it documented —
/// is gone, because every era that ships an `Unsourced` row precedes the 2025
/// floor and every date-aware query about those dates is refused, on the
/// identity and on the detached snapshot alike.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = nkd();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    // The withheld row contributes no clip, so the detached normal week keeps
    // the ordinary answer; the attached identity refuses the date and every
    // derivation that reads it as `UnresolvedGap` — never as a claimed
    // closure. 2021-06-19 is a Saturday, which the grid has no session on
    // whether or not a row exists, so that query answers there without
    // reading the row; and the refusal may name the withheld neighbour the
    // derivation reads rather than the queried date itself.
    if matches!(date.weekday(), Weekday::Sat | Weekday::Sun) {
        assert_eq!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("the coverage contract must answer a covered date"),
            detached
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("the coverage contract must answer a covered date"),
            "{date}"
        );
    } else {
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
            "{date}: a withheld date is never reported closed"
        );
    }
    for probe in [
        ct_on(day_before(date), (18, 0, 0)),
        ct_on(date, (9, 0, 0)),
        ct_on(date, (15, 59, 0)),
        ct_on(date, (18, 0, 0)),
    ] {
        // A probe whose derivation reads the withheld date — the on-date
        // probes name it directly and a held wrap is dated by it — is
        // refused; a probe off the date that holds no session anywhere
        // answers exactly as the detached grid does.
        let detached_open = detached
            .is_open(probe)
            .expect("the coverage contract must answer a covered date");
        let on_the_date = probe.with_timezone(&US::Central).date_naive() == date;
        if on_the_date || detached_open {
            assert!(
                calendar
                    .is_open(probe)
                    .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
                "{probe}: a withheld date is refused, never claimed closed"
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
    }
}

/// All seventeen `Unsourced` dates, not a sample.
///
/// The dates are all inside the 2022-2024 era, so the identity answers none of
/// them: each one earns the refusal its path derives, and the detached calendar
/// refuses them too because they precede the floor.
#[test]
fn era_2022_2024_unsourced_rows_change_no_answer() {
    let calendar = nkd();
    let mut checked = 0_usize;
    for (date, kind, tier) in ERA_ROWS {
        if *kind == HolidayKind::Unsourced {
            let date = day(date.0, date.1, date.2);
            let row = calendar
                .holiday_on(date)
                .unwrap_or_else(|| panic!("{date} ships a row"));
            assert_unsourced_changes_nothing(date, row, *tier);
            checked += 1;
        }
    }
    assert_eq!(checked, 17, "every unsourced row is fenced");
}

/// The 2022-05-30 merged `Nikkei & BTIC` line and one 2024 T2 window: both
/// ship `Unsourced` with the document that failed to state the outright
/// Nikkei's own instant, and neither moves an answer.
///
/// The rows' kind, tier and document id are fenced as before; the answers they
/// used not to move are now refusals, because both dates precede the 2025
/// floor and the identity refuses them on the identity calendar and on a
/// detached snapshot alike.
#[test]
fn era_2022_2024_unsourced_rows_cite_the_document_that_withheld_the_instant() {
    let calendar = nkd();

    let merged = calendar
        .holiday_on(day(2022, 5, 30))
        .expect("2022-05-30 ships a row");
    assert_eq!(merged.kind(), HolidayKind::Unsourced);
    assert_eq!(merged.tier(), EvidenceTier::T1);
    assert_eq!(
        merged.document_id(),
        "2022-memorial-day-holiday-schedule.xls @2022-07-04T06:54:38Z"
    );
    // The sheet prints the merged line's BTIC-style 01:00 CT Friday close; the
    // outright Nikkei's own close is not stated, so no early close ships. The
    // date itself is withheld: the attached identity refuses it as
    // `UnresolvedGap` while the detached normal week answers, the row having
    // clipped nothing there.
    assert!(
        calendar
            .is_open(ct(2022, 5, 30, 10, 0, 0))
            .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
        "2022-05-30 is withheld as Unsourced"
    );
    assert!(
        calendar
            .without_holidays()
            .is_open(ct(2022, 5, 30, 10, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );

    let service = calendar
        .holiday_on(day(2024, 7, 3))
        .expect("2024-07-03 ships a row");
    assert_eq!(service.kind(), HolidayKind::Unsourced);
    assert_eq!(service.tier(), EvidenceTier::T2);
    assert_eq!(service.document_id(), "CME-SVC-2024-07-03");
    // The withheld date is refused, never answered off either calendar.
    assert!(
        calendar
            .is_open(ct(2024, 7, 3, 15, 59, 0))
            .is_err_and(|error| matches!(error, CalendarQueryError::UnresolvedGap { .. })),
        "2024-07-03 is withheld as Unsourced"
    );
    assert!(
        calendar
            .without_holidays()
            .is_open(ct(2024, 7, 3, 15, 59, 0))
            .is_ok(),
        "the detached grid answers the ordinary week"
    );
}

/// The 2022-2024 window sits third from the end of the declared coverage, its
/// edges answer, and the floor era below the changeover answers since the
/// #225 remainder modelled the old grid and moved the first window to the
/// 2010-01-01 floor (2026-09-30 UTC) — only a pre-floor date refuses.
/// (The 2013-2015 and 2011-01-12..2011-01-16 intervals earlier tests fenced
/// became audited as later waves shipped; the 2011-2015 section fences those.)
#[test]
fn era_2022_2024_window_sits_third_and_the_floor_era_answers() {
    let calendar = nkd();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");

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
    assert_eq!(
        kind_on(calendar, day(2024, 12, 31)),
        Some(HolidayKind::Unsourced)
    );

    // 2013-06-14 and Christmas 2015 are audited dates of the 2013-2015 window
    // since the 2011-2015 wave shipped: the first ships no row and the second
    // ships the closure.
    assert_eq!(calendar.holiday_on(day(2013, 6, 14)), None);
    assert_eq!(
        kind_on(calendar, day(2015, 12, 25)),
        Some(HolidayKind::Closed)
    );
    // The floor era answers since the #225 remainder shipped: the two probes
    // the earlier fence read as refusals are covered dates the modelled old
    // grid answers, and only a pre-floor date refuses — on the identity and on
    // the detached snapshot alike.
    assert!(
        !calendar
            .is_open(ct(2010, 4, 10, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "2010-04-10 is a Saturday: neither old-grid spelling has a session"
    );
    assert!(
        !calendar
            .is_open(ct(2010, 1, 5, 18, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "2010-01-05 18:00 CT is past the CST grid's 16:30 CT close"
    );
    assert!(
        calendar
            .is_open(ct(2010, 1, 5, 14, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "2010-01-05 14:00 CT is inside the CST grid's afternoon run"
    );
    let probe = ct(2009, 12, 31, 18, 0, 0);
    assert!(
        calendar
            .is_open(probe)
            .is_err_and(|error| matches!(error, CalendarQueryError::BeforeSupportFloor { .. })),
        "{probe} is below the 2010 floor"
    );
    assert!(
        bare.is_open(probe)
            .is_err_and(|error| matches!(error, CalendarQueryError::BeforeSupportFloor { .. })),
        "{probe}: the detached snapshot refuses the floor too"
    );
}

// ---------------------------------------------------------------------------
// The 2019-2021 rows.
// ---------------------------------------------------------------------------

/// 08:15 CT, the one Good Friday 2021 close this era prints.
const ERA_EIGHT_FIFTEEN: u32 = 8 * 3_600 + 15 * 60;

/// The era-wide sweep: every row the 2019-2021 window ships, read from the
/// module rather than copied beside it, with both sides of every instant it
/// states.
///
/// The walk covers the whole window, so a dropped or added row fails on the
/// era's total, a moved instant fails on the count for the instant it moved
/// from or to, and a kind this family does not ship fails outright. Both sides
/// of an early close's instant, and the trade date and opening leg of a
/// closure, are still derived from the row and fenced — but the era precedes
/// the 2025 floor, so what they assert is the refusal, not the open state or
/// the trade date the crate used to return.
#[test]
fn era_2019_2021_sweeps_every_shipped_row_kind_and_instant() {
    let calendar = nkd();
    let (mut noons, mut quarters, mut eight_fifteens) = (0_usize, 0, 0);
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
                        ERA_EIGHT_FIFTEEN => eight_fifteens += 1,
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
                    // session, so the 08:15 Good Friday close cannot make the
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
    assert_eq!(rows, 36, "the era's rows");
    assert_eq!(
        (noons, quarters, eight_fifteens, closures, unsourced),
        (18, 6, 1, 8, 3),
        "the era's shape"
    );
}

/// Every `Unsourced` row the era ships: the row states that the date was
/// audited, makes no scheduling claim, and clips nothing.
///
/// Its dates precede the 2025 floor, so the queries that used to demonstrate
/// that neutrality now refuse them instead; `assert_unsourced_changes_nothing`
/// states what survives and why.
#[test]
fn era_2019_2021_unsourced_rows_change_no_answer() {
    let calendar = nkd();
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
///
/// The window and its rows are unchanged. Its last day is no longer answered,
/// though: 2021-12-31 precedes the 2025 floor, so the date-aware question
/// takes the explicit refusal rather than a normal-week answer.
#[test]
fn era_2019_2021_window_edges_answer_as_the_module_declares() {
    let calendar = nkd();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_nikkei_225_dollar ships a table");
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
            .is_open(ct(2021, 12, 31, 9, 0, 0))
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
    let calendar = nkd();
    let coverage = calendar
        .holiday_coverage()
        .expect("globex_nikkei_225_dollar ships a table");

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
    assert_eq!(rows, 36, "the era's rows");
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
        (2019, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: 12 * 3_600 + 15 * 60,
        },
        EvidenceTier::T1,
    ),
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
            close_ssm: 8 * 3_600 + 15 * 60,
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
    let calendar = nkd();
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

/// A Saturday-session row's Sunday legs must be sourced from a window that
/// actually prints them.
///
/// Two of these rows span **two** windows: the window that carries the Saturday
/// session stops at that Saturday and prints no Sunday entry at all, so the
/// Sunday Pre-Open and the Sunday-Monday session come from the window that
/// starts on the Sunday. The third row's Saturday window happens to run through
/// its Sunday and does print the legs, so it needs only the one.
///
/// This fence exists because an independent review found the two-window rows
/// claiming a Sunday pair that their cited artifact does not contain. It pins
/// the distinction that the per-row citation check cannot see.
#[test]
fn a_saturday_rows_sunday_legs_are_sourced_from_a_window_that_prints_them() {
    // (trade date, Saturday window, the window that actually prints the Sunday legs)
    for (trade_date, saturday_window, sunday_window) in [
        ("2026-06-22", "CME-SVC-B-2026-06-18", "CME-SVC-B-2026-06-21"),
        ("2026-07-06", "CME-SVC-B-2026-07-03", "CME-SVC-B-2026-07-03"),
        ("2027-06-21", "CME-SVC-B-2027-06-17", "CME-SVC-B-2027-06-20"),
    ] {
        let evidence = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("docs/evidence/globex_nikkei_225_dollar.md"),
        )
        .expect("the evidence file must be readable");
        for window in [saturday_window, sunday_window] {
            assert!(
                evidence.contains(&format!("| `{window}` |")),
                "{trade_date}: {window} must be a recorded document"
            );
        }
        let row = evidence
            .lines()
            .find(|line| line.starts_with(&format!("| {trade_date} |")))
            .unwrap_or_else(|| panic!("{trade_date} must have an evidence row"));
        assert!(
            row.contains(saturday_window),
            "{trade_date}: the row must cite the Saturday session's window"
        );
        // Where the two differ, the row must name the second window too: the
        // legs the first does not print are not in it.
        if sunday_window != saturday_window {
            assert!(
                row.contains(sunday_window),
                "{trade_date}: the row must name {sunday_window}, which is the window \
                 that prints the Sunday legs"
            );
        }
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
    let calendar = nkd();
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
            .is_open(ct(2025, 11, 28, 7, 15, 0))
            .expect("the coverage contract must answer a covered date"),
        "07:15 CT is the operator's Pre-Open and must not report matching"
    );
    assert!(
        calendar
            .is_open(ct(2025, 11, 28, 7, 45, 0))
            .expect("the coverage contract must answer a covered date"),
        "07:45 CT is inside continuous trading and must report matching"
    );
    // The trade date does not move: the day still carries 2025-11-28.
    assert_eq!(
        calendar
            .trade_date(ct(2025, 11, 28, 6, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );
    assert_eq!(
        calendar
            .trade_date(ct(2025, 11, 28, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2025, 11, 28))
    );

    // The 2026 and 2027 Thanksgiving Fridays publish the final close alone, so
    // the same morning must still report matching there.
    for (year, month, date) in [(2026, 11, 27), (2027, 11, 26)] {
        assert!(
            calendar
                .is_open(ct(year, month, date, 7, 15, 0))
                .expect("the coverage contract must answer a covered date"),
            "{year}-{month:02}-{date:02} publishes no Pre-Open and must stay open at 07:15 CT"
        );
    }
}

// ---------------------------------------------------------------------------
// The five 2025 merged trade dates this family cannot witness (issue #162).
// ---------------------------------------------------------------------------

/// `globex_nikkei_225_dollar` models seventeen merged trade dates — the
/// day-after-holiday dates on which CME assigns the Sunday- or
/// Wednesday-evening-through-holiday span to the *following* business day — and
/// ships twelve. The five 2025 dates (2025-01-21, 2025-02-18, 2025-05-27,
/// 2025-06-20, 2025-09-02) ship no row, and this fence states that as the
/// **witness gap** it is rather than as an accident.
///
/// The twelve that ship each have their own `NKD`(168)/`NIY`(167) witness bytes.
/// The five that do not have no Nikkei event anywhere in the research store:
/// five targeted `THBP-B` captures requested exactly `id=168,167` over exactly
/// these windows and returned an empty event list for both products
/// (`raw/cme-2025-2027-repair/json/edgeB_2025-*.json`). Inventing the rows from
/// the sibling families' line would be a fabricated date, so the gap is recorded
/// in the evidence file with its closing condition and tracked as issue #162.
///
/// The behavioural consequence is what this fence pins: at 18:00 CT on the
/// evening before each of the five, the four sibling families answer the
/// *following* business day while this family still answers the holiday itself.
#[test]
fn the_five_unwitnessed_2025_merged_dates_answer_the_holiday_not_the_next_business_day() {
    // (evening before, the holiday this family answers, the merged date CME assigns)
    type Case = (i32, u32, u32, (i32, u32, u32), (i32, u32, u32));
    let nikkei = nkd();
    let unwitnessed: [Case; 5] = [
        (2025, 1, 19, (2025, 1, 20), (2025, 1, 21)),
        (2025, 2, 16, (2025, 2, 17), (2025, 2, 18)),
        (2025, 5, 25, (2025, 5, 26), (2025, 5, 27)),
        (2025, 6, 18, (2025, 6, 19), (2025, 6, 20)),
        (2025, 8, 31, (2025, 9, 1), (2025, 9, 2)),
    ];
    for (y, m, d, holiday, merged) in unwitnessed {
        let instant = ct(y, m, d, 18, 0, 0);
        // The family still keys the evening leg to the holiday, because it ships
        // no merged row for the following business day.
        assert_eq!(
            nikkei
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(day(holiday.0, holiday.1, holiday.2)),
            "{y}-{m:02}-{d:02} 18:00 CT must still carry the holiday for this family"
        );
        // And it ships no row at all for the merged date.
        assert_eq!(
            nikkei
                .holiday_on(day(merged.0, merged.1, merged.2))
                .map(exchange_hours::Holiday::kind),
            None,
            "{}-{:02}-{:02} must ship no row: the gap is a witness gap, not a modelled date",
            merged.0,
            merged.1,
            merged.2
        );
        // A sibling family that *does* witness its merged dates answers the next
        // business day at the same instant. `globex_fx` is that family.
        assert_eq!(
            exchange_hours::calendar_for_market_hours_key(MarketHoursKey::GlobexFx)
                .trade_date(instant)
                .expect("the coverage contract must answer a covered date"),
            Some(day(merged.0, merged.1, merged.2)),
            "{y}-{m:02}-{d:02} 18:00 CT must carry the merged date for globex_fx"
        );
    }
}

// ---------------------------------------------------------------------------
// The 2011-2015 rows (the 2026-09-29 wave that closed the 2011-2015 span).
// ---------------------------------------------------------------------------

/// 10:30 CT, the halt-era Monday and Thursday holiday close through
/// 2014-02.
const ERA_TEN_THIRTY: u32 = 10 * 3_600 + 30 * 60;
/// 05:00 CT, the six post-closure late opens.
const ERA_FIVE_AM: u32 = 5 * 3_600;

/// Every row the 2011-2015 windows ship, in table order: the venue-local trade
/// date, the kind with the instant the module's own payload carries, and the
/// tier beside the row.
///
/// The list is read back against the sheets by the era sweep below, which
/// walks the whole span so a dropped, added or moved row fails as loudly as a
/// wrong instant. All 62 rows are T1, CME's own published per-holiday
/// schedules, read through the `Equity Products` line no sheet of the era
/// contradicts with a Nikkei line of its own.
const ERA_2011_2015_ROWS: &[((i32, u32, u32), HolidayKind, EvidenceTier)] = &[
    (
        (2011, 1, 17),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2011, 2, 21),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    ((2011, 4, 22), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2011, 5, 30),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2011, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2011, 9, 5),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2011, 11, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2011, 11, 25),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2011, 12, 26), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2011, 12, 27),
        HolidayKind::LateOpen {
            open_ssm: ERA_FIVE_AM,
        },
        EvidenceTier::T1,
    ),
    ((2012, 1, 2), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2012, 1, 3),
        HolidayKind::LateOpen {
            open_ssm: ERA_FIVE_AM,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 1, 16),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 2, 20),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 4, 6),
        HolidayKind::EarlyClose {
            close_ssm: ERA_EIGHT_FIFTEEN,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 5, 28),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 9, 3),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 11, 22),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 11, 23),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2012, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2012, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2012, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: ERA_FIVE_AM,
        },
        EvidenceTier::T1,
    ),
    ((2013, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: ERA_FIVE_AM,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 1, 21),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 2, 18),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    ((2013, 3, 29), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 5, 27),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 9, 2),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 11, 29),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2013, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2013, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2013, 12, 26),
        HolidayKind::LateOpen {
            open_ssm: ERA_FIVE_AM,
        },
        EvidenceTier::T1,
    ),
    ((2014, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 1, 2),
        HolidayKind::LateOpen {
            open_ssm: ERA_FIVE_AM,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 1, 20),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 2, 17),
        HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY,
        },
        EvidenceTier::T1,
    ),
    ((2014, 4, 18), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2014, 5, 26),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 7, 4),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 9, 1),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 11, 28),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2014, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2014, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
    ((2015, 1, 1), HolidayKind::Closed, EvidenceTier::T1),
    (
        (2015, 1, 19),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 2, 16),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 4, 3),
        HolidayKind::EarlyClose {
            close_ssm: ERA_EIGHT_FIFTEEN,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 5, 25),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 7, 3),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 9, 7),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 11, 26),
        HolidayKind::EarlyClose {
            close_ssm: ERA_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 11, 27),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    (
        (2015, 12, 24),
        HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON,
        },
        EvidenceTier::T1,
    ),
    ((2015, 12, 25), HolidayKind::Closed, EvidenceTier::T1),
];

/// The era-wide sweep: every shipped date's kind, instant and tier, with both
/// sides of every moved boundary derived from the row itself.
///
/// Unlike the later eras, these dates are served: the profile's first sourced
/// day is 2011-01-12, the floor is 2010-01-01, and both new windows answer, so
/// the probes state open/closed truth rather than refusals.
#[test]
fn era_2011_2015_sweeps_every_row_kind_tier_and_instant() {
    let calendar = nkd();
    let mut index = 0_usize;
    let (
        mut ten_thirties,
        mut noons,
        mut quarter_past_noon,
        mut eight_fifteens,
        mut closures,
        mut late_opens,
    ) = (0_usize, 0_usize, 0_usize, 0_usize, 0_usize, 0_usize);
    let mut date = day(2011, 1, 12);
    while date <= day(2015, 12, 31) {
        if let Some(row) = calendar.holiday_on(date) {
            let (expected, kind, tier) = ERA_2011_2015_ROWS[index];
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
                        ERA_TEN_THIRTY => ten_thirties += 1,
                        ERA_NOON => noons += 1,
                        ERA_QUARTER_PAST_NOON => quarter_past_noon += 1,
                        ERA_EIGHT_FIFTEEN => eight_fifteens += 1,
                        other => panic!("{date}: the era ships no {other}-second CT close"),
                    }
                    let cutoff = ct_on(
                        date,
                        (close_ssm / 3_600, (close_ssm % 3_600) / 60, close_ssm % 60),
                    );
                    // The wrap that opened this trade date is clipped, not
                    // deleted, and it still carries the trade date.
                    assert!(
                        calendar
                            .is_open(ct_on(day_before(date), (18, 0, 0)))
                            .expect("the coverage contract must answer a covered date"),
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
                }
                HolidayKind::Closed => closures += 1,
                HolidayKind::LateOpen { open_ssm } => {
                    late_opens += 1;
                    assert_eq!(open_ssm, ERA_FIVE_AM, "{date}");
                }
                other => panic!("{date}: the era ships no {other:?}"),
            }
            index += 1;
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(index, ERA_2011_2015_ROWS.len(), "every planned row ships");
    assert_eq!(
        (
            ten_thirties,
            noons,
            quarter_past_noon,
            eight_fifteens,
            closures,
            late_opens
        ),
        (20, 10, 12, 2, 12, 6),
        "the era's shape: 44 early closes, 12 closures and 6 late opens"
    );
}

/// The 2010 window (opened 2026-09-30 UTC at the notice-dated 2010-04-11 grid
/// start) ships exactly the six rows the era's sheets read to, and agrees with
/// the `Equity Products` line of `globex_equity_index` row for row across the
/// whole span both audit. Both sides of the dated grid start are probed: the
/// profile answers the served grid from 2010-04-11 and refuses the old-grid
/// era below it.
#[test]
fn era_2010_window_ships_the_six_sheets_rows_and_follows_the_equity_index_line() {
    let calendar = nkd();
    let equity = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    let mut date = day(2010, 4, 11);
    while date <= day(2011, 1, 11) {
        assert_eq!(
            kind_on(calendar, date),
            kind_on(equity, date),
            "{date}: NKD and the Equity Index line agree in the dated-grid era"
        );
        date = date.succ_opt().expect("the era ends well before the bound");
    }

    let expected: [(NaiveDate, HolidayKind); 6] = [
        (
            day(2010, 5, 31),
            HolidayKind::EarlyClose {
                close_ssm: ERA_TEN_THIRTY,
            },
        ),
        (
            day(2010, 7, 5),
            HolidayKind::EarlyClose {
                close_ssm: ERA_TEN_THIRTY,
            },
        ),
        (
            day(2010, 9, 6),
            HolidayKind::EarlyClose {
                close_ssm: ERA_TEN_THIRTY,
            },
        ),
        (
            day(2010, 11, 25),
            HolidayKind::EarlyClose {
                close_ssm: ERA_TEN_THIRTY,
            },
        ),
        (
            day(2010, 11, 26),
            HolidayKind::EarlyClose {
                close_ssm: ERA_QUARTER_PAST_NOON,
            },
        ),
        (day(2010, 12, 24), HolidayKind::Closed),
    ];
    let mut shipped = Vec::new();
    let mut date = day(2010, 4, 11);
    while date <= day(2011, 1, 11) {
        if let Some(row) = calendar.holiday_on(date) {
            shipped.push((date, row.kind()));
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        shipped, expected,
        "the 2010 window ships the six sheet rows"
    );

    // End-exclusive probes on one row of each kind, and the dated grid start:
    // the Columbus Day and Veterans Day dates answer audited normal beside the
    // rows.
    assert_eq!(kind_on(calendar, day(2010, 10, 11)), None);
    assert_eq!(kind_on(calendar, day(2010, 11, 11)), None);
    let cutoff = ct_on(day(2010, 5, 31), (10, 30, 0));
    assert!(
        calendar
            .is_open(cutoff - Duration::seconds(1))
            .expect("covered"),
        "the Memorial Day halt's last second trades"
    );
    assert!(
        !calendar.is_open(cutoff).expect("covered"),
        "10:30 CT is the end-exclusive holiday halt"
    );
    let close = ct_on(day(2010, 11, 26), (12, 15, 0));
    assert!(
        calendar
            .is_open(close - Duration::seconds(1))
            .expect("covered"),
        "the Thanksgiving Friday morning trades"
    );
    assert!(
        !calendar.is_open(close).expect("covered"),
        "12:15 CT is the end-exclusive early close"
    );
    assert!(
        !calendar
            .is_open(ct(2010, 12, 24, 18, 0, 0))
            .expect("covered"),
        "2010-12-24 is the shipped closure"
    );
}

/// The era agrees with the `Equity Products` line row for row: the two
/// families read the same sheets and neither ships a row the other lacks
/// across the whole span both audit.
#[test]
fn era_2011_2015_rows_follow_the_equity_index_line() {
    let calendar = nkd();
    let equity = calendar_for_market_hours_key(MarketHoursKey::GlobexEquityIndex);

    let mut closures = Vec::new();
    let mut date = day(2011, 1, 12);
    while date <= day(2015, 12, 31) {
        assert_eq!(
            kind_on(calendar, date),
            kind_on(equity, date),
            "{date}: NKD and the Equity Index line agree in this era"
        );
        if kind_on(calendar, date) == Some(HolidayKind::Closed) {
            closures.push(date);
        }
        date = date.succ_opt().expect("the era ends well before the bound");
    }
    assert_eq!(
        closures,
        [
            day(2011, 4, 22),
            day(2011, 12, 26),
            day(2012, 1, 2),
            day(2012, 12, 25),
            day(2013, 1, 1),
            day(2013, 3, 29),
            day(2013, 12, 25),
            day(2014, 1, 1),
            day(2014, 4, 18),
            day(2014, 12, 25),
            day(2015, 1, 1),
            day(2015, 12, 25),
        ],
        "the era's twelve closures"
    );
}

/// The 10:30 halt shape on the served halt-era grid: the wrapped leg that
/// opened Sunday 17:00 CT ends at the Monday halt, the post-halt segment the
/// same grid carries on an ordinary day is deleted with it, and the 17:00 CT
/// resume hands over to the next trade date.
#[test]
fn era_martin_luther_king_2011_ends_the_wrapped_leg_at_the_1030_halt() {
    let calendar = nkd();

    assert_eq!(
        kind_on(calendar, day(2011, 1, 17)),
        Some(HolidayKind::EarlyClose {
            close_ssm: ERA_TEN_THIRTY
        })
    );
    // The leg opened Sunday 2011-01-16 17:00 CT and runs into the holiday.
    assert!(
        calendar
            .is_open(ct(2011, 1, 16, 17, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2011, 1, 17, 9, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    // The halt is end-exclusive, and the ordinary 15:15 close is no longer
    // reachable: the sheet's `1700 CT – Halted products resume trading` is the
    // next trade date's open, so the post-halt segment is gone too.
    assert!(
        !calendar
            .is_open(ct(2011, 1, 17, 10, 30, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2011, 1, 17, 15, 14, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2011, 1, 17, 15, 45, 0))
            .expect("the coverage contract must answer a covered date"),
        "the halt-era post-halt segment is deleted with the clipped leg"
    );
    assert_eq!(
        calendar
            .session_bounds(ct(2011, 1, 17, 9, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some((ct(2011, 1, 16, 17, 0, 0), ct(2011, 1, 17, 10, 30, 0))),
        "the session that opened Sunday 17:00 CT ends at the printed halt"
    );
    assert_eq!(
        calendar
            .trade_date(ct(2011, 1, 17, 9, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2011, 1, 17)),
        "the shortened day keeps its own trade date"
    );
    // 17:00 CT hands over to the next trade date.
    assert!(
        calendar
            .is_open(ct(2011, 1, 17, 17, 30, 0))
            .expect("the coverage contract must answer a covered date")
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and the next session is the ordinary evening open that follows.
#[test]
fn era_good_friday_2011_deletes_the_trade_date_and_the_prior_evening_leg() {
    let calendar = nkd();

    assert_eq!(
        kind_on(calendar, day(2011, 4, 22)),
        Some(HolidayKind::Closed)
    );
    assert!(
        calendar
            .is_closed_trade_date(day(2011, 4, 22), SessionKind::Both)
            .expect("the coverage contract must answer a covered date")
    );
    // The Thursday-evening leg that would have carried the closed Friday.
    assert!(
        !calendar
            .is_open(ct(2011, 4, 21, 18, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2011, 4, 22, 10, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .trade_date(ct(2011, 4, 22, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        None,
        "the closure ships no trade date"
    );
    // The next session is Sunday evening's, for trade date Monday 2011-04-25.
    assert_eq!(
        calendar
            .next_session_open_after(ct(2011, 4, 22, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(ct(2011, 4, 24, 17, 0, 0))
    );
}

/// A Monday closure deletes the Sunday-evening leg, and because the following
/// Tuesday ships a late open, the next matching session is that trade date's
/// own 05:00 CT open.
#[test]
fn era_new_year_2012_closure_hands_over_to_the_late_open() {
    let calendar = nkd();

    assert_eq!(
        kind_on(calendar, day(2012, 1, 2)),
        Some(HolidayKind::Closed)
    );
    assert!(
        !calendar
            .is_open(ct(2012, 1, 1, 18, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "the Sunday-evening leg that would have carried the closed Monday is deleted"
    );
    assert_eq!(
        kind_on(calendar, day(2012, 1, 3)),
        Some(HolidayKind::LateOpen {
            open_ssm: ERA_FIVE_AM
        })
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct(2012, 1, 2, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(ct(2012, 1, 3, 5, 0, 0)),
        "the sheet prints `0500 CT – CME Globex open for trade date Tuesday, Jan 3`"
    );
    // The late open is start-inclusive and start-exclusive on both sides.
    assert!(
        !calendar
            .is_open(ct(2012, 1, 3, 4, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2012, 1, 3, 5, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .session_bounds(ct(2012, 1, 3, 9, 0, 0))
            .expect("the coverage contract must answer a covered date")
            .map(|(open, _)| open),
        Some(ct(2012, 1, 3, 5, 0, 0)),
        "the trade date's session starts at its own 05:00 CT open"
    );
    assert_eq!(
        calendar
            .trade_date(ct(2012, 1, 3, 9, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2012, 1, 3))
    );
}

/// The 2015 window's edges: the ordinary close is the notice-20150817 16:00 CT
/// from 2015-11-26 on, and Christmas 2015 deletes the trade date with its
/// prior-evening leg while the Thursday daytime session runs to its normal
/// close.
#[test]
fn era_christmas_2015_deletes_only_the_legs_the_operator_deleted() {
    let calendar = nkd();

    assert_eq!(
        kind_on(calendar, day(2015, 12, 24)),
        Some(HolidayKind::EarlyClose {
            close_ssm: ERA_QUARTER_PAST_NOON
        })
    );
    assert_eq!(
        kind_on(calendar, day(2015, 12, 25)),
        Some(HolidayKind::Closed)
    );
    // The 12:15 CT Christmas Eve clip on the 16:00 CT grid.
    assert!(
        calendar
            .is_open(ct(2015, 12, 24, 12, 14, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2015, 12, 24, 12, 15, 0))
            .expect("the coverage contract must answer a covered date")
    );
    // The closed Friday and its deleted Thursday-evening leg.
    assert!(
        !calendar
            .is_open(ct(2015, 12, 24, 17, 30, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        !calendar
            .is_open(ct(2015, 12, 25, 10, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    // The closed Friday is a Friday: the grid has no Friday-evening open (one
    // would belong to a Saturday trade date), so Christmas Day's evening is
    // shut and the next session is Sunday 2015-12-27's 17:00 CT open for trade
    // date Monday 2015-12-28, whose regular close is the 16:00 CT the notice
    // moved to.
    assert!(
        !calendar
            .is_open(ct(2015, 12, 25, 18, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2015, 12, 27, 17, 0, 0))
            .expect("the coverage contract must answer a covered date")
    );
    assert_eq!(
        calendar
            .next_session_open_after(ct(2015, 12, 25, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(ct(2015, 12, 27, 17, 0, 0))
    );
    assert_eq!(
        calendar
            .session_bounds(ct(2015, 12, 28, 10, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some((ct(2015, 12, 27, 17, 0, 0), ct(2015, 12, 28, 16, 0, 0))),
        "the Monday session runs to the notice-20150817 16:00 CT close"
    );
}

/// The 2015-01-02 shape: the New Year's sheet prints no reopening line for the
/// Friday, so no late open ships and the holiday Thursday's own 17:00 CT open
/// carries trade date 2015-01-02 at the ordinary grid.
#[test]
fn era_new_year_2015_ships_no_late_open_and_reopens_at_1700() {
    let calendar = nkd();

    assert_eq!(
        kind_on(calendar, day(2015, 1, 1)),
        Some(HolidayKind::Closed)
    );
    assert_eq!(calendar.holiday_on(day(2015, 1, 2)), None);
    assert!(
        !calendar
            .is_open(ct(2015, 1, 1, 16, 59, 59))
            .expect("the coverage contract must answer a covered date")
    );
    assert!(
        calendar
            .is_open(ct(2015, 1, 1, 17, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        "the holiday Thursday reopens at the ordinary 17:00 CT for Friday's trade date"
    );
    assert_eq!(
        calendar
            .trade_date(ct(2015, 1, 1, 18, 0, 0))
            .expect("the coverage contract must answer a covered date"),
        Some(day(2015, 1, 2))
    );
}


// ---------------------------------------------------------------------------
// The block rows' envelope is the family's regular session (#176)
// ---------------------------------------------------------------------------

/// Every trade date a block row covers answers `is_open_regular` inside its
/// session, exactly as an ordinary date does.
///
/// This family's whole tradeable envelope is its `regular` session
/// (`NKD_REGULAR_CURRENT`; `NKD_EXTENDED_CURRENT` is empty), so a block row
/// spelling the matching runs `extended` flipped `is_open_regular` and
/// `is_open_extended` on every date it covered while `is_open` stayed right.
/// The four pre-existing sets restate their 17:00-16:00 CT runs `regular`
/// since #176, and the fence walks one trade date per set — a merged Monday,
/// a published Saturday, the 2025-11-28 two-piece day and a day-after-
/// Thanksgiving Friday — asserting the ordinary answer at an instant inside
/// each session.
#[test]
fn every_block_row_spells_the_envelope_the_regular_session_it_is() {
    let nkd = nkd();
    for (year, month, date) in [
        (2026, 1, 20), // MERGED_SESSION_BLOCKS (merged MLK Tuesday)
        (2026, 6, 22), // SATURDAY_SESSION_BLOCKS (published Saturday session)
        (2025, 11, 28), // MERGED_SESSION_EARLY_CLOSE_BLOCKS_2025_11_28
        (2026, 11, 27), // MERGED_SESSION_EARLY_CLOSE_BLOCKS
    ] {
        let instant = ct(year, month, date, 9, 0, 0);
        assert_eq!(
            nkd.session_state(instant),
            Ok(SessionState::OpenRegular),
            "{year}-{month:02}-{date:02}: the matching session is the regular one"
        );
        assert_eq!(
            nkd.is_open_regular(instant),
            Ok(true),
            "{year}-{month:02}-{date:02}"
        );
        assert_eq!(
            nkd.is_open_extended(instant),
            Ok(false),
            "{year}-{month:02}-{date:02}: the family carries no extended session"
        );
        // The tradeable envelope itself does not move: the same instant stays
        // open and the bounds stay the ordinary wrapped run.
        assert_eq!(
            nkd.is_open(instant),
            Ok(true),
            "{year}-{month:02}-{date:02}: is_open is unaffected by the spelling"
        );
    }
}
