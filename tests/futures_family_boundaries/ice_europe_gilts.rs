// SPDX-License-Identifier: MIT-0

//! The `ice_europe_gilts` family: the shared 08:00-18:00 grid with its 06:03
//! pre-open, the 2026-07-06 Long/Medium split keyed at London local midnight,
//! the 12:15 and 13:00 half-day closes, the announced-but-untimed eves that
//! ship `Unsourced`, and the typed refusals the windowed history owes.

use chrono::{Datelike as _, TimeZone as _};
use chrono_tz::Europe;
use exchange_hours::{
    CalendarQueryError, HolidayKind, MarketHoursKey, SessionKind, SessionState,
    calendar_for_market_hours_key, hours_for_market_hours_key,
};

const GILTS: MarketHoursKey = MarketHoursKey::IceEuropeGilts;

fn london(date: (i32, u32, u32), time: (u32, u32, u32)) -> chrono::DateTime<chrono::Utc> {
    Europe::London
        .with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2)
        .single()
        .expect("fixture must be an unambiguous London instant")
        .with_timezone(&chrono::Utc)
}

fn day(date: (i32, u32, u32)) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(date.0, date.1, date.2).expect("fixture must be a valid date")
}

fn regular_bounds(instant: chrono::DateTime<chrono::Utc>) -> (u32, u32) {
    let hours = hours_for_market_hours_key(GILTS, instant);
    let rule = hours.regular.first().expect("one regular rule per era");
    (rule.open_ssm, rule.close_ssm)
}

/// The pre-change grid, exactly as the operator's product pages print it:
/// trading 08:00-18:00 with the 06:03 pre-open as order entry.
#[test]
fn the_shared_grid_serves_the_product_page_print() {
    let midweek = london((2026, 5, 11), (12, 0, 0));
    let hours = hours_for_market_hours_key(GILTS, midweek);

    assert!(!hours.is_accepting_orders(london((2026, 5, 11), (6, 2, 59))));
    assert!(
        hours.is_order_entry_only(london((2026, 5, 11), (6, 3, 0))),
        "the page's 06:03 pre-open accepts orders and matches nothing"
    );
    assert!(!hours.is_open(london((2026, 5, 11), (7, 59, 59))));
    assert!(hours.is_open(london((2026, 5, 11), (8, 0, 0))));
    assert_eq!(
        hours.session_state(london((2026, 5, 11), (12, 0, 0))),
        SessionState::OpenRegular
    );
    assert!(
        hours.is_open(london((2026, 5, 11), (17, 59, 59)))
            && !hours.is_open(london((2026, 5, 11), (18, 0, 0))),
        "the 18:00 close is end-exclusive"
    );
    assert!(
        !hours.is_open(london((2026, 5, 9), (12, 0, 0))),
        "the family has a true weekend close"
    );
}

/// Circular 26/063's unconditional Monday 2026-07-06 flips at London local
/// midnight: the last instant of 2026-07-05 still serves 08:00-18:00 and the
/// cutover day serves 01:00-21:00 with the circular's 00:45 pre-open.
#[test]
fn the_2026_split_flips_at_london_local_midnight() {
    let last_shared = Europe::London
        .with_ymd_and_hms(2026, 7, 5, 23, 59, 59)
        .single()
        .expect("an unambiguous London instant")
        .with_timezone(&chrono::Utc);
    let first_split = Europe::London
        .with_ymd_and_hms(2026, 7, 6, 0, 0, 0)
        .single()
        .expect("an unambiguous London instant")
        .with_timezone(&chrono::Utc);

    assert_eq!(regular_bounds(last_shared), (8 * 3600, 18 * 3600));
    assert_eq!(regular_bounds(first_split), (3600, 21 * 3600));

    // On the cutover day itself: nothing accepts before the circular's 00:45,
    // the pre-open runs to the 01:00 open, and the session closes end-exclusively.
    let split_day = hours_for_market_hours_key(GILTS, london((2026, 7, 6), (12, 0, 0)));
    assert!(!split_day.is_accepting_orders(london((2026, 7, 6), (0, 44, 59))));
    assert!(split_day.is_order_entry_only(london((2026, 7, 6), (0, 45, 0))));
    assert!(!split_day.is_open(london((2026, 7, 6), (0, 59, 59))));
    assert!(split_day.is_open(london((2026, 7, 6), (1, 0, 0))));
    assert!(split_day.is_open(london((2026, 7, 6), (20, 59, 59))));
    assert!(!split_day.is_open(london((2026, 7, 6), (21, 0, 0))));
}

