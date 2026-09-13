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

use chrono::{Duration, NaiveDate, TimeZone as _, Utc};
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

/// Case 4 — the family ships no late open, and the table says so.
///
/// Every CME re-open after one of these closures is the grid's own 17:00 CT
/// evening open, so there is no row to push an occurrence later. The memo's
/// late-open case is therefore vacuous here, and this is the fence that keeps
/// it honest: it walks every date in the coverage window, so a late open
/// introduced by a later edit fails immediately rather than going untested.
#[test]
fn the_table_ships_no_late_open_and_holds_exactly_its_audited_rows() {
    let nkd = nkd();
    let coverage = nkd
        .holiday_coverage()
        .expect("the family ships a built-in table");

    let mut closed = 0_usize;
    let mut early = 0_usize;
    let mut date = coverage.first();
    while date <= coverage.last() {
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
/// 2024 is the proof that the table does not silently extend below its first
/// audited date.
#[test]
fn holiday_coverage_bounds_what_the_table_answers_for() {
    let nkd = nkd();
    let coverage = nkd
        .holiday_coverage()
        .expect("the family ships a built-in table");

    assert_eq!(coverage.first(), day(2025, 1, 1));
    assert_eq!(coverage.last(), day(2027, 12, 31));
    assert!(coverage.contains(day(2026, 4, 3)));
    assert!(!coverage.contains(day(2024, 12, 31)));
    assert!(!coverage.contains(day(2028, 1, 1)));

    assert!(nkd.holiday_on(day(2024, 12, 31)).is_none());
    assert!(nkd.holiday_on(day(2028, 1, 1)).is_none());
    assert!(
        nkd.holiday_on(day(2025, 1, 1)).is_some(),
        "the first audited date is inside the window"
    );

    // Christmas 2024 is a Wednesday one week below the window. CME shut Globex
    // that day too, but the table does not audit it, so the crate serves the
    // normal week rather than guessing.
    assert!(
        nkd.is_open(ct(2024, 12, 25, 10, 0, 0)),
        "a holiday below the coverage window is not applied"
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
