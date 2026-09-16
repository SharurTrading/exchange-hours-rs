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
    CalendarResolution, EvidenceTier, ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey,
    SessionKind, calendar_for_market_hours_key,
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
    assert!(nkd.is_closed_trade_date(christmas, SessionKind::Both));

    for (hour, minute) in [(0, 30), (10, 0), (15, 59)] {
        assert!(
            !nkd.is_open(ct(2025, 12, 25, hour, minute, 0)),
            "no session belongs to trade date 2025-12-25, so {hour:02}:{minute:02} CT is closed"
        );
    }

    // The previous evening's normal 17:00 CT open is gone with it: that
    // occurrence's trade date was the closed Thursday.
    assert!(
        !nkd.is_open(ct(2025, 12, 24, 17, 30, 0)),
        "the Wednesday-evening leg carried trade date 2025-12-25 and must be deleted"
    );

    // ...while the Thursday-evening leg, whose trade date is Friday, survives.
    assert!(
        nkd.is_open(ct(2025, 12, 25, 18, 0, 0)),
        "the 17:00 CT open on Christmas Day carries trade date 2025-12-26 and still runs"
    );
    assert!(
        !nkd.is_closed_all_day_on(christmas, SessionKind::Both),
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
    assert!(!nkd.is_open(ct(2025, 4, 18, 9, 0, 0)));
    assert!(
        !nkd.is_open(ct(2025, 4, 17, 17, 30, 0)),
        "the Thursday-evening leg carried the closed Friday and must be deleted"
    );
    assert_eq!(
        nkd.next_session_after(ct(2025, 4, 17, 12, 0, 0))
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
        nkd.is_open(ct(2025, 12, 24, 12, 14, 59)),
        "12:14:59 CT is inside the 12:15 CT early close"
    );
    assert!(
        nkd.is_open(ct(2025, 12, 24, 3, 0, 0)),
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
        !nkd.is_open(cutoff),
        "an instant equal to a close is closed"
    );
    assert_eq!(
        nkd.session_bounds(ct(2025, 12, 24, 10, 0, 0)),
        Some((ct(2025, 12, 23, 17, 0, 0), cutoff)),
        "the session that opened Tuesday 17:00 CT now ends at the clipped close"
    );
    assert_eq!(
        nkd.candle_end(ct(2025, 12, 24, 10, 0, 0), CalendarResolution::Daily),
        Some(cutoff),
        "the daily bar closes with the trade date"
    );
    assert!(
        !nkd.is_open(ct(2025, 12, 24, 15, 59, 0)),
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
    assert!(nkd.is_open(ct(2026, 4, 3, 8, 14, 59)));
    assert!(!nkd.is_open(ct(2026, 4, 3, 8, 15, 0)));
    assert!(
        !nkd.is_open(ct(2026, 4, 3, 10, 14, 0)),
        "10:15 CT is the Interest Rates, FX and Cryptocurrency instant, not this family's"
    );
    assert_eq!(
        nkd.next_session_after(ct(2026, 4, 3, 9, 0, 0))
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
                other => panic!("{date} ships an unexpected holiday kind: {other:?}"),
            }
        }
        date = date.succ_opt().expect("coverage stays inside the calendar");
    }

    assert_eq!(closed, 9, "nine full closures across 2025-2027");
    assert_eq!(early, 28, "twenty-eight early closes across 2025-2027");

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
    assert!(!nkd.is_open(ct(2025, 12, 25, 16, 59, 59)));
    assert!(
        nkd.is_open(ct(2025, 12, 25, 17, 0, 0)),
        "Christmas evening reopens at the grid's own 17:00 CT, not later"
    );
    assert!(!nkd.is_open(ct(2027, 3, 28, 16, 59, 59)));
    assert!(
        nkd.is_open(ct(2027, 3, 28, 17, 0, 0)),
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

    assert!(!nkd.is_open(ct(2025, 12, 24, 17, 30, 0)));
    assert_eq!(
        nkd.next_session_after(ct(2025, 12, 24, 12, 20, 0)),
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
        nkd.trade_date(ct(2025, 12, 24, 10, 0, 0)),
        Some(day(2025, 12, 24)),
        "a shortened day keeps its own trade date"
    );
    assert_eq!(
        nkd.trade_date(ct(2025, 12, 25, 18, 0, 0)),
        Some(day(2025, 12, 26)),
        "the evening open on the closed date already belongs to the next trade date"
    );

    // A Monday holiday, where the clipped session opened on the Sunday.
    assert!(nkd.is_open(ct(2026, 1, 19, 11, 59, 59)));
    assert!(!nkd.is_open(ct(2026, 1, 19, 12, 0, 0)));
    assert_eq!(
        nkd.trade_date(ct(2026, 1, 19, 10, 0, 0)),
        Some(day(2026, 1, 19))
    );
    assert_eq!(
        nkd.trade_date(ct(2026, 1, 19, 18, 0, 0)),
        Some(day(2026, 1, 20)),
        "the Monday-evening leg runs normally into Tuesday's trade date"
    );
    assert!(
        nkd.is_open(ct(2026, 1, 19, 17, 30, 0)),
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
    assert!(nkd.is_open(ct(2025, 1, 20, 11, 59, 59)));
    assert!(!nkd.is_open(ct(2025, 1, 20, 12, 0, 0)));
    // Independence Day eve 2025, the one date where the Equity Index line
    // diverges from Rates, FX, Energy, Metals and Cryptocurrency: 12:15 CT
    // against their 16:00 CT.
    assert!(nkd.is_open(ct(2025, 7, 3, 12, 14, 59)));
    assert!(!nkd.is_open(ct(2025, 7, 3, 12, 15, 0)));
    assert!(
        nkd.is_open(ct(2025, 7, 3, 17, 30, 0)),
        "the evening leg runs normally into trade date 2025-07-04"
    );
    // ...which is itself an early close at 12:00 CT.
    assert!(nkd.is_open(ct(2025, 7, 4, 11, 59, 59)));
    assert!(!nkd.is_open(ct(2025, 7, 4, 12, 0, 0)));
}

/// Case 7 — both edges of the coverage window.
///
/// Inside the window a date with no row is audited normal; outside it the
/// crate has no holiday answer at all and serves the normal week. Christmas
/// 2021 is the proof that the table does not silently extend into the
/// 2019-2021 interval no wave audited.
#[test]
fn holiday_coverage_bounds_what_the_table_answers_for() {
    let nkd = nkd();
    let coverage = nkd
        .holiday_coverage()
        .expect("the family ships a built-in table");

    assert_eq!(coverage.first(), day(2016, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2026, 4, 3)));
    assert!(coverage.contains(day(2016, 1, 1)));
    assert!(!coverage.contains(day(2015, 12, 31)));
    assert!(!coverage.contains(day(2019, 1, 1)));
    // 2024-12-31 entered the table with the 2022-2024 wave, as an `Unsourced`
    // statement: inside a window, so `contains` is true and the row is the
    // family's own note that no document covered the date.
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert!(nkd.holiday_on(day(2015, 12, 31)).is_none());
    assert!(nkd.holiday_on(day(2019, 1, 1)).is_none());
    assert_eq!(
        kind_on(nkd, day(2024, 12, 31)),
        Some(HolidayKind::Unsourced)
    );
    assert!(nkd.holiday_on(day(2028, 1, 1)).is_none());
    assert!(
        nkd.holiday_on(day(2025, 1, 1)).is_some(),
        "the first audited date is inside the window"
    );

    // Christmas 2021 is a Friday the operator shut, and it sits in the
    // 2019-2021 interval no wave audited: the crate serves the normal week
    // rather than guessing. (Christmas 2024, which this probe used before the
    // 2022-2024 wave shipped, is now an `Unsourced` row inside the window; it
    // clips nothing, so the normal week still answers there too.)
    assert_eq!(nkd.holiday_on(day(2021, 12, 24)), None);
    assert!(
        nkd.is_open(ct(2021, 12, 24, 10, 0, 0)),
        "a holiday below the coverage window is not applied"
    );
    assert_eq!(
        nkd.is_open(ct(2021, 12, 24, 10, 0, 0)),
        nkd.without_holidays().is_open(ct(2021, 12, 24, 10, 0, 0))
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
        bare.is_open(ct(2025, 12, 25, 10, 0, 0)),
        "detached, Christmas Day is an ordinary Thursday trade date"
    );
    assert!(bare.is_open(ct(2025, 12, 24, 15, 0, 0)));

    let week = Duration::days(7);
    let mut probe = ct(2025, 12, 22, 0, 0, 0);
    let end = ct(2025, 12, 27, 0, 0, 0);
    let mut diverged = false;
    while probe < end {
        assert_eq!(
            bare.is_open(probe),
            nkd.is_open(probe - week),
            "a detached calendar must answer the normal week at {probe}"
        );
        diverged |= nkd.is_open(probe) != bare.is_open(probe);
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
            nkd.is_open(ct(year, month, date, 11, 59, 59)),
            "{year}-{month:02}-{date:02} closes at 12:00 CT, so 11:59:59 is open"
        );
        assert!(
            !nkd.is_open(ct(year, month, date, 12, 0, 0)),
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
            nkd.is_open(ct(year, month, date, 12, 14, 59)),
            "{year}-{month:02}-{date:02} closes at 12:15 CT, so 12:14:59 is open"
        );
        assert!(
            !nkd.is_open(ct(year, month, date, 12, 15, 0)),
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
        assert!(nkd.is_closed_trade_date(closed, SessionKind::Both));
        assert!(!nkd.is_open(ct(year, month, date, 10, 0, 0)));
    }

    assert!(
        nkd.holiday_on(day(2027, 12, 23)).is_none(),
        "2027-12-23 is audited normal: its 16:00 CT close is the family's own"
    );
    assert!(
        nkd.is_open(ct(2027, 12, 23, 15, 59, 0)),
        "the Thursday daytime session runs to its normal 16:00 CT close"
    );
    assert!(
        !nkd.is_open(ct(2027, 12, 23, 17, 30, 0)),
        "the Thursday-evening leg carried the closed Friday and is deleted by that row"
    );
    assert_eq!(
        nkd.next_session_after(ct(2027, 12, 23, 16, 30, 0))
            .map(|(open, _close)| open),
        Some(ct(2027, 12, 26, 17, 0, 0)),
        "the next session is the Sunday-evening open for trade date 2027-12-27"
    );
}

