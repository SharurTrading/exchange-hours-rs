// SPDX-License-Identifier: MIT-0

//! OANDA's own book: the FX (ex-TRY) grid, the TRY grid, and the venue
//! default that shares the FX clock.

use super::prelude::*;

// ---------------------------------------------------------------------------
// oanda_fx — Sunday 17:05 through Friday 16:59 New York time, six-minute
// 16:59-17:05 daily break, no other maintenance.
// ---------------------------------------------------------------------------

#[test]
fn oanda_fx_opens_sunday_1705_new_york_in_both_dst_regimes() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaFx, et((2026, 1, 12), (12, 0, 0)));
    // Winter (EST): 17:05 ET is 22:05 UTC.
    assert!(
        !h.is_open(et((2026, 1, 11), (17, 4, 59))),
        "one second before the winter open"
    );
    assert!(
        h.is_open(et((2026, 1, 11), (17, 5, 0))),
        "the winter open instant"
    );
    // Summer (EDT): the zone carries the transition; 17:05 ET is 21:05 UTC.
    assert!(
        !h.is_open(et((2026, 7, 12), (17, 4, 59))),
        "one second before the summer open"
    );
    assert!(
        h.is_open(et((2026, 7, 12), (17, 5, 0))),
        "the summer open instant"
    );
}

#[test]
fn oanda_fx_daily_break_is_six_minutes_and_end_exclusive() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaFx, et((2026, 6, 10), (9, 0, 0)));
    // Wednesday trades to 16:59 (end-exclusive close) and reopens 17:05.
    assert!(
        h.is_open(et((2026, 6, 10), (16, 58, 59))),
        "the last minute before the close"
    );
    assert!(
        !h.is_open(et((2026, 6, 10), (16, 59, 0))),
        "the close instant is closed"
    );
    assert!(
        !h.is_open(et((2026, 6, 10), (17, 4, 59))),
        "inside the six-minute break"
    );
    assert!(
        h.is_open(et((2026, 6, 10), (17, 5, 0))),
        "the evening reopen"
    );
    assert_eq!(h.tz, America::New_York);
}

#[test]
fn oanda_fx_closes_for_the_week_friday_1659_and_stays_closed_all_weekend() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaFx, et((2026, 6, 12), (9, 0, 0)));
    assert!(
        h.is_open(et((2026, 6, 12), (16, 58, 59))),
        "the week's last minute"
    );
    assert!(
        !h.is_open(et((2026, 6, 12), (16, 59, 0))),
        "the weekly close instant"
    );
    assert!(!h.is_open(et((2026, 6, 13), (12, 0, 0))), "Saturday midday");
    assert!(
        !h.is_open(et((2026, 6, 14), (12, 0, 0))),
        "Sunday midday before the reopen"
    );
    assert!(
        h.is_open(et((2026, 6, 14), (17, 5, 0))),
        "the next week's Sunday open"
    );
    assert!(h.has_daily_close);
    assert!(h.has_weekend_close);
}

#[test]
fn oanda_fx_session_has_no_order_entry_or_extended_phases() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaFx, et((2026, 6, 10), (9, 0, 0)));
    assert!(h.extended.is_empty(), "the book states no extended phase");
    assert!(
        h.order_entry.is_empty(),
        "the book states no order-entry-only phase"
    );
    assert!(!h.is_order_entry_only(et((2026, 6, 10), (9, 0, 0))));
    assert!(h.is_accepting_orders(et((2026, 6, 10), (9, 0, 0))));
}

#[test]
fn oanda_fx_trade_date_is_the_closing_day_and_sunday_opens_the_week() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaFx);
    // Inside the audited window: Wednesday noon sits in the session that
    // opened Tuesday 17:05 and closes Wednesday 16:59, so its trade date is
    // the closing day, Wednesday.
    assert_eq!(
        calendar
            .trade_date(et((2026, 6, 10), (12, 0, 0)))
            .expect("a covered date answers")
            .expect("a containing session has a trade date"),
        chrono::NaiveDate::from_ymd_opt(2026, 6, 10).expect("valid date"),
    );
}