/// The holiday layer: bank-holiday closures, the audited-normal 1 May and
/// asterisk-free December eves, the 12:15 and 13:00 half days, and the
/// announced or unwitnessed dates that ship `Unsourced`.
#[test]
fn the_holiday_rows_answer_closures_half_days_and_unsourced_eves() {
    let calendar = calendar_for_market_hours_key(GILTS);
    let coverage = calendar
        .holiday_coverage()
        .expect("ice_europe_gilts ships a built-in table");
    assert_eq!(coverage.first(), day((2010, 1, 1)));
    assert_eq!(coverage.last(), day((2026, 12, 31)));
    assert_eq!(coverage.windows().len(), 6);

    // The 2010-2014, 2015-2017, 2019-2020 and 2021 rows sit below the carried
    // horizon, so the date-aware surface refuses them while the built-in table
    // still answers through `holiday_on` — the sourced rows are shipped, never
    // inferred. LON3203's sterling closures key 2010 from the floor itself.
    for date in [
        day((2010, 1, 1)),
        day((2010, 4, 2)),
        day((2010, 5, 3)),
        day((2011, 4, 22)),
        day((2012, 6, 5)),
        day((2013, 5, 27)),
        day((2015, 5, 4)),
        day((2016, 12, 27)),
        day((2017, 1, 2)),
        day((2019, 1, 1)),
        day((2019, 4, 19)),
        day((2020, 1, 1)),
        day((2020, 4, 10)),
        day((2021, 12, 28)),
    ] {
        assert_eq!(
            calendar.holiday_on(date).map(exchange_hours::Holiday::kind),
            Some(HolidayKind::Closed),
            "{date}"
        );
        assert!(
            matches!(
                calendar.is_closed_trade_date(date, SessionKind::Both),
                Err(CalendarQueryError::OutsideCoveredRange { .. })
            ),
            "{date} is below the carried horizon and refuses typed"
        );
    }

    // The 2013-12-24 half day closes 12:15 GMT; 2022-12-23 printed 13:00.
    assert_eq!(
        calendar
            .holiday_on(day((2013, 12, 24)))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 12 * 3600 + 15 * 60
        }),
        "the LIFFE attachment's gilt row closes 12:15"
    );
    assert_eq!(
        calendar
            .holiday_on(day((2022, 12, 23)))
            .map(exchange_hours::Holiday::kind),
        Some(HolidayKind::EarlyClose {
            close_ssm: 13 * 3600
        }),
        "the 2022 Christmas document prints a 13:00 cessation"
    );

    // Above the horizon the date-aware surface answers the sourced rows.
    for date in [day((2026, 5, 4)), day((2026, 12, 28))] {
        assert_eq!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            Ok(true),
            "{date}"
        );
    }

    // The editions' open 1 May: audited normal, not a closure — 2015 as well
    // as 2026. The 2015 edition's 1 May note names only the Euribor,
    // Euroswiss and EUR-Swapnote trio (its subject per the 2017 edition), so
    // the gilt date has no row and the window audits it normal.
    for date in [day((2015, 5, 1)), day((2026, 5, 1))] {
        assert!(
            coverage.contains(date),
            "{date} is inside an audited window"
        );
        assert_eq!(
            calendar.holiday_on(date).map(exchange_hours::Holiday::kind),
            None,
            "{date} is audited normal"
        );
    }
    assert_eq!(
        calendar.is_closed_trade_date(day((2026, 5, 1)), SessionKind::Both),
        Ok(false),
        "the August 2026 edition prints the gilt row Open on 1 May"
    );

    // The 2016/2017 December eves carry no asterisk on the gilt rows, so the
    // editions' 18:00 early close never touches the family: audited normal.
    for date in [day((2016, 12, 23)), day((2017, 12, 22))] {
        assert!(
            coverage.contains(date),
            "{date} is inside an audited window"
        );
        assert_eq!(
            calendar.holiday_on(date).map(exchange_hours::Holiday::kind),
            None,
            "{date} is audited normal"
        );
    }

    // The 2025 eves carry sourced 12:15 closes; both sit below the carried
    // horizon, so the rows answer through `holiday_on` while the date-aware
    // session layer refuses.
    for date in [day((2025, 12, 24)), day((2025, 12, 31))] {
        assert_eq!(
            calendar.holiday_on(date).map(exchange_hours::Holiday::kind),
            Some(HolidayKind::EarlyClose {
                close_ssm: 12 * 3600 + 15 * 60
            }),
            "{date}"
        );
        assert!(matches!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
    }

    // The announced eves with unpublished instants ship typed withholdings —
    // 2015's asterisked gilt rows join 2014, 2021, 2024 and 2026 — and so do
    // the 2019/2020 dates the exchange-wide editions name without printing a
    // family arrangement (the unwitnessed Migrated-edition layer).
    for date in [
        day((2015, 12, 24)),
        day((2019, 5, 6)),
        day((2019, 12, 24)),
        day((2020, 5, 8)),
        day((2020, 12, 31)),
    ] {
        assert_eq!(
            calendar.holiday_on(date).map(exchange_hours::Holiday::kind),
            Some(HolidayKind::Unsourced),
            "{date}"
        );
    }
    for date in [day((2026, 12, 24)), day((2026, 12, 31))] {
        assert!(
            matches!(
                calendar.is_closed_trade_date(date, SessionKind::Both),
                Err(CalendarQueryError::UnresolvedGap { .. })
            ),
            "{date} refuses its holiday classification"
        );
        assert!(
            matches!(
                calendar.is_open(london((date.year(), date.month(), date.day()), (10, 0, 0))),
                Err(CalendarQueryError::UnresolvedGap { .. })
            ),
            "{date} refuses its sessions too: a withheld date is never audited normal"
        );
    }
}