/// The Saturday sessions CME publishes and this table cannot state.
///
/// `2026-06-20`, `2026-07-04` and `2027-06-19` carry
/// `05:00 open; 17:00 closed` in CME's own service. The family's normal week
/// has no Saturday session, and a late open can only push an existing
/// occurrence later, so these are declared gaps in the evidence file rather
/// than rows. This fence records the consequence a consumer sees, so the gap
/// cannot be closed silently.
#[test]
fn the_published_saturday_sessions_are_declared_gaps_and_report_closed() {
    let nkd = nkd();

    for (year, month, date) in [(2026, 6, 20), (2026, 7, 4), (2027, 6, 19)] {
        assert!(
            !nkd.is_open(ct(year, month, date, 9, 0, 0)),
            "{year}-{month:02}-{date:02} is a sourced Saturday session the scalar \
             vocabulary cannot state"
        );
        assert!(nkd.holiday_on(day(year, month, date)).is_none());
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

/// The era's early closes and its one late open, probed on both sides of the
/// printed instant.
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
    assert!(calendar.is_open(noon - Duration::seconds(1)));
    assert!(!calendar.is_open(noon));
    assert!(calendar.is_open(ct(2016, 1, 17, 18, 0, 0)));

    // 2018-12-26: the Christmas sheet's Equity line opens the trade date at
    // 15:30 CT rather than the grid's 15:15.
    let late = ct(2018, 12, 26, 15, 30, 0);
    assert_eq!(
        kind_on(calendar, day(2018, 12, 26)),
        Some(HolidayKind::LateOpen {
            open_ssm: 15 * 3_600 + 30 * 60
        })
    );
    assert!(!calendar.is_open(late - Duration::seconds(1)));
    assert!(calendar.is_open(late));
    assert_eq!(calendar.trade_date(late), Some(day(2018, 12, 26)));
}

/// The era's own window edges, and the interval below it.
#[test]
fn wave2_window_edges_answer_as_declared() {
    let calendar = nkd();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");

    assert!(coverage.contains(day(2016, 1, 1)));
    assert!(coverage.contains(day(2018, 12, 31)));
    assert!(!coverage.contains(day(2015, 12, 31)));
    assert!(!coverage.contains(day(2019, 1, 1)));
    for date in [(2013, 6, 14), (2015, 12, 25), (2019, 1, 1)] {
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }
    assert!(calendar.is_open(ct(2015, 12, 25, 10, 0, 0)));
    assert_eq!(
        calendar.is_open(ct(2015, 12, 25, 10, 0, 0)),
        bare.is_open(ct(2015, 12, 25, 10, 0, 0))
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
        (11, 3, 5, 17),
        "the era's shape"
    );
}

/// A closure deletes the trade date and the leg that opened it the previous
/// evening, and whatever the crate offers next is the ordinary 17:00 CT
/// evening open — named here so a shifted reopen fails.
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
    assert_eq!(closures, 5, "the era's closures");
}

