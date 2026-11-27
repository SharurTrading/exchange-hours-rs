// SPDX-License-Identifier: MIT-0

//! The `bme_ibex` family: the three sourced grids, the two dated cutovers
//! keyed at Madrid local midnight, the 14:00 Christmas-eve early closes, and
//! the typed coverage refusals the partial history owes.
//!
//! Every probe is stated in Europe/Madrid wall clock, so a DST slip fails
//! rather than passing on a coincidence. The 2017-01-23 cutover is a Monday,
//! so its Friday-to-Monday pair distinguishes the 09:00-open and 08:00-open
//! grids at the same wall clock.

use crate::support::zones::MADRID;
use crate::support::{date_opt, local_opt};
use exchange_hours::{
    CalendarQueryErrorKind, HolidayKind, MarketHoursKey, SessionKind, SessionState,
    calendar_for_market_hours_key, hours_for_market_hours_key,
};

const IBEX: MarketHoursKey = MarketHoursKey::BmeIbex;

fn madrid(date: (i16, i8, i8), time: (u32, u32, u32)) -> jiff::Timestamp {
    local_opt(&MADRID, date, time).expect("fixture must be an unambiguous Madrid instant")
}

fn day(date: (i16, i8, i8)) -> jiff::civil::Date {
    date_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

/// The regular-session bounds the dated selector serves at `instant`'s
/// Madrid local day, read off the fixed rule slice.
fn regular_bounds(instant: jiff::Timestamp) -> (u32, u32) {
    let hours = hours_for_market_hours_key(IBEX, instant);
    let rule = hours.regular.first().expect("one regular rule per era");
    (rule.open_ssm(), rule.close_ssm())
}

/// The floor grid is the operator's own 2010-06-18 product-page print:
/// auction 08:30-09:00 as order entry, trading 09:00-17:35.
#[test]
fn the_floor_grid_serves_the_2010_product_page_print() {
    let midweek = madrid((2016, 6, 15), (12, 0, 0));
    let hours = hours_for_market_hours_key(IBEX, midweek);

    assert!(!hours.is_accepting_orders(madrid((2016, 6, 15), (8, 29, 59))));
    assert!(
        hours.is_order_entry_only(madrid((2016, 6, 15), (8, 30, 0))),
        "the 08:30-09:00 opening auction accepts orders and matches nothing"
    );
    assert!(!hours.is_open(madrid((2016, 6, 15), (8, 59, 59))));
    assert!(
        hours.is_open(madrid((2016, 6, 15), (9, 0, 0))),
        "the operator prints trading from 9:00 a.m."
    );
    assert_eq!(
        hours.session_state(madrid((2016, 6, 15), (12, 0, 0))),
        SessionState::OpenRegular,
        "the era's one executable session is regular"
    );
    assert!(
        hours.is_open(madrid((2016, 6, 15), (17, 34, 59)))
            && !hours.is_open(madrid((2016, 6, 15), (17, 35, 0))),
        "the 17:35 close is end-exclusive"
    );
    assert!(
        !hours.is_open(madrid((2016, 6, 11), (12, 0, 0))),
        "the family has a true weekend close"
    );
}

/// The 2017-01-23 cutover is keyed to the Madrid opening day: the last
/// instant of 2017-01-22 still resolves to the floor grid and Madrid
/// midnight on the 23rd to the 08:00-open grid.
#[test]
fn the_2017_cutover_flips_at_madrid_local_midnight() {
    let last_floor =
        local_opt(&MADRID, (2017, 1, 22), (23, 59, 59)).expect("an unambiguous Madrid instant");
    let first_new =
        local_opt(&MADRID, (2017, 1, 23), (0, 0, 0)).expect("an unambiguous Madrid instant");

    assert_eq!(regular_bounds(last_floor), (9 * 3600, 17 * 3600 + 35 * 60));
    assert_eq!(regular_bounds(first_new), (8 * 3600, 20 * 3600));

    // The same wall clock on the Friday before and the Monday itself:
    // 08:30 is the floor's order-entry auction and the new grid's open.
    let friday = madrid((2017, 1, 20), (8, 30, 0));
    let monday = madrid((2017, 1, 23), (8, 30, 0));
    assert_eq!(
        hours_for_market_hours_key(IBEX, friday).session_state(friday),
        SessionState::OrderEntry,
        "the floor grid is still in the auction at 08:30 on the Friday"
    );
    assert_eq!(
        hours_for_market_hours_key(IBEX, monday).session_state(monday),
        SessionState::OpenRegular,
        "the press release's starting-today grid is open at 08:30 on the Monday"
    );
}

/// The 2025-06-23 cutover from notice 16/2025 extends the close to 22:00;
/// the conditional 2024-07-01 date of notice 7/2024 keys no row, so the
/// whole 2017 grid still serves straight through June 2025.
#[test]
fn the_2025_close_extension_flips_at_madrid_local_midnight() {
    let last_20 =
        local_opt(&MADRID, (2025, 6, 22), (23, 59, 59)).expect("an unambiguous Madrid instant");
    let first_22 =
        local_opt(&MADRID, (2025, 6, 23), (0, 0, 0)).expect("an unambiguous Madrid instant");

    assert_eq!(regular_bounds(last_20), (8 * 3600, 20 * 3600));
    assert_eq!(regular_bounds(first_22), (8 * 3600, 22 * 3600));

    // 21:00 on Friday 2025-06-20 is after the old close; on Monday the 23rd
    // the notice's extended session is open, end-exclusive at 22:00.
    assert!(
        !hours_for_market_hours_key(IBEX, madrid((2025, 6, 20), (21, 0, 0)))
            .is_open(madrid((2025, 6, 20), (21, 0, 0)))
    );
    let extended = hours_for_market_hours_key(IBEX, madrid((2025, 6, 23), (21, 0, 0)));
    assert!(extended.is_open(madrid((2025, 6, 23), (21, 59, 59))));
    assert!(!extended.is_open(madrid((2025, 6, 23), (22, 0, 0))));

    // The slipped conditional: the 2024-07-01 the first notice named never
    // took effect, so 2024 serves the 08:00-20:00 grid at every probe.
    let slipped = hours_for_market_hours_key(IBEX, madrid((2024, 7, 1), (12, 0, 0)));
    assert_eq!(
        regular_bounds(madrid((2024, 7, 1), (12, 0, 0))),
        (8 * 3600, 20 * 3600)
    );
    assert!(!slipped.is_open(madrid((2024, 7, 1), (20, 0, 1))));
}

/// The current grid, the order-entry auction and the calendar surface agree
/// with the fixed snapshot, and the 14:00 Christmas-eve early close clips
/// the session end-exclusively.
#[test]
fn the_current_grid_and_the_christmas_eve_early_close_answer() {
    let calendar = calendar_for_market_hours_key(IBEX);
    let noon = madrid((2026, 6, 15), (12, 0, 0));

    assert_eq!(calendar.session_state(noon), Ok(SessionState::OpenRegular));
    assert_eq!(
        calendar.session_state(madrid((2026, 6, 15), (7, 57, 0))),
        Ok(SessionState::OrderEntry),
        "the 07:55-08:00 opening auction is order entry"
    );
    assert_eq!(
        calendar.session_bounds(madrid((2026, 6, 15), (12, 0, 0))),
        Ok(Some((
            madrid((2026, 6, 15), (8, 0, 0)),
            madrid((2026, 6, 15), (22, 0, 0))
        ))),
        "the notice's 8:00-22:00 trading period bounds the session"
    );

    // Christmas Day 2025 is closed; Christmas Eve clips at 14:00.
    assert_eq!(
        calendar.is_closed_trade_date(day((2025, 12, 25)), SessionKind::Both),
        Ok(true)
    );
    let eve = madrid((2025, 12, 24), (12, 0, 0));
    assert_eq!(
        calendar.is_closed_trade_date(day((2025, 12, 24)), SessionKind::Both),
        Ok(false),
        "the eve trades a shortened session"
    );
    assert_eq!(
        calendar.session_bounds(eve),
        Ok(Some((
            madrid((2025, 12, 24), (8, 0, 0)),
            madrid((2025, 12, 24), (14, 0, 0))
        ))),
        "notice 49/2025's Cierre de Negociacion 14:00h clips the session"
    );
    assert!(
        calendar
            .is_open(madrid((2025, 12, 24), (13, 59, 59)))
            .expect("covered date")
    );
    assert!(
        !calendar
            .is_open(madrid((2025, 12, 24), (14, 0, 0)))
            .expect("covered date")
    );
    assert_eq!(
        calendar
            .holiday_on(day((2025, 12, 24)))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 14 * 3600
        }),
    );
}