/// The windowed history: the unwitnessed 2018 year and 2023 span, and every
/// pre-horizon date — including dates the shipped holiday rows and windows
/// now sit under — refuse typed, because the normal week there is carried.
#[test]
fn the_windowed_history_refuses_typed_and_bridges_the_gaps() {
    let calendar = calendar_for_market_hours_key(GILTS);

    // The whole span below the carried horizon refuses typed, whatever the
    // holiday layer ships under it: the normal week there is carried, not
    // sourced, so the charter's one-flank bridge — which reaches only where
    // the sourced week answers — never does. 2010-06-15 sits inside the first
    // audited holiday window (LON3203's full 2010 year), 2018-06-12 inside
    // no window at all, and 2022-12-23 under a shipped early-close row; all
    // three refuse identically. The shipped rows stay readable through
    // `holiday_on` (asserted in the holiday test above).
    for date in [day((2010, 6, 15)), day((2018, 6, 12)), day((2022, 12, 23))] {
        assert!(matches!(
            calendar.is_open(london((date.year(), date.month(), date.day()), (10, 0, 0))),
            Err(CalendarQueryError::OutsideCoveredRange { date: refused, .. }) if refused == date
        ));
        assert!(matches!(
            calendar.is_closed_trade_date(date, SessionKind::Both),
            Err(CalendarQueryError::OutsideCoveredRange { .. })
        ));
    }

    // Below the support floor and above the last window.
    assert!(matches!(
        calendar.is_open(london((2009, 12, 31), (12, 0, 0))),
        Err(CalendarQueryError::BeforeSupportFloor { date, .. }) if date == day((2009, 12, 31))
    ));
    let above = london((2027, 1, 4), (10, 0, 0));
    assert!(matches!(
        calendar.is_open(above),
        Err(CalendarQueryError::OutsideCoveredRange { date, .. }) if date == day((2027, 1, 4))
    ));
    assert!(
        calendar
            .without_holidays()
            .is_open(above)
            .expect("a detached snapshot claims no coverage")
    );
}
