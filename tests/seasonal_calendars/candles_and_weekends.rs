// SPDX-License-Identifier: MIT-0

//! Seasonal candle, close, and weekend semantics.

use super::prelude::*;

#[test]
fn seasonal_daily_candles_use_the_profile_for_each_trading_day() {
    let b3 = calendar_for_exchange(Exchange::B3);
    let sao_paulo = America::Sao_Paulo;
    let b3_short = local(sao_paulo, (2026, 8, 19), (10, 30, 0));
    let b3_long = local(sao_paulo, (2026, 1, 14), (10, 30, 0));

    // B3 is served and both probe days sit inside its audited window, so the
    // date-aware candles answer: the short day's regular session ends 16:55,
    // the long day's at 17:55, and the final daily close is the venue's own.
    assert_eq!(
        b3.candle_end_with(b3_short, CalendarResolution::Daily, SessionKind::Regular)
            .expect("an audited date answers"),
        Some(local(sao_paulo, (2026, 8, 19), (16, 55, 0))),
        "the short day's regular candle ends at 16:55"
    );
    assert_eq!(
        b3.candle_end_with(b3_long, CalendarResolution::Daily, SessionKind::Regular)
            .expect("an audited date answers"),
        Some(local(sao_paulo, (2026, 1, 14), (17, 55, 0))),
        "the long day's regular candle ends at 17:55"
    );
    assert_eq!(
        b3.time_end_of_day(b3_short)
            .expect("an audited date answers"),
        Some(local(sao_paulo, (2026, 8, 19), (18, 0, 0))),
        "the short day's final close is the after-market envelope end"
    );
    assert_eq!(
        b3.time_end_of_day(b3_long)
            .expect("an audited date answers"),
        Some(local(sao_paulo, (2026, 1, 14), (18, 0, 0))),
        "the long day's final close is the closing-call end"
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
    // B3 is served and 2026-08-21/22 sit inside its audited window, so the
    // weekend boundary answers: shut at the Friday after-market close, shut
    // all Saturday, and end-exclusive at the close.
    assert!(
        !b3.is_open(local(sao_paulo, (2026, 8, 21), (18, 0, 0)))
            .expect("an audited date answers"),
        "the Friday after-market close is end-exclusive"
    );
    assert!(
        !b3.is_open(local(sao_paulo, (2026, 8, 22), (12, 0, 0)))
            .expect("an audited date answers"),
        "the Saturday is closed"
    );
    assert!(
        b3.is_closed_all_day_on(day((2026, 8, 22)), SessionKind::Both)
            .expect("an audited date answers"),
        "the Saturday has no session"
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
