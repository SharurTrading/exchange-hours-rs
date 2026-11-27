// SPDX-License-Identifier: MIT-0

//! ICE Cotton No. 2's Sunday opens, receiving trade dates and holiday queues.

use crate::support::{local, zones::NEW_YORK};
use exchange_hours::{ExchangeCalendar, Holiday, HolidayKind, MarketHoursKey, SessionKind};
use jiff::{SignedDuration, Timestamp, civil::Date};

const NANO: SignedDuration = SignedDuration::from_nanos(1);

fn at(d: Date, h: u32, m: u32) -> Timestamp {
    local(&NEW_YORK, (d.year(), d.month(), d.day()), (h, m, 0))
}

fn date(s: &str) -> Date {
    s.parse().unwrap()
}

fn cotton() -> ExchangeCalendar {
    ExchangeCalendar::for_market_hours_key(MarketHoursKey::IceUsCotton)
}

fn queue(c: ExchangeCalendar, open: Timestamp, end: Timestamp, owner: Date) {
    assert_eq!(c.is_accepting_orders(open - NANO), Ok(false), "{open}");
    for t in [open, end - NANO] {
        assert_eq!(c.is_accepting_orders(t), Ok(true), "{t}");
        assert_eq!(c.is_order_entry_only(t), Ok(true), "{t}");
        assert_eq!(c.is_open(t), Ok(false), "{t}");
        assert_eq!(c.trade_date(t), Ok(Some(owner)), "{t}");
    }
    assert_eq!(c.is_order_entry_only(end), Ok(false), "{end}");
}

fn matching(c: ExchangeCalendar, open: Timestamp, close: Timestamp, owner: Date) {
    assert_eq!(
        c.next_session_after(open - NANO),
        Ok(Some((open, close))),
        "{open}"
    );
    for t in [open, close - NANO] {
        assert_eq!(c.session_bounds(t), Ok(Some((open, close))), "{t}");
        assert_eq!(c.is_open_regular(t), Ok(true), "{t}");
        assert_eq!(c.trade_date(t), Ok(Some(owner)), "{t}");
    }
    assert_eq!(c.is_open(close), Ok(false), "{close}");
}

#[test]
fn cotton_sundays_and_dated_revisions_take_effect_on_the_opening_day() {
    let c = cotton().without_holidays();
    for (sunday, close_minute) in [
        ("2012-02-05", 30),
        ("2014-01-26", 30),
        ("2014-02-02", 20),
        ("2018-10-07", 20),
        ("2026-10-04", 20),
    ] {
        let sunday = date(sunday);
        let monday = sunday.tomorrow().unwrap();
        queue(c, at(sunday, 19, 30), at(sunday, 21, 0), monday);
        matching(c, at(sunday, 21, 0), at(monday, 14, close_minute), monday);
    }
    assert_eq!(
        c.is_order_entry_only(at(date("2018-10-04"), 15, 0)),
        Ok(false)
    );
    queue(
        c,
        at(date("2018-10-05"), 14, 50),
        at(date("2018-10-05"), 18, 0),
        date("2018-10-08"),
    );
}

#[test]
fn thanksgiving_queues_and_all_three_matching_days_keep_their_own_trade_dates() {
    let c = cotton();
    for (wednesday, friday, monday) in [
        ("2025-11-26", "2025-11-28", "2025-12-01"),
        ("2026-11-25", "2026-11-27", "2026-11-30"),
    ] {
        let (wednesday, friday, monday) = (date(wednesday), date(friday), date(monday));
        let thursday = wednesday.tomorrow().unwrap();
        let sunday = monday.yesterday().unwrap();
        let tuesday = monday.tomorrow().unwrap();
        queue(c, at(wednesday, 14, 50), at(wednesday, 18, 0), friday);
        for t in [
            at(thursday, 15, 0),
            at(thursday, 20, 00),
            at(thursday, 22, 00),
        ] {
            assert_eq!(c.is_accepting_orders(t), Ok(false), "{t}");
        }
        matching(c, at(friday, 8, 0), at(friday, 13, 30), friday);
        queue(c, at(friday, 14, 0), at(friday, 15, 30), monday);
        for t in [
            at(friday, 15, 30),
            at(friday, 17, 59),
            at(friday.tomorrow().unwrap(), 12, 0),
        ] {
            assert_eq!(c.is_accepting_orders(t), Ok(false), "{t}");
        }
        assert_eq!(
            c.next_session_after(at(friday, 13, 30)),
            Ok(Some((at(sunday, 21, 0), at(monday, 14, 20))))
        );
        queue(c, at(sunday, 19, 30), at(sunday, 21, 0), monday);
        matching(c, at(sunday, 21, 0), at(monday, 14, 20), monday);
        queue(c, at(monday, 14, 50), at(monday, 18, 0), tuesday);
        queue(c, at(monday, 19, 30), at(monday, 21, 0), tuesday);
        matching(c, at(monday, 21, 0), at(tuesday, 14, 20), tuesday);
    }
}