/// Every query about an `Unsourced` date answers exactly as the detached
/// calendar does: the row states that the date was audited, makes no
/// scheduling claim, and clips nothing.
///
/// One exception is structural rather than a clip: a query that looks *past*
/// the date — `next_session_open_after`, and a probe inside the evening leg
/// that opens the next trade date — is answered by the **next** date's row. On
/// 2024-12-31 that successor is 2025-01-01, a real closure in the 2025-2027
/// window, so those two answers differ from the detached calendar for a reason
/// that has nothing to do with this date's `Unsourced` statement. They are
/// compared only where the successor carries no such row.
fn assert_unsourced_changes_nothing(date: NaiveDate, row: Holiday, tier: EvidenceTier) {
    let calendar = nkd();
    let detached = calendar.without_holidays();
    assert_eq!(row.kind(), HolidayKind::Unsourced, "{date}");
    assert_eq!(row.tier(), tier, "{date}");
    assert!(
        !calendar.is_closed_trade_date(date, SessionKind::Both),
        "{date} must not be reported closed"
    );

    let successor_clips = calendar
        .holiday_on(day_after(date))
        .is_some_and(|next| next.kind() != HolidayKind::Unsourced);
    let mut probes = vec![
        ct_on(day_before(date), (18, 0, 0)),
        ct_on(date, (9, 0, 0)),
        ct_on(date, (15, 59, 0)),
    ];
    if !successor_clips {
        probes.push(ct_on(date, (18, 0, 0)));
    }

    for probe in probes {
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
            calendar.candle_end(probe, CalendarResolution::Daily),
            detached.candle_end(probe, CalendarResolution::Daily),
            "{probe}"
        );
        if !successor_clips {
            assert_eq!(
                calendar.next_session_open_after(probe),
                detached.next_session_open_after(probe),
                "{probe}"
            );
        }
    }

    if successor_clips {
        // The difference the guard skips is the successor's, and it is exactly
        // the successor's row: with that one date detached too, the answers
        // agree again — here asserted through the successor's own closure.
        let next = day_after(date);
        assert!(
            calendar.is_closed_trade_date(next, SessionKind::Both),
            "{next} must be the row that moves the forward-looking answer"
        );
        assert!(
            !calendar.is_open(ct_on(date, (18, 0, 0))),
            "{date}: the evening leg into the closed successor is gone"
        );
        assert!(detached.is_open(ct_on(date, (18, 0, 0))));
    }
}

