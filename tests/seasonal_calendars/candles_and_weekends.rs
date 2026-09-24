// SPDX-License-Identifier: MIT-0

//! Seasonal candle, close, and weekend semantics.

use super::prelude::*;

#[test]
fn seasonal_daily_candles_use_the_profile_for_each_trading_day() {
    let b3 = calendar_for_exchange(Exchange::B3);
    let sao_paulo = America::Sao_Paulo;
    let b3_short = local(sao_paulo, (2026, 8, 19), (10, 30, 0));
    let b3_long = local(sao_paulo, (2026, 1, 14), (10, 30, 0));

    assert!(
        b3.candle_end_with(b3_short, CalendarResolution::Daily, SessionKind::Regular)
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        b3.candle_end_with(b3_long, CalendarResolution::Daily, SessionKind::Regular)
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        b3.time_end_of_day(b3_short).is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        b3.time_end_of_day(b3_long).is_err(),
        "a dormant identity refuses this probe"
    );

    let bmv = calendar_for_exchange(Exchange::Bmv);
    let mexico = America::Mexico_City;
    let normal = local(mexico, (2024, 3, 8), (10, 0, 0));
    let early = local(mexico, (2024, 3, 11), (10, 0, 0));
    assert!(
        bmv.candle_end(normal, CalendarResolution::Daily).is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        bmv.candle_end(early, CalendarResolution::Daily).is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        bmv.is_open_extended(local(mexico, (2024, 3, 8), (15, 19, 59)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        bmv.is_open_extended(local(mexico, (2024, 3, 11), (14, 19, 59)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        bmv.is_open(local(mexico, (2024, 3, 8), (15, 20, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        bmv.is_open(local(mexico, (2024, 3, 11), (14, 20, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
}

#[test]
fn seasonal_calendars_keep_weekends_and_closes_end_exclusive() {
    let b3 = calendar_for_exchange(Exchange::B3);
    let sao_paulo = America::Sao_Paulo;
    assert!(
        b3.is_open(local(sao_paulo, (2026, 8, 21), (18, 0, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        b3.is_open(local(sao_paulo, (2026, 8, 22), (12, 0, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        b3.is_closed_all_day_on(day((2026, 8, 22)), SessionKind::Both)
            .is_err(),
        "a dormant identity refuses this probe"
    );

    let bmv = calendar_for_exchange(Exchange::Bmv);
    let mexico = America::Mexico_City;
    assert!(
        bmv.is_open(local(mexico, (2026, 8, 21), (14, 20, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        bmv.is_open(local(mexico, (2026, 8, 22), (10, 0, 0)))
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        bmv.is_closed_all_day_on(day((2026, 8, 22)), SessionKind::Both)
            .is_err(),
        "a dormant identity refuses this probe"
    );
}

#[test]
fn launch_day_candle_starts_do_not_require_a_prelaunch_close() {
    let berlin = Europe::Berlin;
    let eex = calendar_for_exchange(Exchange::Eex);
    let eex_instant = local(berlin, (2024, 3, 25), (10, 0, 0));
    assert!(
        eex.candle_start(eex_instant, CalendarResolution::Daily)
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        eex.candle_end(eex_instant, CalendarResolution::Daily)
            .is_err(),
        "a dormant identity refuses this probe"
    );

    let singapore = Asia::Singapore;
    let sgx = calendar_for_exchange(Exchange::Sgx);
    let sgx_instant = local(singapore, (2024, 7, 29), (12, 0, 0));
    assert!(
        sgx.candle_start(sgx_instant, CalendarResolution::Daily)
            .is_err(),
        "a dormant identity refuses this probe"
    );
    assert!(
        sgx.candle_end(sgx_instant, CalendarResolution::Daily)
            .is_err(),
        "a dormant identity refuses this probe"
    );
}