#[test]
fn the_2027_meff_calendar_sets_ibex_closures_and_eve_hours() {
    let calendar = calendar_for_market_hours_key(IBEX);
    for trade_date in [(2027, 1, 1), (2027, 3, 26), (2027, 3, 29)] {
        assert_eq!(
            calendar.holiday_on(day(trade_date)).unwrap().kind(),
            HolidayKind::Closed
        );
        assert_eq!(
            calendar.is_closed_trade_date(day(trade_date), SessionKind::Both),
            Ok(true)
        );
    }
    for trade_date in [(2027, 1, 6), (2027, 5, 6), (2027, 6, 25), (2027, 12, 6)] {
        assert_eq!(calendar.is_open(madrid(trade_date, (12, 0, 0))), Ok(true));
    }
    for trade_date in [(2027, 12, 24), (2027, 12, 31)] {
        assert_eq!(
            calendar.session_bounds(madrid(trade_date, (12, 0, 0))),
            Ok(Some((
                madrid(trade_date, (8, 0, 0)),
                madrid(trade_date, (14, 0, 0))
            )))
        );
        assert_eq!(calendar.is_open(madrid(trade_date, (13, 59, 59))), Ok(true));
        assert_eq!(calendar.is_open(madrid(trade_date, (14, 0, 0))), Ok(false));
    }
}