/// All seventeen `Unsourced` dates, not a sample: the identity's answers are
/// exactly the detached calendar's on each of them.
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
    // outright Nikkei's own close is not stated, so no early close ships and
    // the ordinary Monday answers instead.
    assert!(calendar.is_open(ct(2022, 5, 30, 10, 0, 0)));
    assert_eq!(
        calendar.is_open(ct(2022, 5, 30, 10, 0, 0)),
        calendar
            .without_holidays()
            .is_open(ct(2022, 5, 30, 10, 0, 0))
    );

    let service = calendar
        .holiday_on(day(2024, 7, 3))
        .expect("2024-07-03 ships a row");
    assert_eq!(service.kind(), HolidayKind::Unsourced);
    assert_eq!(service.tier(), EvidenceTier::T2);
    assert_eq!(service.document_id(), "CME-SVC-2024-07-03");
    assert!(calendar.is_open(ct(2024, 7, 3, 15, 59, 0)));
}

/// The new window sits second in the declared coverage, its edges answer, and
/// the 2019-2021 interval between it and the 2016-2018 wave stays unaudited.
#[test]
fn era_2022_2024_window_sits_second_and_the_2019_2021_interval_is_unaudited() {
    let calendar = nkd();
    let bare = calendar.without_holidays();
    let coverage = calendar
        .holiday_coverage()
        .expect("the family ships a table");

    assert_eq!(
        coverage.windows(),
        vec![
            (day(2016, 1, 1), day(2018, 12, 31)),
            (day(2022, 1, 1), day(2024, 12, 31)),
            (day(2025, 1, 1), day(2027, 12, 31)),
        ]
    );
    assert!(coverage.contains(day(2022, 1, 1)));
    assert!(coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2021, 12, 31)));
    assert_eq!(calendar.holiday_on(day(2022, 1, 1)), None);
    assert_eq!(
        kind_on(calendar, day(2024, 12, 31)),
        Some(HolidayKind::Unsourced)
    );

    for date in [(2019, 1, 1), (2020, 12, 25), (2021, 7, 5)] {
        assert!(!coverage.contains(day(date.0, date.1, date.2)), "{date:?}");
        assert_eq!(
            calendar.holiday_on(day(date.0, date.1, date.2)),
            None,
            "{date:?}"
        );
    }
    // Christmas 2020 is a real CME closure no wave audited; the ordinary
    // Friday answers, and the detached calendar answers it the same.
    for probe in [ct(2020, 12, 25, 10, 0, 0), ct(2020, 12, 24, 18, 0, 0)] {
        assert!(calendar.is_open(probe), "{probe}");
        assert_eq!(calendar.is_open(probe), bare.is_open(probe), "{probe}");
    }
}