#[test]
fn oanda_fx_holiday_rows_close_their_trade_dates() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaFx);
    for day in [(2025, 1, 1), (2025, 12, 25), (2026, 1, 1)] {
        let date = chrono::NaiveDate::from_ymd_opt(day.0, day.1, day.2).expect("valid date");
        assert!(
            calendar
                .is_closed_trade_date(date, SessionKind::Both)
                .expect("inside the audited window the holiday layer answers"),
            "{date} is the operator's stated FX closure",
        );
    }
    // Dates whose only FX statement is the normal daily close carry no
    // holiday row: Christmas Eve 2025's `close at 16:59` is the no-reopen.
    assert!(
        !calendar
            .is_closed_trade_date(
                chrono::NaiveDate::from_ymd_opt(2025, 12, 24).expect("valid date"),
                SessionKind::Both,
            )
            .expect("inside the audited window the holiday layer answers"),
    );
}

#[test]
fn oanda_fx_refuses_dates_outside_its_sourced_and_audited_spans() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaFx);
    // Below the sourced span (the 2024-02-21 witness) the grid is carried,
    // so the date-aware surface refuses rather than answering on carried
    // data.
    assert_refused(
        calendar.is_closed_trade_date(
            chrono::NaiveDate::from_ymd_opt(2023, 6, 7).expect("valid date"),
            SessionKind::Both,
        ),
        DateCoverage::OutsideCoveredRange,
        calendar,
        et((2023, 6, 7), (12, 0, 0)),
    );
    // Beyond the audited holiday window (2026-10-30) the table has no answer.
    assert_refused(
        calendar.is_closed_trade_date(
            chrono::NaiveDate::from_ymd_opt(2026, 12, 25).expect("valid date"),
            SessionKind::Both,
        ),
        DateCoverage::OutsideCoveredRange,
        calendar,
        et((2026, 12, 25), (12, 0, 0)),
    );
}

#[test]
fn oanda_fx_bridge_day_answers_sessions_while_its_holiday_layer_refuses() {
    // 2024-12-31 sits below the first audited holiday window (which starts
    // 2025-01-01) with the normal week sourced from the 2024-02-21 witness:
    // the one-flank bridged residual of the 2026-10-07 ruling (#296). Its
    // session questions answer from the sourced week; its holiday
    // classification refuses typed.
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaFx);
    let bridge = chrono::NaiveDate::from_ymd_opt(2024, 12, 31).expect("valid date");
    assert!(
        calendar
            .is_open(et((2024, 12, 31), (12, 0, 0)))
            .expect("the bridged span answers its session questions"),
        "New Year's Eve midday trades on the sourced week",
    );
    assert!(
        !calendar
            .is_open(et((2024, 12, 31), (17, 0, 0)))
            .expect("the bridged span answers its session questions"),
        "the 16:59-17:05 break answers on the sourced week",
    );
    assert!(
        calendar
            .is_closed_trade_date(bridge, SessionKind::Both)
            .is_err(),
        "the bridge day's holiday classification refuses, never answers a bool",
    );
    // The first holiday row's own date answers: its prior-evening opening day
    // is inside the sourced span, so no resolution edge reaches it.
    assert!(
        calendar
            .is_closed_trade_date(
                chrono::NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                SessionKind::Both,
            )
            .expect("inside the audited window the holiday layer answers"),
    );
}

// ---------------------------------------------------------------------------
// oanda_try_fx — daily Monday-Friday 08:00-17:00 Central European time.
// ---------------------------------------------------------------------------