#[test]
fn a_closed_trade_date_moves_the_prior_business_day_queue_to_the_reopening() {
    let c = cotton();
    for (closed, previous, next) in [
        ("2025-01-01", "2024-12-31", "2025-01-02"),
        ("2025-01-20", "2025-01-17", "2025-01-21"),
        ("2025-02-17", "2025-02-14", "2025-02-18"),
        ("2025-04-18", "2025-04-17", "2025-04-21"),
        ("2025-05-26", "2025-05-23", "2025-05-27"),
        ("2025-06-19", "2025-06-18", "2025-06-20"),
        ("2025-07-04", "2025-07-03", "2025-07-07"),
        ("2025-09-01", "2025-08-29", "2025-09-02"),
        ("2026-01-01", "2025-12-31", "2026-01-02"),
        ("2026-01-19", "2026-01-16", "2026-01-20"),
        ("2026-02-16", "2026-02-13", "2026-02-17"),
        ("2026-04-03", "2026-04-02", "2026-04-06"),
        ("2026-05-25", "2026-05-22", "2026-05-26"),
        ("2026-06-19", "2026-06-18", "2026-06-22"),
        ("2026-07-03", "2026-07-02", "2026-07-06"),
        ("2026-09-07", "2026-09-04", "2026-09-08"),
        ("2027-01-01", "2026-12-31", "2027-01-04"),
        ("2027-01-18", "2027-01-15", "2027-01-19"),
        ("2027-02-15", "2027-02-12", "2027-02-16"),
        ("2027-03-26", "2027-03-25", "2027-03-29"),
        ("2027-05-31", "2027-05-28", "2027-06-01"),
        ("2027-06-18", "2027-06-17", "2027-06-21"),
        ("2027-07-05", "2027-07-02", "2027-07-06"),
        ("2027-09-06", "2027-09-03", "2027-09-07"),
    ] {
        let (closed, previous, next) = (date(closed), date(previous), date(next));
        assert_eq!(
            c.is_closed_trade_date(closed, SessionKind::Both),
            Ok(true),
            "{closed}"
        );
        assert_eq!(
            c.is_accepting_orders(at(closed, 15, 0)),
            Ok(false),
            "{closed}"
        );
        queue(c, at(previous, 14, 50), at(previous, 18, 0), next);
        assert_eq!(c.is_open(at(next, 9, 0)), Ok(true), "{next}");
        assert_eq!(c.trade_date(at(next, 9, 0)), Ok(Some(next)), "{next}");
    }
}

#[test]
fn christmas_queues_feed_the_published_reopens() {
    let c = cotton();
    for (eve, reopen, hour, minute) in [
        ("2025-12-24", "2025-12-26", 7, 30),
        ("2026-12-24", "2026-12-28", 8, 0),
    ] {
        let (eve, reopen) = (date(eve), date(reopen));
        queue(c, at(eve, 13, 35), at(eve, 15, 30), reopen);
        assert_eq!(
            c.is_accepting_orders(at(eve.tomorrow().unwrap(), 15, 0)),
            Ok(false)
        );
        matching(c, at(reopen, hour, minute), at(reopen, 14, 20), reopen);
    }
    assert_eq!(c.holiday_on(date("2026-12-29")), None);
    queue(
        c,
        at(date("2026-12-28"), 14, 50),
        at(date("2026-12-28"), 18, 0),
        date("2026-12-29"),
    );
}

#[test]
fn annual_cotton_closures_do_not_supply_unpublished_holiday_hours() {
    let c = cotton();
    for d in [
        "2027-11-24",
        "2027-11-26",
        "2027-11-29",
        "2027-12-23",
        "2027-12-27",
        "2027-12-28",
    ] {
        let d = date(d);
        assert_eq!(
            c.holiday_on(d).map(Holiday::kind),
            Some(HolidayKind::Unsourced)
        );
        let err = c.is_order_entry_only(at(d, 15, 0)).unwrap_err();
        assert!(
            c.holiday_on(err.date())
                .is_some_and(|h| h.kind() == HolidayKind::Unsourced)
        );
    }
    for d in ["2027-11-25", "2027-12-24"] {
        assert_eq!(
            c.holiday_on(date(d)).map(Holiday::kind),
            Some(HolidayKind::Closed)
        );
    }
    let sunday = date("2027-11-28");
    let monday = date("2027-11-29");
    assert!(c.is_open(at(sunday, 21, 0)).is_err());
    let selected = c.with_last_known_week();
    queue(selected, at(sunday, 19, 30), at(sunday, 21, 0), monday);
    matching(selected, at(sunday, 21, 0), at(monday, 14, 20), monday);
}
