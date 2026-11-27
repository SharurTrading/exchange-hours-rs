// SPDX-License-Identifier: MIT-0

//! Brussels cash-calendar arrangements published for 2026.

use exchange_hours::{
    CalendarQueryErrorKind, Exchange, ExchangeCalendar, HolidayKind, SessionKind,
};
use jiff::Timestamp;
use jiff::civil::Date;

fn date(text: &str) -> Date {
    text.parse().expect("valid date")
}
fn utc(text: &str) -> Timestamp {
    text.parse().expect("valid UTC instant")
}

#[test]
fn brussels_published_closures_remove_the_executable_day() {
    let calendar = ExchangeCalendar::new(Exchange::EuronextBrussels);
    for day in [
        "2026-01-01",
        "2026-04-03",
        "2026-04-06",
        "2026-05-01",
        "2026-12-25",
        "2027-01-01",
    ] {
        assert_eq!(
            calendar
                .holiday_on(date(day))
                .map(exchange_hours::Holiday::kind),
            Some(HolidayKind::Closed)
        );
        assert_eq!(
            calendar.is_closed_trade_date(date(day), SessionKind::Both),
            Ok(true),
            "{day}: the market's column states a full closure"
        );
        assert_eq!(
            calendar.is_open(utc(&format!("{day}T12:00:00Z"))),
            Ok(false)
        );
    }
}

#[test]
fn brussels_normal_phases_and_dst_remain_the_named_profile() {
    let calendar = ExchangeCalendar::new(Exchange::EuronextBrussels);
    for (open, close, queue) in [
        (
            "2026-10-12T07:00:00Z",
            "2026-10-12T15:40:00Z",
            "2026-10-12T05:30:00Z",
        ),
        (
            "2026-10-26T08:00:00Z",
            "2026-10-26T16:40:00Z",
            "2026-10-26T06:30:00Z",
        ),
    ] {
        assert_eq!(calendar.is_open(utc(queue)), Ok(false));
        assert_eq!(calendar.is_accepting_orders(utc(queue)), Ok(true));
        assert_eq!(calendar.is_open(utc(open)), Ok(true));
        assert_eq!(
            calendar.session_bounds(utc(open)),
            Ok(Some((utc(open), utc(&close.replace(":40:", ":30:")))))
        );
        assert_eq!(calendar.is_open(utc(close)), Ok(false));
    }
    assert_eq!(
        calendar.next_session_open_after(utc("2026-04-02T18:00:00Z")),
        Ok(Some(utc("2026-04-07T07:00:00Z")))
    );
}

#[test]
fn brussels_unpublished_half_days_and_pre_window_classification_remain_typed() {
    let calendar = ExchangeCalendar::new(Exchange::EuronextBrussels);
    for day in ["2026-12-24", "2026-12-31"] {
        assert_eq!(
            calendar
                .holiday_on(date(day))
                .map(exchange_hours::Holiday::kind),
            Some(HolidayKind::Unsourced)
        );
        let gap = CalendarQueryErrorKind::UnresolvedGap {
            source: calendar.source(),
            date: date(day),
        };
        assert_eq!(
            calendar
                .is_open(utc(&format!("{day}T12:00:00Z")))
                .map_err(|error| error.kind()),
            Err(gap)
        );
        assert_eq!(
            calendar
                .is_closed_trade_date(date(day), SessionKind::Both)
                .map_err(|error| error.kind()),
            Err(gap)
        );
    }

    let pre_window = date("2025-12-31");
    assert_eq!(
        calendar
            .is_closed_trade_date(pre_window, SessionKind::Both)
            .map_err(|error| error.kind()),
        Err(CalendarQueryErrorKind::UnresolvedGap {
            source: calendar.source(),
            date: pre_window,
        })
    );
}