#[test]
fn oanda_try_fx_runs_a_daily_0800_1700_central_european_session() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaTryFx, cet((2026, 6, 10), (12, 0, 0)));
    assert!(
        !h.is_open(cet((2026, 6, 10), (7, 59, 59))),
        "one second before the open"
    );
    assert!(h.is_open(cet((2026, 6, 10), (8, 0, 0))), "the open instant");
    assert!(
        h.is_open(cet((2026, 6, 10), (16, 59, 59))),
        "the last minute"
    );
    assert!(
        !h.is_open(cet((2026, 6, 10), (17, 0, 0))),
        "the close is end-exclusive"
    );
    // Overnight gap: the book reopens the next morning, not at 17:05.
    assert!(
        !h.is_open(cet((2026, 6, 10), (18, 0, 0))),
        "the evening gap"
    );
}

#[test]
fn oanda_try_fx_closes_for_the_weekend_and_carries_dst_in_the_zone() {
    let h = hours_for_market_hours_key(MarketHoursKey::OandaTryFx, cet((2026, 6, 12), (12, 0, 0)));
    assert!(!h.is_open(cet((2026, 6, 13), (12, 0, 0))), "Saturday");
    assert!(!h.is_open(cet((2026, 6, 14), (12, 0, 0))), "Sunday");
    assert!(h.is_open(cet((2026, 6, 15), (8, 0, 0))), "Monday reopen");
    // Winter (CET): 08:00 local is 07:00 UTC; summer (CEST): 06:00 UTC.
    assert_eq!(
        cet((2026, 1, 12), (8, 0, 0)),
        utc((2026, 1, 12), (7, 0, 0)),
        "the zone anchors the winter session"
    );
    assert_eq!(
        cet((2026, 7, 13), (8, 0, 0)),
        utc((2026, 7, 13), (6, 0, 0)),
        "the zone anchors the summer session"
    );
    assert_eq!(h.tz, Europe::Berlin);
}

#[test]
fn oanda_try_fx_refuses_its_carried_history() {
    let calendar = calendar_for_market_hours_key(MarketHoursKey::OandaTryFx);
    // The TRY grid's only witness is the 2026-10-10 capture; every earlier
    // day is carried and the date-aware surface refuses it.
    assert_refused(
        calendar.is_closed_trade_date(
            chrono::NaiveDate::from_ymd_opt(2026, 6, 10).expect("valid date"),
            SessionKind::Both,
        ),
        DateCoverage::OutsideCoveredRange,
        calendar,
        cet((2026, 6, 10), (12, 0, 0)),
    );
}

// ---------------------------------------------------------------------------
// The venue default and the wire forms.
// ---------------------------------------------------------------------------

#[test]
fn oanda_venue_default_is_the_fx_grid() {
    let venue = hours_for_exchange(Exchange::Oanda, et((2026, 6, 10), (9, 0, 0)));
    let fx = hours_for_market_hours_key(MarketHoursKey::OandaFx, et((2026, 6, 10), (9, 0, 0)));
    assert_eq!(venue.tz, fx.tz);
    assert!(
        venue.is_open(et((2026, 6, 10), (17, 5, 0))),
        "the venue answers the FX evening reopen"
    );
    assert!(
        !venue.is_open(et((2026, 6, 10), (17, 2, 0))),
        "the venue answers the FX daily break"
    );
    assert!(
        !venue.is_open(et((2026, 6, 13), (12, 0, 0))),
        "the venue shares the FX weekend"
    );
}

#[test]
fn oanda_wire_forms_are_snake_case_and_stable() {
    assert_eq!(Exchange::Oanda.as_str(), "oanda");
    assert_eq!(MarketHoursKey::OandaFx.as_str(), "oanda_fx",);
    assert_eq!(MarketHoursKey::OandaTryFx.as_str(), "oanda_try_fx",);
    assert_eq!("oanda".parse::<Exchange>().unwrap(), Exchange::Oanda);
    assert_eq!(
        "oanda_fx".parse::<MarketHoursKey>().unwrap(),
        MarketHoursKey::OandaFx,
    );
    assert_eq!(
        "oanda_try_fx".parse::<MarketHoursKey>().unwrap(),
        MarketHoursKey::OandaTryFx,
    );
    assert!("oanda_fx_book".parse::<MarketHoursKey>().is_err());
}