/// The coverage contract: refusals below the floor and above the audited
/// windows, and the bridged 2012-2013 span whose sessions answer while the
/// holiday classification refuses typed.
#[test]
fn the_partial_history_refuses_typed_and_bridges_the_2012_2013_span() {
    let calendar = calendar_for_market_hours_key(IBEX);
    let coverage = calendar
        .holiday_coverage()
        .expect("bme_ibex ships a built-in table");
    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2027, 12, 31)));

    // Below the support floor.
    assert!(matches!(
        calendar.is_open(madrid((2009, 12, 31), (12, 0, 0))).map_err(|error| error.kind()),
        Err(CalendarQueryErrorKind::BeforeSupportFloor { date, .. }) if date == day((2009, 12, 31))
    ));

    // The bridged 2012-2013 span (two flanks): sessions answer from the
    // sourced normal week and the classification keeps answering beside the
    // disclosed residual, which the coverage metadata reports as
    // `HolidayWindowsBridged`; only a one-flank date refuses.
    let bridged = madrid((2012, 6, 13), (10, 0, 0));
    assert!(
        calendar
            .is_open(bridged)
            .expect("a bridged span answers sessions")
    );
    assert_eq!(
        calendar.is_closed_trade_date(day((2012, 6, 13)), SessionKind::Both),
        Ok(false),
        "a two-flank bridged date answers its classification beside the residual"
    );

    // Above the last audited window nothing is asserted.
    let above = madrid((2028, 1, 3), (10, 0, 0));
    assert!(matches!(
        calendar.is_open(above).map_err(|error| error.kind()),
        Err(CalendarQueryErrorKind::OutsideCoveredRange { date, .. }) if date == day((2028, 1, 3))
    ));
    assert!(
        calendar
            .without_holidays()
            .is_open(above)
            .expect("a detached snapshot claims no coverage"),
        "the detached normal week still answers"
    );
}
